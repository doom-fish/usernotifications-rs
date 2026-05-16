use core::ffi::c_char;
use core::ffi::c_void;

unsafe extern "C" {
    pub fn un_center_get_delivered_notifications_json(
        center: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn un_center_remove_delivered_notifications(
        center: *mut c_void,
        identifiers_json: *const c_char,
    );
    pub fn un_center_remove_all_delivered_notifications(center: *mut c_void);
    pub fn un_response_get_default_action_identifier() -> *mut c_char;
    pub fn un_response_get_dismiss_action_identifier() -> *mut c_char;
}
