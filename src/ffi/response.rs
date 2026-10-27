use crate::ffi::abi::UNBufferRaw;
use core::ffi::c_char;
use core::ffi::c_void;

unsafe extern "C" {
    pub fn un_center_get_delivered_notifications_binary(
        center: *mut c_void,
        out_binary: *mut *mut UNBufferRaw,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_center_remove_delivered_notifications(
        center: *mut c_void,
        identifiers_binary: *const UNBufferRaw,
    );
    pub fn un_center_remove_all_delivered_notifications(center: *mut c_void);
    pub fn un_response_get_default_action_identifier() -> *mut c_char;
    pub fn un_response_get_dismiss_action_identifier() -> *mut c_char;
}
