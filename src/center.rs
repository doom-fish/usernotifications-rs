use core::ffi::{c_char, c_void};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Mutex;

use serde::Deserialize;

use crate::error::{from_swift, UserNotificationsError};
use crate::ffi;
use crate::notification::{
    decode_categories_json, decode_notifications_json, decode_requests_json, decode_settings_json,
    encode_categories_json, encode_request_json, AuthorizationOptions, Notification,
    NotificationCategory, NotificationPayload, NotificationRequest, NotificationResponse,
    NotificationResponsePayload, NotificationSettings,
};
use crate::private::to_cstring;

#[derive(Deserialize)]
struct CenterEventPayload {
    event: String,
    notification: Option<NotificationPayload>,
    response: Option<NotificationResponsePayload>,
}

mod private {
    pub trait Sealed {}
}

pub trait UserNotificationCenterDelegate: Send + private::Sealed {
    fn did_receive_notification_response(&mut self, response: NotificationResponse) {
        let _ = response;
    }

    fn open_settings_for_notification(&mut self, notification: Option<Notification>) {
        let _ = notification;
    }
}

type ResponseHandler = Box<dyn FnMut(NotificationResponse) + Send + 'static>;
type OpenSettingsHandler = Box<dyn FnMut(Option<Notification>) + Send + 'static>;

#[allow(clippy::type_complexity)]
pub struct UserNotificationCenterCallbacks {
    response: Option<ResponseHandler>,
    open_settings: Option<OpenSettingsHandler>,
}

impl UserNotificationCenterCallbacks {
    #[must_use]
    pub fn new() -> Self {
        Self {
            response: None,
            open_settings: None,
        }
    }

    #[must_use]
    pub fn on_response(
        mut self,
        callback: impl FnMut(NotificationResponse) + Send + 'static,
    ) -> Self {
        self.response = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_open_settings(
        mut self,
        callback: impl FnMut(Option<Notification>) + Send + 'static,
    ) -> Self {
        self.open_settings = Some(Box::new(callback));
        self
    }
}

impl Default for UserNotificationCenterCallbacks {
    fn default() -> Self {
        Self::new()
    }
}

impl private::Sealed for UserNotificationCenterCallbacks {}
impl UserNotificationCenterDelegate for UserNotificationCenterCallbacks {
    fn did_receive_notification_response(&mut self, response: NotificationResponse) {
        if let Some(callback) = &mut self.response {
            callback(response);
        }
    }

    fn open_settings_for_notification(&mut self, notification: Option<Notification>) {
        if let Some(callback) = &mut self.open_settings {
            callback(notification);
        }
    }
}

struct CallbackState {
    delegate: Mutex<Box<dyn UserNotificationCenterDelegate>>,
}

pub struct UserNotificationCenter {
    raw: *mut c_void,
    callback_state: Option<Box<CallbackState>>,
}

unsafe impl Send for UserNotificationCenter {}
unsafe impl Sync for UserNotificationCenter {}

extern "C" fn center_event_trampoline(user_info: *mut c_void, event_json: *const c_char) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if user_info.is_null() || event_json.is_null() {
            return;
        }

        let state = unsafe { &*(user_info as *const CallbackState) };
        let json = unsafe { std::ffi::CStr::from_ptr(event_json) }.to_string_lossy();
        let Ok(payload) = serde_json::from_str::<CenterEventPayload>(&json) else {
            return;
        };

        let Ok(mut delegate) = state.delegate.lock() else {
            return;
        };

        match payload.event.as_str() {
            "didReceiveNotificationResponse" => {
                if let Some(response) = payload.response {
                    delegate.did_receive_notification_response(response.into());
                }
            }
            "openSettingsForNotification" => {
                delegate.open_settings_for_notification(payload.notification.map(Into::into));
            }
            _ => {}
        }
    }));
}

impl UserNotificationCenter {
    pub fn current() -> Result<Self, UserNotificationsError> {
        Self::current_inner(None)
    }

    pub fn with_delegate<D>(delegate: D) -> Result<Self, UserNotificationsError>
    where
        D: UserNotificationCenterDelegate + 'static,
    {
        Self::current_inner(Some(Box::new(delegate)))
    }

    pub fn with_callbacks(
        callbacks: UserNotificationCenterCallbacks,
    ) -> Result<Self, UserNotificationsError> {
        Self::with_delegate(callbacks)
    }

    fn current_inner(
        delegate: Option<Box<dyn UserNotificationCenterDelegate>>,
    ) -> Result<Self, UserNotificationsError> {
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe { ffi::un_center_current(&mut raw, &mut error) };
        if status != ffi::status::OK {
            return Err(from_swift(status, error));
        }

        let mut center = Self {
            raw,
            callback_state: None,
        };
        if let Some(delegate) = delegate {
            center.set_boxed_delegate(delegate)?;
        }
        Ok(center)
    }

    pub fn set_delegate<D>(&mut self, delegate: D) -> Result<(), UserNotificationsError>
    where
        D: UserNotificationCenterDelegate + 'static,
    {
        self.set_boxed_delegate(Box::new(delegate))
    }

    pub fn set_callbacks(
        &mut self,
        callbacks: UserNotificationCenterCallbacks,
    ) -> Result<(), UserNotificationsError> {
        self.set_delegate(callbacks)
    }

    fn set_boxed_delegate(
        &mut self,
        delegate: Box<dyn UserNotificationCenterDelegate>,
    ) -> Result<(), UserNotificationsError> {
        let callback_state = Box::new(CallbackState {
            delegate: Mutex::new(delegate),
        });
        let mut callback_state = Some(callback_state);
        let user_info = callback_state
            .as_deref_mut()
            .map_or(core::ptr::null_mut(), |state| {
                std::ptr::from_mut::<CallbackState>(state).cast::<c_void>()
            });
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::un_center_set_delegate(
                self.raw,
                Some(center_event_trampoline as ffi::CenterEventCallback),
                user_info,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            self.callback_state = callback_state;
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn clear_delegate(&mut self) {
        unsafe { ffi::un_center_clear_delegate(self.raw) };
        self.callback_state = None;
    }

    pub fn request_authorization(
        &self,
        options: AuthorizationOptions,
    ) -> Result<bool, UserNotificationsError> {
        let mut granted = false;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::un_center_request_authorization(self.raw, options.bits(), &mut granted, &mut error)
        };
        if status == ffi::status::OK {
            Ok(granted)
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn notification_settings(&self) -> Result<NotificationSettings, UserNotificationsError> {
        let mut error = core::ptr::null_mut();
        let payload =
            unsafe { ffi::un_center_get_notification_settings_json(self.raw, &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_settings_json(payload)
        }
    }

    pub fn set_notification_categories(
        &self,
        categories: &[NotificationCategory],
    ) -> Result<(), UserNotificationsError> {
        let categories = encode_categories_json(categories)?;
        let categories = to_cstring(&categories)?;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::un_center_set_notification_categories(self.raw, categories.as_ptr(), &mut error)
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn notification_categories(
        &self,
    ) -> Result<Vec<NotificationCategory>, UserNotificationsError> {
        let mut error = core::ptr::null_mut();
        let payload =
            unsafe { ffi::un_center_get_notification_categories_json(self.raw, &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_categories_json(payload)
        }
    }

    pub fn add_notification_request(
        &self,
        request: &NotificationRequest,
    ) -> Result<(), UserNotificationsError> {
        let request = encode_request_json(request)?;
        let request = to_cstring(&request)?;
        let mut error = core::ptr::null_mut();
        let status = unsafe { ffi::un_center_add_request(self.raw, request.as_ptr(), &mut error) };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn pending_notification_requests(
        &self,
    ) -> Result<Vec<NotificationRequest>, UserNotificationsError> {
        let mut error = core::ptr::null_mut();
        let payload = unsafe { ffi::un_center_get_pending_requests_json(self.raw, &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_requests_json(payload)
        }
    }

    pub fn remove_pending_notification_requests(
        &self,
        identifiers: &[&str],
    ) -> Result<(), UserNotificationsError> {
        let identifiers = encode_identifiers(identifiers)?;
        unsafe { ffi::un_center_remove_pending_requests(self.raw, identifiers.as_ptr()) };
        Ok(())
    }

    pub fn remove_all_pending_notification_requests(&self) {
        unsafe { ffi::un_center_remove_all_pending_requests(self.raw) };
    }

    pub fn delivered_notifications(&self) -> Result<Vec<Notification>, UserNotificationsError> {
        let mut error = core::ptr::null_mut();
        let payload =
            unsafe { ffi::un_center_get_delivered_notifications_json(self.raw, &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_notifications_json(payload)
        }
    }

    pub fn remove_delivered_notifications(
        &self,
        identifiers: &[&str],
    ) -> Result<(), UserNotificationsError> {
        let identifiers = encode_identifiers(identifiers)?;
        unsafe { ffi::un_center_remove_delivered_notifications(self.raw, identifiers.as_ptr()) };
        Ok(())
    }

    pub fn remove_all_delivered_notifications(&self) {
        unsafe { ffi::un_center_remove_all_delivered_notifications(self.raw) };
    }
}

impl Drop for UserNotificationCenter {
    fn drop(&mut self) {
        if self.callback_state.is_some() {
            unsafe { ffi::un_center_clear_delegate(self.raw) };
        }
        unsafe { ffi::un_object_release(self.raw) };
    }
}

fn encode_identifiers(identifiers: &[&str]) -> Result<std::ffi::CString, UserNotificationsError> {
    let json = serde_json::to_string(identifiers).map_err(|error| {
        UserNotificationsError::FrameworkError(format!("failed to encode identifiers: {error}"))
    })?;
    to_cstring(&json)
}
