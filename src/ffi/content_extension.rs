use core::ffi::{c_char, c_void};

pub type ContentExtensionNotificationCallback =
    unsafe extern "C" fn(*mut c_void, *const c_char);
pub type ContentExtensionResponseCallback =
    unsafe extern "C" fn(*mut c_void, *const c_char) -> u64;
pub type ContentExtensionSimpleCallback = unsafe extern "C" fn(*mut c_void);

unsafe extern "C" {
    pub fn un_content_extension_context_create(out_context: *mut *mut c_void) -> i32;
    pub fn un_content_extension_context_set_notification_actions(
        context: *mut c_void,
        actions_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_content_extension_context_get_notification_actions_json(
        context: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn un_content_extension_context_perform_default_action(context: *mut c_void);
    pub fn un_content_extension_context_dismiss(context: *mut c_void);
    pub fn un_content_extension_context_media_playing_started(context: *mut c_void);
    pub fn un_content_extension_context_media_playing_paused(context: *mut c_void);

    pub fn un_content_extension_simulator_create(
        notification_callback: Option<ContentExtensionNotificationCallback>,
        response_callback: Option<ContentExtensionResponseCallback>,
        media_play_callback: Option<ContentExtensionSimpleCallback>,
        media_pause_callback: Option<ContentExtensionSimpleCallback>,
        user_info: *mut c_void,
        out_simulator: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_content_extension_simulator_set_media_play_pause_button_type(
        simulator: *mut c_void,
        button_type: u64,
    );
    pub fn un_content_extension_simulator_get_media_play_pause_button_type(
        simulator: *mut c_void,
    ) -> u64;
    pub fn un_content_extension_simulator_set_media_play_pause_button_frame(
        simulator: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    );
    pub fn un_content_extension_simulator_get_media_play_pause_button_frame(
        simulator: *mut c_void,
        x: *mut f64,
        y: *mut f64,
        width: *mut f64,
        height: *mut f64,
    );
    pub fn un_content_extension_simulator_set_media_play_pause_button_tint_color(
        simulator: *mut c_void,
        red: f64,
        green: f64,
        blue: f64,
        alpha: f64,
    );
    pub fn un_content_extension_simulator_get_media_play_pause_button_tint_color(
        simulator: *mut c_void,
        red: *mut f64,
        green: *mut f64,
        blue: *mut f64,
        alpha: *mut f64,
    ) -> bool;
    pub fn un_content_extension_simulator_receive_notification_json(
        simulator: *mut c_void,
        notification_json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_content_extension_simulator_receive_response_json(
        simulator: *mut c_void,
        response_json: *const c_char,
        out_option: *mut u64,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn un_content_extension_simulator_media_play(simulator: *mut c_void);
    pub fn un_content_extension_simulator_media_pause(simulator: *mut c_void);
}
