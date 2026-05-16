use core::ffi::c_char;
use core::ffi::c_void;

unsafe extern "C" {
    pub fn un_center_set_notification_categories(
        center: *mut c_void,
        categories_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_center_get_notification_categories_json(
        center: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn un_category_roundtrip_json(
        category_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
}
