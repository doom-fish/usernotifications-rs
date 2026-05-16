use core::ffi::{c_char, c_void};

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const FRAMEWORK_ERROR: i32 = -2;
}

pub type CenterEventCallback = unsafe extern "C" fn(*mut c_void, *const c_char);

unsafe extern "C" {
    pub fn un_object_release(ptr: *mut c_void);

    pub fn un_center_current(out_center: *mut *mut c_void, error_out: *mut *mut c_char) -> i32;
    pub fn un_center_set_delegate(
        center: *mut c_void,
        callback: Option<CenterEventCallback>,
        user_info: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_center_clear_delegate(center: *mut c_void);

    pub fn un_center_request_authorization(
        center: *mut c_void,
        options: u64,
        out_granted: *mut bool,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_center_get_notification_settings_json(
        center: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn un_center_set_notification_categories(
        center: *mut c_void,
        categories_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_center_get_notification_categories_json(
        center: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn un_center_add_request(
        center: *mut c_void,
        request_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_center_get_pending_requests_json(
        center: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn un_center_remove_pending_requests(center: *mut c_void, identifiers_json: *const c_char);
    pub fn un_center_remove_all_pending_requests(center: *mut c_void);
    pub fn un_center_get_delivered_notifications_json(
        center: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn un_center_remove_delivered_notifications(
        center: *mut c_void,
        identifiers_json: *const c_char,
    );
    pub fn un_center_remove_all_delivered_notifications(center: *mut c_void);
}
