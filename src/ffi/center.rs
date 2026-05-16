use core::ffi::{c_char, c_void};

pub type CenterEventCallback = unsafe extern "C" fn(*mut c_void, *const c_char);
pub type CenterWillPresentCallback = unsafe extern "C" fn(*mut c_void, *const c_char) -> u64;

unsafe extern "C" {
    pub fn un_center_current(out_center: *mut *mut c_void, error_out: *mut *mut c_char) -> i32;
    pub fn un_center_set_delegate(
        center: *mut c_void,
        event_callback: Option<CenterEventCallback>,
        will_present_callback: Option<CenterWillPresentCallback>,
        user_info: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_center_clear_delegate(center: *mut c_void);
    pub fn un_center_request_authorization(
        center: *mut c_void,
        options: u64,
        out_granted: *mut bool,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_center_supports_content_extensions(center: *mut c_void) -> bool;
    pub fn un_center_set_badge_count(
        center: *mut c_void,
        new_badge_count: isize,
        error_out: *mut *mut c_char,
    ) -> i32;
}
