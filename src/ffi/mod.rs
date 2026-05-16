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
