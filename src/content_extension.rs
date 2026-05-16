use core::ffi::{c_char, c_void};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Mutex;

use serde_json::to_string;

use crate::action::{NotificationAction, NotificationActionPayload};
use crate::error::{from_swift, UserNotificationsError};
use crate::ffi;
use crate::notification::{encode_notification_json, Notification, NotificationPayload};
use crate::private::{decode_json, to_cstring};
use crate::response::{
    encode_response_json, NotificationResponse, NotificationResponsePayload,
};

mod private {
    pub trait Sealed {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NotificationContentExtensionMediaPlayPauseButtonType {
    None = 0,
    Default = 1,
    Overlay = 2,
}

impl NotificationContentExtensionMediaPlayPauseButtonType {
    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        match raw {
            1 => Self::Default,
            2 => Self::Overlay,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NotificationContentExtensionResponseOption {
    DoNotDismiss = 0,
    Dismiss = 1,
    DismissAndForwardAction = 2,
}

impl NotificationContentExtensionResponseOption {
    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        match raw {
            0 => Self::DoNotDismiss,
            1 => Self::Dismiss,
            _ => Self::DismissAndForwardAction,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NotificationContentExtensionRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl NotificationContentExtensionRect {
    #[must_use]
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NotificationContentExtensionTintColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl NotificationContentExtensionTintColor {
    #[must_use]
    pub const fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

pub trait NotificationContentExtensionHandler: Send + private::Sealed {
    fn did_receive_notification(&mut self, notification: Notification) {
        let _ = notification;
    }

    fn did_receive_notification_response(
        &mut self,
        response: NotificationResponse,
    ) -> NotificationContentExtensionResponseOption {
        let _ = response;
        NotificationContentExtensionResponseOption::DismissAndForwardAction
    }

    fn media_play(&mut self) {}

    fn media_pause(&mut self) {}
}

type NotificationHandler = Box<dyn FnMut(Notification) + Send + 'static>;
type ResponseHandler =
    Box<dyn FnMut(NotificationResponse) -> NotificationContentExtensionResponseOption + Send + 'static>;
type SimpleHandler = Box<dyn FnMut() + Send + 'static>;

pub struct NotificationContentExtensionCallbacks {
    notification: Option<NotificationHandler>,
    response: Option<ResponseHandler>,
    media_play: Option<SimpleHandler>,
    media_pause: Option<SimpleHandler>,
}

impl NotificationContentExtensionCallbacks {
    #[must_use]
    pub fn new() -> Self {
        Self {
            notification: None,
            response: None,
            media_play: None,
            media_pause: None,
        }
    }

    #[must_use]
    pub fn on_notification(
        mut self,
        callback: impl FnMut(Notification) + Send + 'static,
    ) -> Self {
        self.notification = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_response(
        mut self,
        callback: impl FnMut(NotificationResponse) -> NotificationContentExtensionResponseOption
            + Send
            + 'static,
    ) -> Self {
        self.response = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_media_play(mut self, callback: impl FnMut() + Send + 'static) -> Self {
        self.media_play = Some(Box::new(callback));
        self
    }

    #[must_use]
    pub fn on_media_pause(mut self, callback: impl FnMut() + Send + 'static) -> Self {
        self.media_pause = Some(Box::new(callback));
        self
    }
}

impl Default for NotificationContentExtensionCallbacks {
    fn default() -> Self {
        Self::new()
    }
}

impl private::Sealed for NotificationContentExtensionCallbacks {}
impl NotificationContentExtensionHandler for NotificationContentExtensionCallbacks {
    fn did_receive_notification(&mut self, notification: Notification) {
        if let Some(callback) = &mut self.notification {
            callback(notification);
        }
    }

    fn did_receive_notification_response(
        &mut self,
        response: NotificationResponse,
    ) -> NotificationContentExtensionResponseOption {
        self.response.as_mut().map_or(
            NotificationContentExtensionResponseOption::DismissAndForwardAction,
            |callback| callback(response),
        )
    }

    fn media_play(&mut self) {
        if let Some(callback) = &mut self.media_play {
            callback();
        }
    }

    fn media_pause(&mut self) {
        if let Some(callback) = &mut self.media_pause {
            callback();
        }
    }
}

struct CallbackState {
    handler: Mutex<Box<dyn NotificationContentExtensionHandler>>,
}

pub struct NotificationContentExtensionContext {
    raw: *mut c_void,
}

pub struct NotificationContentExtensionSimulator {
    raw: *mut c_void,
    _callback_state: Box<CallbackState>,
}

extern "C" fn content_notification_trampoline(user_info: *mut c_void, notification_json: *const c_char) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if user_info.is_null() || notification_json.is_null() {
            return;
        }

        let state = unsafe { &*(user_info as *const CallbackState) };
        let json = unsafe { std::ffi::CStr::from_ptr(notification_json) }.to_string_lossy();
        let Ok(payload) = serde_json::from_str::<NotificationPayload>(&json) else {
            return;
        };

        if let Ok(mut handler) = state.handler.lock() {
            handler.did_receive_notification(payload.into());
        }
    }));
}

extern "C" fn content_response_trampoline(
    user_info: *mut c_void,
    response_json: *const c_char,
) -> u64 {
    catch_unwind(AssertUnwindSafe(|| {
        if user_info.is_null() || response_json.is_null() {
            return NotificationContentExtensionResponseOption::DismissAndForwardAction as u64;
        }

        let state = unsafe { &*(user_info as *const CallbackState) };
        let json = unsafe { std::ffi::CStr::from_ptr(response_json) }.to_string_lossy();
        let Ok(payload) = serde_json::from_str::<NotificationResponsePayload>(&json) else {
            return NotificationContentExtensionResponseOption::DismissAndForwardAction as u64;
        };

        let Ok(mut handler) = state.handler.lock() else {
            return NotificationContentExtensionResponseOption::DismissAndForwardAction as u64;
        };

        handler.did_receive_notification_response(payload.into()) as u64
    }))
    .unwrap_or(NotificationContentExtensionResponseOption::DismissAndForwardAction as u64)
}

extern "C" fn content_media_play_trampoline(user_info: *mut c_void) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if user_info.is_null() {
            return;
        }

        let state = unsafe { &*(user_info as *const CallbackState) };
        if let Ok(mut handler) = state.handler.lock() {
            handler.media_play();
        }
    }));
}

extern "C" fn content_media_pause_trampoline(user_info: *mut c_void) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if user_info.is_null() {
            return;
        }

        let state = unsafe { &*(user_info as *const CallbackState) };
        if let Ok(mut handler) = state.handler.lock() {
            handler.media_pause();
        }
    }));
}

impl NotificationContentExtensionContext {
    pub fn new() -> Result<Self, UserNotificationsError> {
        let mut raw = core::ptr::null_mut();
        let status = unsafe { ffi::content_extension::un_content_extension_context_create(&mut raw) };
        if status == ffi::status::OK {
            Ok(Self { raw })
        } else {
            Err(UserNotificationsError::FrameworkError(
                "failed to create notification content extension context".into(),
            ))
        }
    }

    pub fn set_notification_actions(
        &self,
        actions: &[NotificationAction],
    ) -> Result<(), UserNotificationsError> {
        let payloads = actions
            .iter()
            .map(NotificationActionPayload::from)
            .collect::<Vec<_>>();
        let json = to_string(&payloads).map_err(|error| {
            UserNotificationsError::FrameworkError(format!(
                "failed to encode notification actions: {error}",
            ))
        })?;
        let json = to_cstring(&json)?;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::content_extension::un_content_extension_context_set_notification_actions(
                self.raw,
                json.as_ptr(),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn notification_actions(&self) -> Result<Vec<NotificationAction>, UserNotificationsError> {
        let mut error = core::ptr::null_mut();
        let payload = unsafe {
            ffi::content_extension::un_content_extension_context_get_notification_actions_json(
                self.raw,
                &mut error,
            )
        };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_json::<Vec<NotificationActionPayload>>(payload)
                .map(|actions| actions.into_iter().map(Into::into).collect())
        }
    }

    pub fn perform_default_action(&self) {
        unsafe { ffi::content_extension::un_content_extension_context_perform_default_action(self.raw) };
    }

    pub fn dismiss_notification_content_extension(&self) {
        unsafe { ffi::content_extension::un_content_extension_context_dismiss(self.raw) };
    }

    pub fn media_playing_started(&self) {
        unsafe { ffi::content_extension::un_content_extension_context_media_playing_started(self.raw) };
    }

    pub fn media_playing_paused(&self) {
        unsafe { ffi::content_extension::un_content_extension_context_media_playing_paused(self.raw) };
    }
}

impl Drop for NotificationContentExtensionContext {
    fn drop(&mut self) {
        unsafe { ffi::core::un_object_release(self.raw) };
    }
}

impl NotificationContentExtensionSimulator {
    pub fn new(
        callbacks: NotificationContentExtensionCallbacks,
    ) -> Result<Self, UserNotificationsError> {
        Self::with_handler(callbacks)
    }

    pub fn with_handler<H>(handler: H) -> Result<Self, UserNotificationsError>
    where
        H: NotificationContentExtensionHandler + 'static,
    {
        let callback_state = Box::new(CallbackState {
            handler: Mutex::new(Box::new(handler)),
        });
        let user_info = std::ptr::from_ref::<CallbackState>(&*callback_state).cast_mut().cast();
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::content_extension::un_content_extension_simulator_create(
                Some(
                    content_notification_trampoline
                        as ffi::content_extension::ContentExtensionNotificationCallback,
                ),
                Some(
                    content_response_trampoline
                        as ffi::content_extension::ContentExtensionResponseCallback,
                ),
                Some(
                    content_media_play_trampoline
                        as ffi::content_extension::ContentExtensionSimpleCallback,
                ),
                Some(
                    content_media_pause_trampoline
                        as ffi::content_extension::ContentExtensionSimpleCallback,
                ),
                user_info,
                &mut raw,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(Self {
                raw,
                _callback_state: callback_state,
            })
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn set_media_play_pause_button_type(
        &self,
        button_type: NotificationContentExtensionMediaPlayPauseButtonType,
    ) {
        unsafe {
            ffi::content_extension::un_content_extension_simulator_set_media_play_pause_button_type(
                self.raw,
                button_type as u64,
            );
        }
    }

    #[must_use]
    pub fn media_play_pause_button_type(&self) -> NotificationContentExtensionMediaPlayPauseButtonType {
        NotificationContentExtensionMediaPlayPauseButtonType::from_raw(unsafe {
            ffi::content_extension::un_content_extension_simulator_get_media_play_pause_button_type(
                self.raw,
            )
        })
    }

    pub fn set_media_play_pause_button_frame(
        &self,
        frame: NotificationContentExtensionRect,
    ) {
        unsafe {
            ffi::content_extension::un_content_extension_simulator_set_media_play_pause_button_frame(
                self.raw,
                frame.x,
                frame.y,
                frame.width,
                frame.height,
            );
        }
    }

    #[must_use]
    pub fn media_play_pause_button_frame(&self) -> NotificationContentExtensionRect {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe {
            ffi::content_extension::un_content_extension_simulator_get_media_play_pause_button_frame(
                self.raw,
                &mut x,
                &mut y,
                &mut width,
                &mut height,
            );
        }
        NotificationContentExtensionRect::new(x, y, width, height)
    }

    pub fn set_media_play_pause_button_tint_color(
        &self,
        tint_color: NotificationContentExtensionTintColor,
    ) {
        unsafe {
            ffi::content_extension::un_content_extension_simulator_set_media_play_pause_button_tint_color(
                self.raw,
                tint_color.red,
                tint_color.green,
                tint_color.blue,
                tint_color.alpha,
            );
        }
    }

    #[must_use]
    pub fn media_play_pause_button_tint_color(
        &self,
    ) -> Option<NotificationContentExtensionTintColor> {
        let mut red = 0.0;
        let mut green = 0.0;
        let mut blue = 0.0;
        let mut alpha = 0.0;
        let present = unsafe {
            ffi::content_extension::un_content_extension_simulator_get_media_play_pause_button_tint_color(
                self.raw,
                &mut red,
                &mut green,
                &mut blue,
                &mut alpha,
            )
        };
        present.then(|| NotificationContentExtensionTintColor::new(red, green, blue, alpha))
    }

    pub fn receive_notification(&self, notification: &Notification) -> Result<(), UserNotificationsError> {
        let notification = encode_notification_json(notification)?;
        let notification = to_cstring(&notification)?;
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::content_extension::un_content_extension_simulator_receive_notification_json(
                self.raw,
                notification.as_ptr(),
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(())
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn receive_notification_response(
        &self,
        response: &NotificationResponse,
    ) -> Result<NotificationContentExtensionResponseOption, UserNotificationsError> {
        let response = encode_response_json(response)?;
        let response = to_cstring(&response)?;
        let mut error = core::ptr::null_mut();
        let mut option = 0;
        let status = unsafe {
            ffi::content_extension::un_content_extension_simulator_receive_response_json(
                self.raw,
                response.as_ptr(),
                &mut option,
                &mut error,
            )
        };
        if status == ffi::status::OK {
            Ok(NotificationContentExtensionResponseOption::from_raw(option))
        } else {
            Err(from_swift(status, error))
        }
    }

    pub fn media_play(&self) {
        unsafe { ffi::content_extension::un_content_extension_simulator_media_play(self.raw) };
    }

    pub fn media_pause(&self) {
        unsafe { ffi::content_extension::un_content_extension_simulator_media_pause(self.raw) };
    }
}

impl Drop for NotificationContentExtensionSimulator {
    fn drop(&mut self) {
        unsafe { ffi::core::un_object_release(self.raw) };
    }
}
