use core::ffi::{c_char, c_void};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Mutex;

use libc::strdup;

use crate::content::{encode_content_json, NotificationContent};
use crate::error::{from_swift, UserNotificationsError};
use crate::ffi;
use crate::private::to_cstring;
use crate::request::{encode_request_json, NotificationRequest, NotificationRequestPayload};

mod private {
    pub trait Sealed {}
}

/// Wraps the `UNNotificationServiceExtension` callback surface.
pub trait NotificationServiceExtensionHandler: Send + private::Sealed {
    /// Handles an incoming request before a service extension delivers modified content.
    fn did_receive_notification_request(
        &mut self,
        request: NotificationRequest,
    ) -> NotificationContent {
        request.content
    }

    /// Handles the service-extension expiration callback.
    fn service_extension_time_will_expire(&mut self) {}
}

type ReceiveHandler = Box<dyn FnMut(NotificationRequest) -> NotificationContent + Send + 'static>;
type ExpireHandler = Box<dyn FnMut() + Send + 'static>;

/// Closure-based builder for a `UNNotificationServiceExtension` handler.
pub struct NotificationServiceExtensionCallbacks {
    receive: Option<ReceiveHandler>,
    expire: Option<ExpireHandler>,
}

impl NotificationServiceExtensionCallbacks {
    /// Creates an empty callback-based service-extension handler.
    #[must_use]
    pub fn new() -> Self {
        Self {
            receive: None,
            expire: None,
        }
    }

    /// Registers a callback for receive notification request.
    #[must_use]
    pub fn on_receive_notification_request(
        mut self,
        callback: impl FnMut(NotificationRequest) -> NotificationContent + Send + 'static,
    ) -> Self {
        self.receive = Some(Box::new(callback));
        self
    }

    /// Registers a callback for time will expire.
    #[must_use]
    pub fn on_time_will_expire(mut self, callback: impl FnMut() + Send + 'static) -> Self {
        self.expire = Some(Box::new(callback));
        self
    }
}

impl Default for NotificationServiceExtensionCallbacks {
    fn default() -> Self {
        Self::new()
    }
}

impl private::Sealed for NotificationServiceExtensionCallbacks {}
impl NotificationServiceExtensionHandler for NotificationServiceExtensionCallbacks {
    fn did_receive_notification_request(
        &mut self,
        request: NotificationRequest,
    ) -> NotificationContent {
        if let Some(callback) = &mut self.receive {
            callback(request)
        } else {
            request.content
        }
    }

    fn service_extension_time_will_expire(&mut self) {
        if let Some(callback) = &mut self.expire {
            callback();
        }
    }
}

struct CallbackState {
    handler: Mutex<Box<dyn NotificationServiceExtensionHandler>>,
}

/// Simulates a `UNNotificationServiceExtension` host environment.
pub struct NotificationServiceExtensionSimulator {
    raw: *mut c_void,
    _callback_state: Box<CallbackState>,
}

extern "C" fn service_receive_trampoline(
    user_info: *mut c_void,
    request_json: *const c_char,
    error_out: *mut *mut c_char,
) -> *mut c_char {
    catch_unwind(AssertUnwindSafe(|| {
        if user_info.is_null() || request_json.is_null() {
            write_error(error_out, "service extension callback received null input");
            return core::ptr::null_mut();
        }

        let state = unsafe { &*(user_info as *const CallbackState) };
        let json = unsafe { std::ffi::CStr::from_ptr(request_json) }.to_string_lossy();
        let request = match serde_json::from_str::<NotificationRequestPayload>(&json) {
            Ok(payload) => NotificationRequest::from(payload),
            Err(error) => {
                write_error(
                    error_out,
                    &format!("failed to decode notification request payload: {error}"),
                );
                return core::ptr::null_mut();
            }
        };

        let Ok(mut handler) = state.handler.lock() else {
            write_error(error_out, "notification service handler lock poisoned");
            return core::ptr::null_mut();
        };
        let content = handler.did_receive_notification_request(request);
        match encode_content_json(&content) {
            Ok(json) => duplicate_string(&json),
            Err(error) => {
                write_error(error_out, &error.to_string());
                core::ptr::null_mut()
            }
        }
    }))
    .unwrap_or_else(|_| {
        write_error(error_out, "notification service handler panicked");
        core::ptr::null_mut()
    })
}

extern "C" fn service_expire_trampoline(user_info: *mut c_void) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if user_info.is_null() {
            return;
        }

        let state = unsafe { &*(user_info as *const CallbackState) };
        if let Ok(mut handler) = state.handler.lock() {
            handler.service_extension_time_will_expire();
        }
    }));
}

impl NotificationServiceExtensionSimulator {
    /// Creates a notification service extension simulator with callbacks.
    pub fn new(
        callbacks: NotificationServiceExtensionCallbacks,
    ) -> Result<Self, UserNotificationsError> {
        Self::with_handler(callbacks)
    }

    /// Creates a notification service extension simulator with a custom handler.
    pub fn with_handler<H>(handler: H) -> Result<Self, UserNotificationsError>
    where
        H: NotificationServiceExtensionHandler + 'static,
    {
        let callback_state = Box::new(CallbackState {
            handler: Mutex::new(Box::new(handler)),
        });
        let user_info = std::ptr::from_ref::<CallbackState>(&*callback_state)
            .cast_mut()
            .cast();
        let mut raw = core::ptr::null_mut();
        let mut error = core::ptr::null_mut();
        let status = unsafe {
            ffi::service_extension::un_service_extension_simulator_create(
                Some(
                    service_receive_trampoline
                        as ffi::service_extension::ServiceExtensionReceiveCallback,
                ),
                Some(
                    service_expire_trampoline
                        as ffi::service_extension::ServiceExtensionExpireCallback,
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

    /// Sends a request to the simulated service extension.
    pub fn receive_notification_request(
        &self,
        request: &NotificationRequest,
    ) -> Result<NotificationContent, UserNotificationsError> {
        let request = encode_request_json(request)?;
        let request = to_cstring(&request)?;
        let mut error = core::ptr::null_mut();
        let payload = unsafe {
            ffi::service_extension::un_service_extension_simulator_receive_request_json(
                self.raw,
                request.as_ptr(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            crate::content::decode_content_json(payload)
        }
    }

    /// Invokes the simulated expiration callback.
    pub fn service_extension_time_will_expire(&self) {
        unsafe {
            ffi::service_extension::un_service_extension_simulator_time_will_expire(self.raw);
        };
    }
}

impl Drop for NotificationServiceExtensionSimulator {
    fn drop(&mut self) {
        unsafe { ffi::core::un_object_release(self.raw) };
    }
}

fn duplicate_string(value: &str) -> *mut c_char {
    let c_string = std::ffi::CString::new(value).expect("encoded JSON must not contain NUL bytes");
    unsafe { strdup(c_string.as_ptr()) }
}

fn write_error(error_out: *mut *mut c_char, message: &str) {
    if error_out.is_null() {
        return;
    }
    unsafe {
        *error_out = duplicate_string(message);
    }
}
