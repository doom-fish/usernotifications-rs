use core::ffi::c_char;

use serde::{Deserialize, Serialize};

use crate::action::{NotificationAction, NotificationActionPayload};
use crate::content::LocalizedNotificationString;
use crate::error::{from_swift, UserNotificationsError};
use crate::ffi;
use crate::option_set::raw_option_set;
use crate::private::{decode_json, to_cstring};

raw_option_set!(NotificationCategoryOptions);

impl NotificationCategoryOptions {
    /// No flags.
    pub const NONE: Self = Self(0);
    /// The custom dismiss action flag.
    pub const CUSTOM_DISMISS_ACTION: Self = Self(1 << 0);
    /// The hidden previews show title flag.
    pub const HIDDEN_PREVIEWS_SHOW_TITLE: Self = Self(1 << 2);
    /// The hidden previews show subtitle flag.
    pub const HIDDEN_PREVIEWS_SHOW_SUBTITLE: Self = Self(1 << 3);
}

/// Wraps `UNNotificationCategory`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationCategory {
    /// The identifier.
    pub identifier: String,
    /// The actions.
    pub actions: Vec<NotificationAction>,
    /// The intent identifiers.
    pub intent_identifiers: Vec<String>,
    /// The options.
    pub options: NotificationCategoryOptions,
    /// The hidden previews body placeholder.
    pub hidden_previews_body_placeholder: Option<String>,
    /// The category summary format.
    pub category_summary_format: Option<String>,
    /// The localized hidden previews body placeholder.
    pub localized_hidden_previews_body_placeholder: Option<LocalizedNotificationString>,
    /// The localized category summary format.
    pub localized_category_summary_format: Option<LocalizedNotificationString>,
}

impl NotificationCategory {
    /// Creates a new notification category.
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
            hidden_previews_body_placeholder: None,
            category_summary_format: None,
            localized_hidden_previews_body_placeholder: None,
            localized_category_summary_format: None,
        }
    }

    /// Sets hidden previews body placeholder.
    #[must_use]
    pub fn with_hidden_previews_body_placeholder(
        mut self,
        hidden_previews_body_placeholder: impl Into<String>,
    ) -> Self {
        self.hidden_previews_body_placeholder = Some(hidden_previews_body_placeholder.into());
        self
    }

    /// Sets category summary format.
    #[must_use]
    pub fn with_category_summary_format(
        mut self,
        category_summary_format: impl Into<String>,
    ) -> Self {
        self.category_summary_format = Some(category_summary_format.into());
        self
    }

    /// Sets localized hidden previews body placeholder.
    #[must_use]
    pub fn with_localized_hidden_previews_body_placeholder(
        mut self,
        localized_hidden_previews_body_placeholder: LocalizedNotificationString,
    ) -> Self {
        self.localized_hidden_previews_body_placeholder =
            Some(localized_hidden_previews_body_placeholder);
        self
    }

    /// Sets localized category summary format.
    #[must_use]
    pub fn with_localized_category_summary_format(
        mut self,
        localized_category_summary_format: LocalizedNotificationString,
    ) -> Self {
        self.localized_category_summary_format = Some(localized_category_summary_format);
        self
    }

    /// Round-trips this value through the Swift bridge.
    pub fn bridge_roundtrip(&self) -> Result<Self, UserNotificationsError> {
        let category = encode_category_json(self)?;
        let category = to_cstring(&category)?;
        let mut error = core::ptr::null_mut();
        let payload =
            unsafe { ffi::category::un_category_roundtrip_json(category.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_category_json(payload)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NotificationCategoryPayload {
    identifier: String,
    actions: Vec<NotificationActionPayload>,
    intent_identifiers: Vec<String>,
    options: u64,
    hidden_previews_body_placeholder: Option<String>,
    category_summary_format: Option<String>,
    localized_hidden_previews_body_placeholder: Option<LocalizedNotificationString>,
    localized_category_summary_format: Option<LocalizedNotificationString>,
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
            hidden_previews_body_placeholder: value.hidden_previews_body_placeholder.clone(),
            category_summary_format: value.category_summary_format.clone(),
            localized_hidden_previews_body_placeholder: value
                .localized_hidden_previews_body_placeholder
                .clone(),
            localized_category_summary_format: value.localized_category_summary_format.clone(),
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
            hidden_previews_body_placeholder: value.hidden_previews_body_placeholder,
            category_summary_format: value.category_summary_format,
            localized_hidden_previews_body_placeholder: value
                .localized_hidden_previews_body_placeholder,
            localized_category_summary_format: value.localized_category_summary_format,
        }
    }
}

pub(crate) fn encode_category_json(
    category: &NotificationCategory,
) -> Result<String, UserNotificationsError> {
    serde_json::to_string(&NotificationCategoryPayload::from(category)).map_err(|error| {
        UserNotificationsError::FrameworkError(format!(
            "failed to encode notification category: {error}",
        ))
    })
}

pub(crate) fn decode_category_json(
    ptr: *mut c_char,
) -> Result<NotificationCategory, UserNotificationsError> {
    decode_json::<NotificationCategoryPayload>(ptr).map(Into::into)
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
            "failed to encode notification categories: {error}",
        ))
    })
}

pub(crate) fn decode_categories_json(
    ptr: *mut c_char,
) -> Result<Vec<NotificationCategory>, UserNotificationsError> {
    decode_json::<Vec<NotificationCategoryPayload>>(ptr)
        .map(|payloads| payloads.into_iter().map(Into::into).collect())
}
