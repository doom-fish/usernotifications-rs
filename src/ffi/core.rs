use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn un_object_release(ptr: *mut c_void);
    pub fn un_error_domain() -> *mut c_char;
}
