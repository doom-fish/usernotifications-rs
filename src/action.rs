use core::ffi::c_char;

use serde::{Deserialize, Serialize};

use crate::content::LocalizedNotificationString;
use crate::error::{from_swift, UserNotificationsError};
use crate::ffi;
use crate::option_set::raw_option_set;
use crate::private::{decode_json, to_cstring};

raw_option_set!(NotificationActionOptions);

impl NotificationActionOptions {
    /// No flags.
    pub const NONE: Self = Self(0);
    /// The authentication required flag.
    pub const AUTHENTICATION_REQUIRED: Self = Self(1 << 0);
    /// The destructive flag.
    pub const DESTRUCTIVE: Self = Self(1 << 1);
    /// The foreground flag.
    pub const FOREGROUND: Self = Self(1 << 2);
}

/// Wraps icon metadata used by `UNNotificationActionIcon`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationActionIcon {
    /// Uses a template image name for the action icon.
    TemplateImage(String),
    /// Uses a system image name for the action icon.
    SystemImage(String),
    /// Fallback for icon values the framework returns but this crate does not model yet.
    Unknown,
}

/// Wraps `UNNotificationAction` and `UNTextInputNotificationAction`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationAction {
    /// The identifier.
    pub identifier: String,
    /// The title.
    pub title: String,
    /// The options.
    pub options: NotificationActionOptions,
    /// The icon.
    pub icon: Option<NotificationActionIcon>,
    /// The text input button title.
    pub text_input_button_title: Option<String>,
    /// The text input placeholder.
    pub text_input_placeholder: Option<String>,
    /// The localized title.
    pub localized_title: Option<LocalizedNotificationString>,
    /// The localized text input button title.
    pub localized_text_input_button_title: Option<LocalizedNotificationString>,
    /// The localized text input placeholder.
    pub localized_text_input_placeholder: Option<LocalizedNotificationString>,
}

impl NotificationAction {
    /// Creates a new notification action.
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
            icon: None,
            text_input_button_title: None,
            text_input_placeholder: None,
            localized_title: None,
            localized_text_input_button_title: None,
            localized_text_input_placeholder: None,
        }
    }

    /// Creates a new text-input notification action.
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
            icon: None,
            text_input_button_title: Some(text_input_button_title.into()),
            text_input_placeholder: Some(text_input_placeholder.into()),
            localized_title: None,
            localized_text_input_button_title: None,
            localized_text_input_placeholder: None,
        }
    }

    /// Sets icon.
    #[must_use]
    pub fn with_icon(mut self, icon: NotificationActionIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets localized title.
    #[must_use]
    pub fn with_localized_title(mut self, localized_title: LocalizedNotificationString) -> Self {
        self.localized_title = Some(localized_title);
        self
    }

    /// Sets localized text input button title.
    #[must_use]
    pub fn with_localized_text_input_button_title(
        mut self,
        localized_text_input_button_title: LocalizedNotificationString,
    ) -> Self {
        self.localized_text_input_button_title = Some(localized_text_input_button_title);
        self
    }

    /// Sets localized text input placeholder.
    #[must_use]
    pub fn with_localized_text_input_placeholder(
        mut self,
        localized_text_input_placeholder: LocalizedNotificationString,
    ) -> Self {
        self.localized_text_input_placeholder = Some(localized_text_input_placeholder);
        self
    }

    /// Round-trips this value through the Swift bridge.
    pub fn bridge_roundtrip(&self) -> Result<Self, UserNotificationsError> {
        let action = encode_action_json(self)?;
        let action = to_cstring(&action)?;
        let mut error = core::ptr::null_mut();
        let payload = unsafe { ffi::action::un_action_roundtrip_json(action.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_action_json(payload)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NotificationActionIconPayload {
    kind: String,
    name: Option<String>,
}

impl From<&NotificationActionIcon> for NotificationActionIconPayload {
    fn from(value: &NotificationActionIcon) -> Self {
        match value {
            NotificationActionIcon::TemplateImage(name) => Self {
                kind: "templateImage".into(),
                name: Some(name.clone()),
            },
            NotificationActionIcon::SystemImage(name) => Self {
                kind: "systemImage".into(),
                name: Some(name.clone()),
            },
            NotificationActionIcon::Unknown => Self {
                kind: "unknown".into(),
                name: None,
            },
        }
    }
}

impl From<NotificationActionIconPayload> for NotificationActionIcon {
    fn from(value: NotificationActionIconPayload) -> Self {
        match value.kind.as_str() {
            "templateImage" => value.name.map_or(Self::Unknown, Self::TemplateImage),
            "systemImage" => value.name.map_or(Self::Unknown, Self::SystemImage),
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NotificationActionPayload {
    identifier: String,
    title: String,
    options: u64,
    icon: Option<NotificationActionIconPayload>,
    text_input_button_title: Option<String>,
    text_input_placeholder: Option<String>,
    localized_title: Option<LocalizedNotificationString>,
    localized_text_input_button_title: Option<LocalizedNotificationString>,
    localized_text_input_placeholder: Option<LocalizedNotificationString>,
}

impl From<&NotificationAction> for NotificationActionPayload {
    fn from(value: &NotificationAction) -> Self {
        Self {
            identifier: value.identifier.clone(),
            title: value.title.clone(),
            options: value.options.bits(),
            icon: value.icon.as_ref().map(NotificationActionIconPayload::from),
            text_input_button_title: value.text_input_button_title.clone(),
            text_input_placeholder: value.text_input_placeholder.clone(),
            localized_title: value.localized_title.clone(),
            localized_text_input_button_title: value.localized_text_input_button_title.clone(),
            localized_text_input_placeholder: value.localized_text_input_placeholder.clone(),
        }
    }
}

impl From<NotificationActionPayload> for NotificationAction {
    fn from(value: NotificationActionPayload) -> Self {
        Self {
            identifier: value.identifier,
            title: value.title,
            options: NotificationActionOptions::from_bits(value.options),
            icon: value.icon.map(Into::into),
            text_input_button_title: value.text_input_button_title,
            text_input_placeholder: value.text_input_placeholder,
            localized_title: value.localized_title,
            localized_text_input_button_title: value.localized_text_input_button_title,
            localized_text_input_placeholder: value.localized_text_input_placeholder,
        }
    }
}

pub(crate) fn encode_action_json(
    action: &NotificationAction,
) -> Result<String, UserNotificationsError> {
    serde_json::to_string(&NotificationActionPayload::from(action)).map_err(|error| {
        UserNotificationsError::FrameworkError(format!(
            "failed to encode notification action: {error}",
        ))
    })
}

pub(crate) fn decode_action_json(
    ptr: *mut c_char,
) -> Result<NotificationAction, UserNotificationsError> {
    decode_json::<NotificationActionPayload>(ptr).map(Into::into)
}
