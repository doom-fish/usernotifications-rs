unsafe extern "C" {
    pub(crate) fn free(ptr: *mut core::ffi::c_void);
    pub(crate) fn strdup(ptr: *const core::ffi::c_char) -> *mut core::ffi::c_char;
}
