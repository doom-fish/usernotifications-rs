use core::ffi::c_char;
use core::ffi::c_void;

unsafe extern "C" {
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
    pub fn un_request_roundtrip_json(
        request_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
}
