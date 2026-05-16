use core::ffi::c_char;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::error::{take_owned_c_string, UserNotificationsError};
use crate::ffi;
use crate::notification::{Notification, NotificationPayload};
use crate::option_set::raw_option_set;
use crate::private::decode_json;

raw_option_set!(NotificationPresentationOptions);

impl NotificationPresentationOptions {
    pub const NONE: Self = Self(0);
    pub const BADGE: Self = Self(1 << 0);
    pub const SOUND: Self = Self(1 << 1);
    pub const ALERT: Self = Self(1 << 2);
    pub const LIST: Self = Self(1 << 3);
    pub const BANNER: Self = Self(1 << 4);
}

#[derive(Debug, Clone, PartialEq)]
pub struct NotificationResponse {
    pub action_identifier: String,
    pub notification: Notification,
    pub user_text: Option<String>,
}

impl NotificationResponse {
    #[must_use]
    pub fn is_default_action(&self) -> bool {
        self.action_identifier == default_action_identifier()
    }

    #[must_use]
    pub fn is_dismiss_action(&self) -> bool {
        self.action_identifier == dismiss_action_identifier()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub(crate) fn encode_response_json(
    response: &NotificationResponse,
) -> Result<String, UserNotificationsError> {
    serde_json::to_string(&NotificationResponsePayload::from(response)).map_err(|error| {
        UserNotificationsError::FrameworkError(format!(
            "failed to encode notification response: {error}",
        ))
    })
}

#[allow(dead_code)]
pub(crate) fn decode_response_json(
    ptr: *mut c_char,
) -> Result<NotificationResponse, UserNotificationsError> {
    decode_json::<NotificationResponsePayload>(ptr).map(Into::into)
}

#[must_use]
pub fn default_action_identifier() -> &'static str {
    static DEFAULT_ACTION_IDENTIFIER: OnceLock<String> = OnceLock::new();
    DEFAULT_ACTION_IDENTIFIER
        .get_or_init(|| unsafe {
            take_owned_c_string(ffi::response::un_response_get_default_action_identifier())
        })
        .as_str()
}

#[must_use]
pub fn dismiss_action_identifier() -> &'static str {
    static DISMISS_ACTION_IDENTIFIER: OnceLock<String> = OnceLock::new();
    DISMISS_ACTION_IDENTIFIER
        .get_or_init(|| unsafe {
            take_owned_c_string(ffi::response::un_response_get_dismiss_action_identifier())
        })
        .as_str()
}
