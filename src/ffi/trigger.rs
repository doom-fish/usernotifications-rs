use crate::ffi::abi::UNBufferRaw;
use core::ffi::c_char;

unsafe extern "C" {
    pub fn un_trigger_roundtrip_binary(
        trigger_binary: *const UNBufferRaw,
        error_out: *mut *mut c_char,
    ) -> *mut UNBufferRaw;
}
