use std::ffi::{c_char, c_void};

pub mod action;
pub mod attachment;
pub mod category;
pub mod center;
pub mod content;
pub mod content_extension;
pub mod core;
pub mod request;
pub mod response;
pub mod service_extension;
pub mod trigger;

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const FRAMEWORK_ERROR: i32 = -2;
}

unsafe extern "C" {
    pub fn un_center_get_notification_settings_json(
        center: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
}

// Async FFI declarations
#[cfg(feature = "async")]
pub mod r#async {
    use super::{c_char, c_void};

    pub type AsyncCallback = unsafe extern "C" fn(*const c_void, *const c_char, *mut c_void);

    unsafe extern "C" {
        pub fn un_center_request_authorization_async(
            center: *mut c_void,
            options: u64,
            cb: AsyncCallback,
            ctx: *mut c_void,
        );
        pub fn un_center_add_notification_request_async(
            center: *mut c_void,
            request_json: *const c_char,
            cb: AsyncCallback,
            ctx: *mut c_void,
        );
        pub fn un_center_get_delivered_notifications_async(
            center: *mut c_void,
            cb: AsyncCallback,
            ctx: *mut c_void,
        );
        pub fn un_center_get_pending_notification_requests_async(
            center: *mut c_void,
            cb: AsyncCallback,
            ctx: *mut c_void,
        );
        pub fn un_center_get_notification_categories_async(
            center: *mut c_void,
            cb: AsyncCallback,
            ctx: *mut c_void,
        );
        pub fn un_center_get_notification_settings_async(
            center: *mut c_void,
            cb: AsyncCallback,
            ctx: *mut c_void,
        );
    }
}
