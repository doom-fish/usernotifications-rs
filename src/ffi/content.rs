use crate::ffi::abi::UNBufferRaw;
use core::ffi::c_char;

unsafe extern "C" {
    pub fn un_content_roundtrip_binary(
        content_binary: *const UNBufferRaw,
        error_out: *mut *mut c_char,
    ) -> *mut UNBufferRaw;
    pub fn un_content_updating_with_provider_binary(
        content_binary: *const UNBufferRaw,
        provider_binary: *const UNBufferRaw,
        error_out: *mut *mut c_char,
    ) -> *mut UNBufferRaw;
}
