use std::ops::{BitOr, BitOrAssign};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::UserNotificationsError;
use crate::private::decode_json;

macro_rules! raw_option_set {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct $name(u64);

        impl $name {
            #[must_use]
            pub const fn from_bits(bits: u64) -> Self {
                Self(bits)
            }

            #[must_use]
            pub const fn bits(self) -> u64 {
                self.0
            }

            #[must_use]
            pub const fn contains(self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }
        }

        impl BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }
    };
}

raw_option_set!(AuthorizationOptions);
raw_option_set!(NotificationActionOptions);
raw_option_set!(NotificationCategoryOptions);

impl AuthorizationOptions {
    pub const NONE: Self = Self(0);
    pub const BADGE: Self = Self(1 << 0);
    pub const SOUND: Self = Self(1 << 1);
    pub const ALERT: Self = Self(1 << 2);
    pub const CRITICAL_ALERT: Self = Self(1 << 4);
    pub const PROVIDES_APP_NOTIFICATION_SETTINGS: Self = Self(1 << 5);
    pub const PROVISIONAL: Self = Self(1 << 6);
    pub const TIME_SENSITIVE: Self = Self(1 << 8);
}

impl NotificationActionOptions {
    pub const NONE: Self = Self(0);
    pub const AUTHENTICATION_REQUIRED: Self = Self(1 << 0);
    pub const DESTRUCTIVE: Self = Self(1 << 1);
    pub const FOREGROUND: Self = Self(1 << 2);
}

impl NotificationCategoryOptions {
    pub const NONE: Self = Self(0);
    pub const CUSTOM_DISMISS_ACTION: Self = Self(1 << 0);
    pub const HIDDEN_PREVIEWS_SHOW_TITLE: Self = Self(1 << 2);
    pub const HIDDEN_PREVIEWS_SHOW_SUBTITLE: Self = Self(1 << 3);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AuthorizationStatus {
    NotDetermined = 0,
    Denied = 1,
    Authorized = 2,
    Provisional = 3,
}

impl AuthorizationStatus {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Denied,
            2 => Self::Authorized,
            3 => Self::Provisional,
            _ => Self::NotDetermined,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NotificationSetting {
    NotSupported = 0,
    Disabled = 1,
    Enabled = 2,
}

impl NotificationSetting {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Disabled,
            2 => Self::Enabled,
            _ => Self::NotSupported,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AlertStyle {
    None = 0,
    Banner = 1,
    Alert = 2,
}

impl AlertStyle {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Banner,
            2 => Self::Alert,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ShowPreviewsSetting {
    Always = 0,
    WhenAuthenticated = 1,
    Never = 2,
}

impl ShowPreviewsSetting {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::WhenAuthenticated,
            2 => Self::Never,
            _ => Self::Always,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotificationSound {
    Default,
    Named(String),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DateComponents {
    pub year: Option<i64>,
    pub month: Option<i64>,
    pub day: Option<i64>,
    pub hour: Option<i64>,
    pub minute: Option<i64>,
    pub second: Option<i64>,
    pub weekday: Option<i64>,
}

impl DateComponents {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_year(mut self, year: i64) -> Self {
        self.year = Some(year);
        self
    }

    #[must_use]
    pub fn with_month(mut self, month: i64) -> Self {
        self.month = Some(month);
        self
    }

    #[must_use]
    pub fn with_day(mut self, day: i64) -> Self {
        self.day = Some(day);
        self
    }

    #[must_use]
    pub fn with_hour(mut self, hour: i64) -> Self {
        self.hour = Some(hour);
        self
    }

    #[must_use]
    pub fn with_minute(mut self, minute: i64) -> Self {
        self.minute = Some(minute);
        self
    }

    #[must_use]
    pub fn with_second(mut self, second: i64) -> Self {
        self.second = Some(second);
        self
    }

    #[must_use]
    pub fn with_weekday(mut self, weekday: i64) -> Self {
        self.weekday = Some(weekday);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimeIntervalTrigger {
    pub time_interval: f64,
    pub repeats: bool,
    pub next_trigger_date: Option<SystemTime>,
}

impl TimeIntervalTrigger {
    #[must_use]
    pub fn new(time_interval: f64, repeats: bool) -> Self {
        Self {
            time_interval,
            repeats,
            next_trigger_date: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarTrigger {
    pub date_components: DateComponents,
    pub repeats: bool,
    pub next_trigger_date: Option<SystemTime>,
}

impl CalendarTrigger {
    #[must_use]
    pub fn new(date_components: DateComponents, repeats: bool) -> Self {
        Self {
            date_components,
            repeats,
            next_trigger_date: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationTrigger {
    TimeInterval(TimeIntervalTrigger),
    Calendar(CalendarTrigger),
    Push,
    Unknown { class_name: String, repeats: bool },
}

impl NotificationTrigger {
    #[must_use]
    pub fn time_interval(seconds: f64, repeats: bool) -> Self {
        Self::TimeInterval(TimeIntervalTrigger::new(seconds, repeats))
    }

    #[must_use]
    pub fn calendar(date_components: DateComponents, repeats: bool) -> Self {
        Self::Calendar(CalendarTrigger::new(date_components, repeats))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NotificationContent {
    pub title: String,
    pub subtitle: String,
    pub body: String,
    pub badge: Option<i64>,
    pub category_identifier: String,
    pub thread_identifier: String,
    pub user_info: Option<Value>,
    pub sound: Option<NotificationSound>,
}

impl NotificationContent {
    #[must_use]
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            ..Self::default()
        }
    }

    #[must_use]
    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = subtitle.into();
        self
    }

    #[must_use]
    pub fn with_badge(mut self, badge: i64) -> Self {
        self.badge = Some(badge);
        self
    }

    #[must_use]
    pub fn with_category_identifier(mut self, category_identifier: impl Into<String>) -> Self {
        self.category_identifier = category_identifier.into();
        self
    }

    #[must_use]
    pub fn with_thread_identifier(mut self, thread_identifier: impl Into<String>) -> Self {
        self.thread_identifier = thread_identifier.into();
        self
    }

    #[must_use]
    pub fn with_user_info(mut self, user_info: Value) -> Self {
        self.user_info = Some(user_info);
        self
    }

    #[must_use]
    pub fn with_sound(mut self, sound: NotificationSound) -> Self {
        self.sound = Some(sound);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationAction {
    pub identifier: String,
    pub title: String,
    pub options: NotificationActionOptions,
    pub text_input_button_title: Option<String>,
    pub text_input_placeholder: Option<String>,
}

impl NotificationAction {
    #[must_use]
    pub fn new(
        identifier: impl Into<String>,
        title: impl Into<String>,
        options: NotificationActionOptions,
    ) -> Self {
        Self {
            identifier: identifier.into(),
            title: title.into(),
            options,
            text_input_button_title: None,
            text_input_placeholder: None,
        }
    }

    #[must_use]
    pub fn new_text_input(
        identifier: impl Into<String>,
        title: impl Into<String>,
        options: NotificationActionOptions,
        text_input_button_title: impl Into<String>,
        text_input_placeholder: impl Into<String>,
    ) -> Self {
        Self {
            identifier: identifier.into(),
            title: title.into(),
            options,
            text_input_button_title: Some(text_input_button_title.into()),
            text_input_placeholder: Some(text_input_placeholder.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationCategory {
    pub identifier: String,
    pub actions: Vec<NotificationAction>,
    pub intent_identifiers: Vec<String>,
    pub options: NotificationCategoryOptions,
}

impl NotificationCategory {
    #[must_use]
    pub fn new(
        identifier: impl Into<String>,
        actions: Vec<NotificationAction>,
        intent_identifiers: Vec<String>,
        options: NotificationCategoryOptions,
    ) -> Self {
        Self {
            identifier: identifier.into(),
            actions,
            intent_identifiers,
            options,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NotificationRequest {
    pub identifier: String,
    pub content: NotificationContent,
    pub trigger: Option<NotificationTrigger>,
}

impl NotificationRequest {
    #[must_use]
    pub fn new(
        identifier: impl Into<String>,
        content: NotificationContent,
        trigger: Option<NotificationTrigger>,
    ) -> Self {
        Self {
            identifier: identifier.into(),
            content,
            trigger,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub date: SystemTime,
    pub request: NotificationRequest,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NotificationResponse {
    pub action_identifier: String,
    pub notification: Notification,
    pub user_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationSettings {
    pub authorization_status: AuthorizationStatus,
    pub sound_setting: NotificationSetting,
    pub badge_setting: NotificationSetting,
    pub alert_setting: NotificationSetting,
    pub notification_center_setting: NotificationSetting,
    pub lock_screen_setting: NotificationSetting,
    pub alert_style: AlertStyle,
    pub show_previews_setting: ShowPreviewsSetting,
    pub critical_alert_setting: NotificationSetting,
    pub provides_app_notification_settings: bool,
    pub time_sensitive_setting: NotificationSetting,
    pub scheduled_delivery_setting: NotificationSetting,
    pub direct_messages_setting: NotificationSetting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NotificationSoundPayload {
    kind: String,
    name: Option<String>,
}

impl From<&NotificationSound> for NotificationSoundPayload {
    fn from(value: &NotificationSound) -> Self {
        match value {
            NotificationSound::Default => Self {
                kind: "default".into(),
                name: None,
            },
            NotificationSound::Named(name) => Self {
                kind: "named".into(),
                name: Some(name.clone()),
            },
            NotificationSound::Unknown => Self {
                kind: "unknown".into(),
                name: None,
            },
        }
    }
}

impl From<NotificationSoundPayload> for NotificationSound {
    fn from(value: NotificationSoundPayload) -> Self {
        match value.kind.as_str() {
            "default" => Self::Default,
            "named" => value.name.map_or(Self::Unknown, Self::Named),
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DateComponentsPayload {
    year: Option<i64>,
    month: Option<i64>,
    day: Option<i64>,
    hour: Option<i64>,
    minute: Option<i64>,
    second: Option<i64>,
    weekday: Option<i64>,
}

impl From<&DateComponents> for DateComponentsPayload {
    fn from(value: &DateComponents) -> Self {
        Self {
            year: value.year,
            month: value.month,
            day: value.day,
            hour: value.hour,
            minute: value.minute,
            second: value.second,
            weekday: value.weekday,
        }
    }
}

impl From<DateComponentsPayload> for DateComponents {
    fn from(value: DateComponentsPayload) -> Self {
        Self {
            year: value.year,
            month: value.month,
            day: value.day,
            hour: value.hour,
            minute: value.minute,
            second: value.second,
            weekday: value.weekday,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NotificationContentPayload {
    title: String,
    subtitle: String,
    body: String,
    badge: Option<i64>,
    category_identifier: String,
    thread_identifier: String,
    user_info: Option<Value>,
    sound: Option<NotificationSoundPayload>,
}

impl From<&NotificationContent> for NotificationContentPayload {
    fn from(value: &NotificationContent) -> Self {
        Self {
            title: value.title.clone(),
            subtitle: value.subtitle.clone(),
            body: value.body.clone(),
            badge: value.badge,
            category_identifier: value.category_identifier.clone(),
            thread_identifier: value.thread_identifier.clone(),
            user_info: value.user_info.clone(),
            sound: value.sound.as_ref().map(NotificationSoundPayload::from),
        }
    }
}

impl From<NotificationContentPayload> for NotificationContent {
    fn from(value: NotificationContentPayload) -> Self {
        Self {
            title: value.title,
            subtitle: value.subtitle,
            body: value.body,
            badge: value.badge,
            category_identifier: value.category_identifier,
            thread_identifier: value.thread_identifier,
            user_info: value.user_info,
            sound: value.sound.map(Into::into),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NotificationTriggerPayload {
    kind: String,
    repeats: Option<bool>,
    time_interval: Option<f64>,
    date_components: Option<DateComponentsPayload>,
    next_trigger_date: Option<f64>,
    class_name: Option<String>,
}

impl From<&NotificationTrigger> for NotificationTriggerPayload {
    fn from(value: &NotificationTrigger) -> Self {
        match value {
            NotificationTrigger::TimeInterval(trigger) => Self {
                kind: "timeInterval".into(),
                repeats: Some(trigger.repeats),
                time_interval: Some(trigger.time_interval),
                date_components: None,
                next_trigger_date: timestamp_from_system_time_opt(
                    trigger.next_trigger_date.as_ref(),
                ),
                class_name: None,
            },
            NotificationTrigger::Calendar(trigger) => Self {
                kind: "calendar".into(),
                repeats: Some(trigger.repeats),
                time_interval: None,
                date_components: Some(DateComponentsPayload::from(&trigger.date_components)),
                next_trigger_date: timestamp_from_system_time_opt(
                    trigger.next_trigger_date.as_ref(),
                ),
                class_name: None,
            },
            NotificationTrigger::Push => Self {
                kind: "push".into(),
                repeats: Some(false),
                time_interval: None,
                date_components: None,
                next_trigger_date: None,
                class_name: None,
            },
            NotificationTrigger::Unknown {
                class_name,
                repeats,
            } => Self {
                kind: "unknown".into(),
                repeats: Some(*repeats),
                time_interval: None,
                date_components: None,
                next_trigger_date: None,
                class_name: Some(class_name.clone()),
            },
        }
    }
}

impl From<NotificationTriggerPayload> for NotificationTrigger {
    fn from(value: NotificationTriggerPayload) -> Self {
        match value.kind.as_str() {
            "timeInterval" => Self::TimeInterval(TimeIntervalTrigger {
                time_interval: value.time_interval.unwrap_or_default(),
                repeats: value.repeats.unwrap_or(false),
                next_trigger_date: system_time_from_timestamp_opt(value.next_trigger_date),
            }),
            "calendar" => Self::Calendar(CalendarTrigger {
                date_components: value
                    .date_components
                    .map_or_else(DateComponents::new, Into::into),
                repeats: value.repeats.unwrap_or(false),
                next_trigger_date: system_time_from_timestamp_opt(value.next_trigger_date),
            }),
            "push" => Self::Push,
            _ => Self::Unknown {
                class_name: value
                    .class_name
                    .unwrap_or_else(|| "UNNotificationTrigger".into()),
                repeats: value.repeats.unwrap_or(false),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NotificationActionPayload {
    identifier: String,
    title: String,
    options: u64,
    text_input_button_title: Option<String>,
    text_input_placeholder: Option<String>,
}

impl From<&NotificationAction> for NotificationActionPayload {
    fn from(value: &NotificationAction) -> Self {
        Self {
            identifier: value.identifier.clone(),
            title: value.title.clone(),
            options: value.options.bits(),
            text_input_button_title: value.text_input_button_title.clone(),
            text_input_placeholder: value.text_input_placeholder.clone(),
        }
    }
}

impl From<NotificationActionPayload> for NotificationAction {
    fn from(value: NotificationActionPayload) -> Self {
        Self {
            identifier: value.identifier,
            title: value.title,
            options: NotificationActionOptions::from_bits(value.options),
            text_input_button_title: value.text_input_button_title,
            text_input_placeholder: value.text_input_placeholder,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NotificationCategoryPayload {
    identifier: String,
    actions: Vec<NotificationActionPayload>,
    intent_identifiers: Vec<String>,
    options: u64,
}

impl From<&NotificationCategory> for NotificationCategoryPayload {
    fn from(value: &NotificationCategory) -> Self {
        Self {
            identifier: value.identifier.clone(),
            actions: value
                .actions
                .iter()
                .map(NotificationActionPayload::from)
                .collect(),
            intent_identifiers: value.intent_identifiers.clone(),
            options: value.options.bits(),
        }
    }
}

impl From<NotificationCategoryPayload> for NotificationCategory {
    fn from(value: NotificationCategoryPayload) -> Self {
        Self {
            identifier: value.identifier,
            actions: value.actions.into_iter().map(Into::into).collect(),
            intent_identifiers: value.intent_identifiers,
            options: NotificationCategoryOptions::from_bits(value.options),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NotificationRequestPayload {
    identifier: String,
    content: NotificationContentPayload,
    trigger: Option<NotificationTriggerPayload>,
}

impl From<&NotificationRequest> for NotificationRequestPayload {
    fn from(value: &NotificationRequest) -> Self {
        Self {
            identifier: value.identifier.clone(),
            content: NotificationContentPayload::from(&value.content),
            trigger: value.trigger.as_ref().map(NotificationTriggerPayload::from),
        }
    }
}

impl From<NotificationRequestPayload> for NotificationRequest {
    fn from(value: NotificationRequestPayload) -> Self {
        Self {
            identifier: value.identifier,
            content: value.content.into(),
            trigger: value.trigger.map(Into::into),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct NotificationPayload {
    date: f64,
    request: NotificationRequestPayload,
}

impl From<NotificationPayload> for Notification {
    fn from(value: NotificationPayload) -> Self {
        Self {
            date: UNIX_EPOCH + Duration::from_secs_f64(value.date.max(0.0)),
            request: value.request.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct NotificationResponsePayload {
    action_identifier: String,
    notification: NotificationPayload,
    user_text: Option<String>,
}

impl From<NotificationResponsePayload> for NotificationResponse {
    fn from(value: NotificationResponsePayload) -> Self {
        Self {
            action_identifier: value.action_identifier,
            notification: value.notification.into(),
            user_text: value.user_text,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct NotificationSettingsPayload {
    authorization_status: i32,
    sound_setting: i32,
    badge_setting: i32,
    alert_setting: i32,
    notification_center_setting: i32,
    lock_screen_setting: i32,
    alert_style: i32,
    show_previews_setting: i32,
    critical_alert_setting: i32,
    provides_app_notification_settings: bool,
    time_sensitive_setting: i32,
    scheduled_delivery_setting: i32,
    direct_messages_setting: i32,
}

impl From<NotificationSettingsPayload> for NotificationSettings {
    fn from(value: NotificationSettingsPayload) -> Self {
        Self {
            authorization_status: AuthorizationStatus::from_raw(value.authorization_status),
            sound_setting: NotificationSetting::from_raw(value.sound_setting),
            badge_setting: NotificationSetting::from_raw(value.badge_setting),
            alert_setting: NotificationSetting::from_raw(value.alert_setting),
            notification_center_setting: NotificationSetting::from_raw(
                value.notification_center_setting,
            ),
            lock_screen_setting: NotificationSetting::from_raw(value.lock_screen_setting),
            alert_style: AlertStyle::from_raw(value.alert_style),
            show_previews_setting: ShowPreviewsSetting::from_raw(value.show_previews_setting),
            critical_alert_setting: NotificationSetting::from_raw(value.critical_alert_setting),
            provides_app_notification_settings: value.provides_app_notification_settings,
            time_sensitive_setting: NotificationSetting::from_raw(value.time_sensitive_setting),
            scheduled_delivery_setting: NotificationSetting::from_raw(
                value.scheduled_delivery_setting,
            ),
            direct_messages_setting: NotificationSetting::from_raw(value.direct_messages_setting),
        }
    }
}

#[allow(clippy::single_option_map)]
fn timestamp_from_system_time_opt(time: Option<&SystemTime>) -> Option<f64> {
    time.map(|time| {
        time.duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs_f64()
    })
}

#[allow(clippy::single_option_map)]
fn system_time_from_timestamp_opt(timestamp: Option<f64>) -> Option<SystemTime> {
    timestamp.map(|timestamp| UNIX_EPOCH + Duration::from_secs_f64(timestamp.max(0.0)))
}

pub(crate) fn encode_request_json(
    request: &NotificationRequest,
) -> Result<String, UserNotificationsError> {
    serde_json::to_string(&NotificationRequestPayload::from(request)).map_err(|error| {
        UserNotificationsError::FrameworkError(format!(
            "failed to encode notification request: {error}"
        ))
    })
}

pub(crate) fn encode_categories_json(
    categories: &[NotificationCategory],
) -> Result<String, UserNotificationsError> {
    let payloads = categories
        .iter()
        .map(NotificationCategoryPayload::from)
        .collect::<Vec<_>>();
    serde_json::to_string(&payloads).map_err(|error| {
        UserNotificationsError::FrameworkError(format!(
            "failed to encode notification categories: {error}"
        ))
    })
}

pub(crate) fn decode_settings_json(
    ptr: *mut core::ffi::c_char,
) -> Result<NotificationSettings, UserNotificationsError> {
    decode_json::<NotificationSettingsPayload>(ptr).map(Into::into)
}

pub(crate) fn decode_categories_json(
    ptr: *mut core::ffi::c_char,
) -> Result<Vec<NotificationCategory>, UserNotificationsError> {
    decode_json::<Vec<NotificationCategoryPayload>>(ptr)
        .map(|payloads| payloads.into_iter().map(Into::into).collect())
}

pub(crate) fn decode_requests_json(
    ptr: *mut core::ffi::c_char,
) -> Result<Vec<NotificationRequest>, UserNotificationsError> {
    decode_json::<Vec<NotificationRequestPayload>>(ptr)
        .map(|payloads| payloads.into_iter().map(Into::into).collect())
}

pub(crate) fn decode_notifications_json(
    ptr: *mut core::ffi::c_char,
) -> Result<Vec<Notification>, UserNotificationsError> {
    decode_json::<Vec<NotificationPayload>>(ptr)
        .map(|payloads| payloads.into_iter().map(Into::into).collect())
}
