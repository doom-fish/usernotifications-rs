use core::ffi::c_char;

unsafe extern "C" {
    pub fn un_content_roundtrip_json(
        content_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
}
