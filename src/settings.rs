use core::ffi::c_char;

use serde::Deserialize;

use crate::error::UserNotificationsError;
use crate::option_set::raw_option_set;
use crate::private::decode_json;

raw_option_set!(AuthorizationOptions);

impl AuthorizationOptions {
    pub const NONE: Self = Self(0);
    pub const BADGE: Self = Self(1 << 0);
    pub const SOUND: Self = Self(1 << 1);
    pub const ALERT: Self = Self(1 << 2);
    pub const CAR_PLAY: Self = Self(1 << 3);
    pub const CRITICAL_ALERT: Self = Self(1 << 4);
    pub const PROVIDES_APP_NOTIFICATION_SETTINGS: Self = Self(1 << 5);
    pub const PROVISIONAL: Self = Self(1 << 6);
    pub const TIME_SENSITIVE: Self = Self(1 << 8);
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

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct NotificationSettingsPayload {
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

pub(crate) fn decode_settings_json(
    ptr: *mut c_char,
) -> Result<NotificationSettings, UserNotificationsError> {
    decode_json::<NotificationSettingsPayload>(ptr).map(Into::into)
}
