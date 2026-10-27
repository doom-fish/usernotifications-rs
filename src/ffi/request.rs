use crate::ffi::abi::UNBufferRaw;
use core::ffi::c_char;
use core::ffi::c_void;

unsafe extern "C" {
    pub fn un_center_add_request(
        center: *mut c_void,
        request_binary: *const UNBufferRaw,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_center_get_pending_requests_binary(
        center: *mut c_void,
        out_binary: *mut *mut UNBufferRaw,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_center_remove_pending_requests(
        center: *mut c_void,
        identifiers_binary: *const UNBufferRaw,
    );
    pub fn un_center_remove_all_pending_requests(center: *mut c_void);
    pub fn un_request_roundtrip_binary(
        request_binary: *const UNBufferRaw,
        error_out: *mut *mut c_char,
    ) -> *mut UNBufferRaw;
}
