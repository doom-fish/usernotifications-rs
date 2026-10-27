use std::sync::OnceLock;

use crate::binary::decode_binary;
use crate::error::{take_owned_c_string, UserNotificationsError};
use crate::ffi;
use crate::notification::{Notification, NotificationPayload};
use crate::option_set::raw_option_set;

raw_option_set!(NotificationPresentationOptions);

impl NotificationPresentationOptions {
    /// No flags.
    pub const NONE: Self = Self(0);
    /// The badge flag.
    pub const BADGE: Self = Self(1 << 0);
    /// The sound flag.
    pub const SOUND: Self = Self(1 << 1);
    /// The alert flag.
    pub const ALERT: Self = Self(1 << 2);
    /// The list flag.
    pub const LIST: Self = Self(1 << 3);
    /// The banner flag.
    pub const BANNER: Self = Self(1 << 4);
}

/// Wraps `UNNotificationResponse` and `UNTextInputNotificationResponse`.
#[derive(Debug, Clone, PartialEq)]
pub struct NotificationResponse {
    /// The action identifier.
    pub action_identifier: String,
    /// The notification.
    pub notification: Notification,
    /// The user text.
    pub user_text: Option<String>,
}

impl NotificationResponse {
    /// Returns whether this response uses the default action identifier.
    #[must_use]
    pub fn is_default_action(&self) -> bool {
        self.action_identifier == default_action_identifier()
    }

    /// Returns whether this response uses the dismiss action identifier.
    #[must_use]
    pub fn is_dismiss_action(&self) -> bool {
        self.action_identifier == dismiss_action_identifier()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NotificationResponsePayload {
    action_identifier: String,
    notification: NotificationPayload,
    user_text: Option<String>,
}

impl From<&NotificationResponse> for NotificationResponsePayload {
    fn from(value: &NotificationResponse) -> Self {
        Self {
            action_identifier: value.action_identifier.clone(),
            notification: NotificationPayload::from(&value.notification),
            user_text: value.user_text.clone(),
        }
    }
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

pub(crate) fn encode_response_binary(response: &NotificationResponse) -> crate::binary::Input {
    crate::binary::Input::new(&NotificationResponsePayload::from(response))
}

#[allow(dead_code)]
pub(crate) fn decode_response_binary(
    ptr: *mut crate::ffi::abi::UNBufferRaw,
) -> Result<NotificationResponse, UserNotificationsError> {
    decode_binary::<NotificationResponsePayload>(ptr).map(Into::into)
}

/// Returns `UNNotificationDefaultActionIdentifier`.
#[must_use]
pub fn default_action_identifier() -> &'static str {
    static DEFAULT_ACTION_IDENTIFIER: OnceLock<String> = OnceLock::new();
    DEFAULT_ACTION_IDENTIFIER
        .get_or_init(|| unsafe {
            take_owned_c_string(ffi::response::un_response_get_default_action_identifier())
        })
        .as_str()
}

/// Returns `UNNotificationDismissActionIdentifier`.
#[must_use]
pub fn dismiss_action_identifier() -> &'static str {
    static DISMISS_ACTION_IDENTIFIER: OnceLock<String> = OnceLock::new();
    DISMISS_ACTION_IDENTIFIER
        .get_or_init(|| unsafe {
            take_owned_c_string(ffi::response::un_response_get_dismiss_action_identifier())
        })
        .as_str()
}

impl crate::binary::Record for NotificationResponsePayload {
    type Raw = crate::ffi::abi::UNNotificationResponsePayloadRaw;
    fn raw(&self, arena: &mut crate::binary::Arena) -> Self::Raw {
        Self::Raw {
            action_identifier: crate::binary::Record::raw(&self.action_identifier, arena),
            notification: crate::binary::Record::raw(&self.notification, arena),
            user_text: self.user_text.as_ref().map_or(core::ptr::null(), |value| {
                let raw = crate::binary::Record::raw(value, arena);
                arena.keep(raw)
            }),
        }
    }
    unsafe fn read(raw: &Self::Raw) -> Result<Self, UserNotificationsError> {
        Ok(Self {
            action_identifier: unsafe {
                <String as crate::binary::Record>::read(&raw.action_identifier)
            }?,
            notification: unsafe {
                <NotificationPayload as crate::binary::Record>::read(&raw.notification)
            }?,
            user_text: unsafe { raw.user_text.as_ref() }
                .map(|value| unsafe { <String as crate::binary::Record>::read(value) })
                .transpose()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::NotificationPresentationOptions;

    #[test]
    fn notification_presentation_options_round_trip_bits() {
        let options = NotificationPresentationOptions::BADGE
            | NotificationPresentationOptions::SOUND
            | NotificationPresentationOptions::BANNER;
        let roundtrip = NotificationPresentationOptions::from_bits(options.bits());

        assert_eq!(roundtrip.bits(), options.bits());
        assert!(roundtrip.contains(NotificationPresentationOptions::BADGE));
        assert!(roundtrip.contains(NotificationPresentationOptions::SOUND));
        assert!(roundtrip.contains(NotificationPresentationOptions::BANNER));
        assert!(!roundtrip.contains(NotificationPresentationOptions::ALERT));
    }

    #[test]
    fn notification_presentation_options_none_is_empty() {
        let options = NotificationPresentationOptions::NONE;

        assert_eq!(options.bits(), 0);
        assert!(!options.contains(NotificationPresentationOptions::BADGE));
        assert!(!options.contains(NotificationPresentationOptions::LIST));
    }
}
