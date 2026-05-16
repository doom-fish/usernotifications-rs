use core::ffi::c_char;

unsafe extern "C" {
    pub fn un_attachment_roundtrip_json(
        attachment_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
}
