use crate::ffi::abi::UNBufferRaw;
use core::ffi::c_char;

unsafe extern "C" {
    pub fn un_action_roundtrip_binary(
        action_binary: *const UNBufferRaw,
        error_out: *mut *mut c_char,
    ) -> *mut UNBufferRaw;
}
