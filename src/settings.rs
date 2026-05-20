use core::ffi::c_char;

use serde::Deserialize;

use crate::error::UserNotificationsError;
use crate::option_set::raw_option_set;
use crate::private::decode_json;

raw_option_set!(AuthorizationOptions);

impl AuthorizationOptions {
    /// No flags.
    pub const NONE: Self = Self(0);
    /// The badge flag.
    pub const BADGE: Self = Self(1 << 0);
    /// The sound flag.
    pub const SOUND: Self = Self(1 << 1);
    /// The alert flag.
    pub const ALERT: Self = Self(1 << 2);
    /// The car play flag.
    pub const CAR_PLAY: Self = Self(1 << 3);
    /// The critical alert flag.
    pub const CRITICAL_ALERT: Self = Self(1 << 4);
    /// The provides app notification settings flag.
    pub const PROVIDES_APP_NOTIFICATION_SETTINGS: Self = Self(1 << 5);
    /// The provisional flag.
    pub const PROVISIONAL: Self = Self(1 << 6);
    /// The time sensitive flag.
    pub const TIME_SENSITIVE: Self = Self(1 << 8);
}

/// Wraps the `UNAuthorizationStatus` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AuthorizationStatus {
    /// Authorization has not been requested yet.
    NotDetermined = 0,
    /// Authorization was denied.
    Denied = 1,
    /// Authorization was granted.
    Authorized = 2,
    /// Provisional authorization was granted.
    Provisional = 3,
}

impl AuthorizationStatus {
    /// Converts a raw framework value into an authorization status.
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

/// Wraps the `UNNotificationSetting` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NotificationSetting {
    /// The setting is not supported on this platform.
    NotSupported = 0,
    /// The setting is disabled.
    Disabled = 1,
    /// The setting is enabled.
    Enabled = 2,
}

impl NotificationSetting {
    /// Converts a raw framework value into a notification setting.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Disabled,
            2 => Self::Enabled,
            _ => Self::NotSupported,
        }
    }
}

/// Wraps the `UNAlertStyle` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AlertStyle {
    /// No alert style is used.
    None = 0,
    /// Uses the banner alert style.
    Banner = 1,
    /// Uses the alert style.
    Alert = 2,
}

impl AlertStyle {
    /// Converts a raw framework value into an alert style.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Banner,
            2 => Self::Alert,
            _ => Self::None,
        }
    }
}

/// Wraps the `UNShowPreviewsSetting` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ShowPreviewsSetting {
    /// Always show notification previews.
    Always = 0,
    /// Show previews only when the device is authenticated.
    WhenAuthenticated = 1,
    /// Never show notification previews.
    Never = 2,
}

impl ShowPreviewsSetting {
    /// Converts a raw framework value into a show-previews setting.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::WhenAuthenticated,
            2 => Self::Never,
            _ => Self::Always,
        }
    }
}

/// Wraps `UNNotificationSettings`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationSettings {
    /// The authorization status.
    pub authorization_status: AuthorizationStatus,
    /// The sound setting.
    pub sound_setting: NotificationSetting,
    /// The badge setting.
    pub badge_setting: NotificationSetting,
    /// The alert setting.
    pub alert_setting: NotificationSetting,
    /// The notification center setting.
    pub notification_center_setting: NotificationSetting,
    /// The lock screen setting.
    pub lock_screen_setting: NotificationSetting,
    /// The alert style.
    pub alert_style: AlertStyle,
    /// The show previews setting.
    pub show_previews_setting: ShowPreviewsSetting,
    /// The critical alert setting.
    pub critical_alert_setting: NotificationSetting,
    /// Whether the app provides in-app notification settings.
    pub provides_app_notification_settings: bool,
    /// The time sensitive setting.
    pub time_sensitive_setting: NotificationSetting,
    /// The scheduled delivery setting.
    pub scheduled_delivery_setting: NotificationSetting,
    /// The direct messages setting.
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

#[cfg(test)]
mod tests {
    use super::{
        AlertStyle, AuthorizationOptions, AuthorizationStatus, NotificationSetting,
        ShowPreviewsSetting,
    };

    #[test]
    fn authorization_options_round_trip_bits() {
        let options = AuthorizationOptions::BADGE
            | AuthorizationOptions::ALERT
            | AuthorizationOptions::PROVISIONAL;
        let roundtrip = AuthorizationOptions::from_bits(options.bits());

        assert_eq!(roundtrip.bits(), options.bits());
        assert!(roundtrip.contains(AuthorizationOptions::BADGE));
        assert!(roundtrip.contains(AuthorizationOptions::ALERT));
        assert!(roundtrip.contains(AuthorizationOptions::PROVISIONAL));
        assert!(!roundtrip.contains(AuthorizationOptions::SOUND));
    }

    #[test]
    fn authorization_status_from_raw_matches_documented_values() {
        assert_eq!(
            AuthorizationStatus::from_raw(0),
            AuthorizationStatus::NotDetermined
        );
        assert_eq!(
            AuthorizationStatus::from_raw(1),
            AuthorizationStatus::Denied
        );
        assert_eq!(
            AuthorizationStatus::from_raw(2),
            AuthorizationStatus::Authorized
        );
        assert_eq!(
            AuthorizationStatus::from_raw(3),
            AuthorizationStatus::Provisional
        );
        assert_eq!(
            AuthorizationStatus::from_raw(99),
            AuthorizationStatus::NotDetermined
        );
    }

    #[test]
    fn notification_setting_from_raw_matches_documented_values() {
        assert_eq!(
            NotificationSetting::from_raw(0),
            NotificationSetting::NotSupported
        );
        assert_eq!(
            NotificationSetting::from_raw(1),
            NotificationSetting::Disabled
        );
        assert_eq!(
            NotificationSetting::from_raw(2),
            NotificationSetting::Enabled
        );
        assert_eq!(
            NotificationSetting::from_raw(99),
            NotificationSetting::NotSupported
        );
    }

    #[test]
    fn alert_style_from_raw_matches_documented_values() {
        assert_eq!(AlertStyle::from_raw(0), AlertStyle::None);
        assert_eq!(AlertStyle::from_raw(1), AlertStyle::Banner);
        assert_eq!(AlertStyle::from_raw(2), AlertStyle::Alert);
        assert_eq!(AlertStyle::from_raw(99), AlertStyle::None);
    }

    #[test]
    fn show_previews_setting_from_raw_matches_documented_values() {
        assert_eq!(
            ShowPreviewsSetting::from_raw(0),
            ShowPreviewsSetting::Always
        );
        assert_eq!(
            ShowPreviewsSetting::from_raw(1),
            ShowPreviewsSetting::WhenAuthenticated,
        );
        assert_eq!(ShowPreviewsSetting::from_raw(2), ShowPreviewsSetting::Never);
        assert_eq!(
            ShowPreviewsSetting::from_raw(99),
            ShowPreviewsSetting::Always
        );
    }
}
