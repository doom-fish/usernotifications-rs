use crate::ffi::abi::UNBufferRaw;
use core::ffi::c_char;

unsafe extern "C" {
    pub fn un_attachment_roundtrip_binary(
        attachment_binary: *const UNBufferRaw,
        error_out: *mut *mut c_char,
    ) -> *mut UNBufferRaw;
}
