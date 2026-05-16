use core::ffi::{c_char, c_void};

pub type ServiceExtensionReceiveCallback =
    unsafe extern "C" fn(*mut c_void, *const c_char, *mut *mut c_char) -> *mut c_char;
pub type ServiceExtensionExpireCallback = unsafe extern "C" fn(*mut c_void);

unsafe extern "C" {
    pub fn un_service_extension_simulator_create(
        receive_callback: Option<ServiceExtensionReceiveCallback>,
        expire_callback: Option<ServiceExtensionExpireCallback>,
        user_info: *mut c_void,
        out_simulator: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_service_extension_simulator_receive_request_json(
        simulator: *mut c_void,
        request_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn un_service_extension_simulator_time_will_expire(simulator: *mut c_void);
}
