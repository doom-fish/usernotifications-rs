use crate::ffi::abi::UNBufferRaw;
use core::ffi::{c_char, c_void};

pub type ServiceExtensionReceiveCallback =
    unsafe extern "C" fn(*mut c_void, *const UNBufferRaw, *mut *mut c_char) -> *mut UNBufferRaw;
pub type ServiceExtensionExpireCallback = unsafe extern "C" fn(*mut c_void);

unsafe extern "C" {
    pub fn un_service_extension_simulator_create(
        receive_callback: Option<ServiceExtensionReceiveCallback>,
        expire_callback: Option<ServiceExtensionExpireCallback>,
        user_info: *mut c_void,
        out_simulator: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_service_extension_simulator_receive_request_binary(
        simulator: *mut c_void,
        request_binary: *const UNBufferRaw,
        error_out: *mut *mut c_char,
    ) -> *mut UNBufferRaw;
    pub fn un_service_extension_simulator_time_will_expire(simulator: *mut c_void);
}
