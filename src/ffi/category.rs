use crate::ffi::abi::UNBufferRaw;
use core::ffi::c_char;
use core::ffi::c_void;

unsafe extern "C" {
    pub fn un_center_set_notification_categories(
        center: *mut c_void,
        categories_binary: *const UNBufferRaw,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_center_get_notification_categories_binary(
        center: *mut c_void,
        out_binary: *mut *mut UNBufferRaw,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_category_roundtrip_binary(
        category_binary: *const UNBufferRaw,
        error_out: *mut *mut c_char,
    ) -> *mut UNBufferRaw;
}
