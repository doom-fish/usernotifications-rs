use core::ffi::c_void;
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNStringRaw {
    pub data: *const u8,
    pub len: usize,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNSliceRaw {
    pub data: *const c_void,
    pub len: usize,
}
#[repr(C)]
pub struct UNBufferRaw {
    pub data: *const c_void,
    pub size: usize,
    pub owner: *mut c_void,
    pub release: Option<unsafe extern "C" fn(*mut Self)>,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNValueRaw {
    pub tag: u32,
    pub signed_value: i64,
    pub unsigned_value: u64,
    pub real_value: f64,
    pub string: UNStringRaw,
    pub values: UNSliceRaw,
    pub entries: UNSliceRaw,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNDictionaryEntryRaw {
    pub key: UNStringRaw,
    pub value: UNValueRaw,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNUserInfoRaw {
    pub entries: UNSliceRaw,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNAttachmentThumbnailTimePayloadRaw {
    pub tag: u32,
    pub seconds: f64,
    pub frame: u64,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNNotificationResponsePayloadRaw {
    pub action_identifier: UNStringRaw,
    pub notification: UNNotificationPayloadRaw,
    pub user_text: *const UNStringRaw,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNNotificationRequestPayloadRaw {
    pub identifier: UNStringRaw,
    pub content: UNNotificationContentPayloadRaw,
    pub trigger: *const UNNotificationTriggerPayloadRaw,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNNotificationActionIconPayloadRaw {
    pub kind: u32,
    pub name: *const UNStringRaw,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNNotificationActionPayloadRaw {
    pub identifier: UNStringRaw,
    pub title: UNStringRaw,
    pub options: u64,
    pub icon: *const UNNotificationActionIconPayloadRaw,
    pub text_input_button_title: *const UNStringRaw,
    pub text_input_placeholder: *const UNStringRaw,
    pub localized_title: *const UNLocalizedNotificationStringRaw,
    pub localized_text_input_button_title: *const UNLocalizedNotificationStringRaw,
    pub localized_text_input_placeholder: *const UNLocalizedNotificationStringRaw,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNNotificationPayloadRaw {
    pub date: f64,
    pub request: UNNotificationRequestPayloadRaw,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNCenterEventPayloadRaw {
    pub event: u32,
    pub notification: *const UNNotificationPayloadRaw,
    pub response: *const UNNotificationResponsePayloadRaw,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNDateComponentsRaw {
    pub year: *const i64,
    pub month: *const i64,
    pub day: *const i64,
    pub hour: *const i64,
    pub minute: *const i64,
    pub second: *const i64,
    pub weekday: *const i64,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNNotificationTriggerPayloadRaw {
    pub kind: u32,
    pub repeats: *const u8,
    pub time_interval: *const f64,
    pub date_components: *const UNDateComponentsRaw,
    pub next_trigger_date: *const f64,
    pub class_name: *const UNStringRaw,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNNotificationCategoryPayloadRaw {
    pub identifier: UNStringRaw,
    pub actions: UNSliceRaw,
    pub intent_identifiers: UNSliceRaw,
    pub options: u64,
    pub hidden_previews_body_placeholder: *const UNStringRaw,
    pub category_summary_format: *const UNStringRaw,
    pub localized_hidden_previews_body_placeholder: *const UNLocalizedNotificationStringRaw,
    pub localized_category_summary_format: *const UNLocalizedNotificationStringRaw,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNNotificationSettingsPayloadRaw {
    pub authorization_status: i32,
    pub sound_setting: i32,
    pub badge_setting: i32,
    pub alert_setting: i32,
    pub notification_center_setting: i32,
    pub lock_screen_setting: i32,
    pub alert_style: i32,
    pub show_previews_setting: i32,
    pub critical_alert_setting: i32,
    pub provides_app_notification_settings: u8,
    pub time_sensitive_setting: i32,
    pub scheduled_delivery_setting: i32,
    pub direct_messages_setting: i32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNLocalizedNotificationStringRaw {
    pub key: UNStringRaw,
    pub arguments: UNSliceRaw,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNNotificationSoundPayloadRaw {
    pub kind: u32,
    pub name: *const UNStringRaw,
    pub volume: *const f32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNNotificationContentPayloadRaw {
    pub title: UNStringRaw,
    pub subtitle: UNStringRaw,
    pub body: UNStringRaw,
    pub badge: *const i64,
    pub category_identifier: UNStringRaw,
    pub thread_identifier: UNStringRaw,
    pub user_info: *const UNUserInfoRaw,
    pub sound: *const UNNotificationSoundPayloadRaw,
    pub attachments: UNSliceRaw,
    pub summary_argument: UNStringRaw,
    pub summary_argument_count: u64,
    pub interruption_level: *const i32,
    pub relevance_score: *const f64,
    pub filter_criteria: *const UNStringRaw,
    pub localized_title: *const UNLocalizedNotificationStringRaw,
    pub localized_subtitle: *const UNLocalizedNotificationStringRaw,
    pub localized_body: *const UNLocalizedNotificationStringRaw,
    pub localized_summary_argument: *const UNLocalizedNotificationStringRaw,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNNotificationMessagePersonPayloadRaw {
    pub handle: UNStringRaw,
    pub handle_type: i32,
    pub display_name: *const UNStringRaw,
    pub contact_identifier: *const UNStringRaw,
    pub custom_identifier: *const UNStringRaw,
    pub is_me: u8,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNNotificationAttributedMessageContextPayloadRaw {
    pub sender: *const UNNotificationMessagePersonPayloadRaw,
    pub recipients: UNSliceRaw,
    pub attributed_content: UNStringRaw,
    pub content: *const UNStringRaw,
    pub outgoing_message_type: i32,
    pub conversation_identifier: *const UNStringRaw,
    pub service_name: *const UNStringRaw,
    pub group_name: *const UNStringRaw,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNNotificationContentProviderPayloadRaw {
    pub kind: u32,
    pub attributed_message_context: *const UNNotificationAttributedMessageContextPayloadRaw,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNAttachmentThumbnailClippingRectPayloadRaw {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNNotificationAttachmentOptionsPayloadRaw {
    pub type_hint: *const UNStringRaw,
    pub thumbnail_hidden: u8,
    pub thumbnail_clipping_rect: *const UNAttachmentThumbnailClippingRectPayloadRaw,
    pub thumbnail_time: *const UNAttachmentThumbnailTimePayloadRaw,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct UNNotificationAttachmentPayloadRaw {
    pub identifier: UNStringRaw,
    pub file_path: UNStringRaw,
    pub attachment_type: *const UNStringRaw,
    pub options: UNNotificationAttachmentOptionsPayloadRaw,
}
