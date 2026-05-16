use core::ffi::c_char;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::attachment::{NotificationAttachment, NotificationAttachmentPayload};
use crate::error::{from_swift, UserNotificationsError};
use crate::ffi;
use crate::private::{decode_json, to_cstring};

mod content_provider_private {
    pub trait Sealed {}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedNotificationString {
    pub key: String,
    pub arguments: Vec<Value>,
}

impl LocalizedNotificationString {
    #[must_use]
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            arguments: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_argument(mut self, argument: impl Into<Value>) -> Self {
        self.arguments.push(argument.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NotificationInterruptionLevel {
    Passive = 0,
    Active = 1,
    TimeSensitive = 2,
    Critical = 3,
}

impl NotificationInterruptionLevel {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::Passive,
            2 => Self::TimeSensitive,
            3 => Self::Critical,
            _ => Self::Active,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationSound {
    Default,
    DefaultCritical,
    DefaultCriticalWithVolume(f32),
    Named(String),
    CriticalNamed { name: String, volume: Option<f32> },
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum NotificationMessagePersonHandleType {
    Unknown = 0,
    EmailAddress = 1,
    PhoneNumber = 2,
}

impl NotificationMessagePersonHandleType {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::EmailAddress,
            2 => Self::PhoneNumber,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum NotificationMessageType {
    Unknown = 0,
    Text = 1,
    Audio = 2,
}

impl NotificationMessageType {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Text,
            2 => Self::Audio,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationMessagePerson {
    pub handle: String,
    pub handle_type: NotificationMessagePersonHandleType,
    pub display_name: Option<String>,
    pub contact_identifier: Option<String>,
    pub custom_identifier: Option<String>,
    pub is_me: bool,
}

impl NotificationMessagePerson {
    #[must_use]
    pub fn new(
        handle: impl Into<String>,
        handle_type: NotificationMessagePersonHandleType,
    ) -> Self {
        Self {
            handle: handle.into(),
            handle_type,
            display_name: None,
            contact_identifier: None,
            custom_identifier: None,
            is_me: false,
        }
    }

    #[must_use]
    pub fn with_display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = Some(display_name.into());
        self
    }

    #[must_use]
    pub fn with_contact_identifier(mut self, contact_identifier: impl Into<String>) -> Self {
        self.contact_identifier = Some(contact_identifier.into());
        self
    }

    #[must_use]
    pub fn with_custom_identifier(mut self, custom_identifier: impl Into<String>) -> Self {
        self.custom_identifier = Some(custom_identifier.into());
        self
    }

    #[must_use]
    pub const fn with_is_me(mut self, is_me: bool) -> Self {
        self.is_me = is_me;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationAttributedMessageContext {
    pub sender: Option<NotificationMessagePerson>,
    pub recipients: Vec<NotificationMessagePerson>,
    pub attributed_content: String,
    pub content: Option<String>,
    pub outgoing_message_type: NotificationMessageType,
    pub conversation_identifier: Option<String>,
    pub service_name: Option<String>,
    pub group_name: Option<String>,
}

impl NotificationAttributedMessageContext {
    #[must_use]
    pub fn new(attributed_content: impl Into<String>) -> Self {
        Self {
            sender: None,
            recipients: Vec::new(),
            attributed_content: attributed_content.into(),
            content: None,
            outgoing_message_type: NotificationMessageType::Text,
            conversation_identifier: None,
            service_name: None,
            group_name: None,
        }
    }

    #[must_use]
    pub fn with_sender(mut self, sender: NotificationMessagePerson) -> Self {
        self.sender = Some(sender);
        self
    }

    #[must_use]
    pub fn with_recipient(mut self, recipient: NotificationMessagePerson) -> Self {
        self.recipients.push(recipient);
        self
    }

    #[must_use]
    pub fn with_recipients(mut self, recipients: Vec<NotificationMessagePerson>) -> Self {
        self.recipients = recipients;
        self
    }

    #[must_use]
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    #[must_use]
    pub const fn with_outgoing_message_type(
        mut self,
        outgoing_message_type: NotificationMessageType,
    ) -> Self {
        self.outgoing_message_type = outgoing_message_type;
        self
    }

    #[must_use]
    pub fn with_conversation_identifier(
        mut self,
        conversation_identifier: impl Into<String>,
    ) -> Self {
        self.conversation_identifier = Some(conversation_identifier.into());
        self
    }

    #[must_use]
    pub fn with_service_name(mut self, service_name: impl Into<String>) -> Self {
        self.service_name = Some(service_name.into());
        self
    }

    #[must_use]
    pub fn with_group_name(mut self, group_name: impl Into<String>) -> Self {
        self.group_name = Some(group_name.into());
        self
    }
}

pub trait NotificationContentProviding: content_provider_private::Sealed {
    #[doc(hidden)]
    fn encode_provider_json(&self) -> Result<String, UserNotificationsError>;
}

impl content_provider_private::Sealed for NotificationAttributedMessageContext {}

impl NotificationContentProviding for NotificationAttributedMessageContext {
    fn encode_provider_json(&self) -> Result<String, UserNotificationsError> {
        encode_content_provider_json(&NotificationContentProviderPayload::from(self))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NotificationContent {
    pub title: String,
    pub subtitle: String,
    pub body: String,
    pub badge: Option<i64>,
    pub category_identifier: String,
    pub thread_identifier: String,
    pub user_info: Option<Value>,
    pub sound: Option<NotificationSound>,
    pub attachments: Vec<NotificationAttachment>,
    pub summary_argument: String,
    pub summary_argument_count: u64,
    pub interruption_level: Option<NotificationInterruptionLevel>,
    pub relevance_score: Option<f64>,
    pub filter_criteria: Option<String>,
    pub localized_title: Option<LocalizedNotificationString>,
    pub localized_subtitle: Option<LocalizedNotificationString>,
    pub localized_body: Option<LocalizedNotificationString>,
    pub localized_summary_argument: Option<LocalizedNotificationString>,
}

impl NotificationContent {
    #[must_use]
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            summary_argument_count: 1,
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

    #[must_use]
    pub fn with_attachment(mut self, attachment: NotificationAttachment) -> Self {
        self.attachments.push(attachment);
        self
    }

    #[must_use]
    pub fn with_attachments(mut self, attachments: Vec<NotificationAttachment>) -> Self {
        self.attachments = attachments;
        self
    }

    #[must_use]
    pub fn with_summary_argument(mut self, summary_argument: impl Into<String>) -> Self {
        self.summary_argument = summary_argument.into();
        self
    }

    #[must_use]
    pub fn with_summary_argument_count(mut self, summary_argument_count: u64) -> Self {
        self.summary_argument_count = summary_argument_count;
        self
    }

    #[must_use]
    pub fn with_interruption_level(
        mut self,
        interruption_level: NotificationInterruptionLevel,
    ) -> Self {
        self.interruption_level = Some(interruption_level);
        self
    }

    #[must_use]
    pub fn with_relevance_score(mut self, relevance_score: f64) -> Self {
        self.relevance_score = Some(relevance_score);
        self
    }

    #[must_use]
    pub fn with_filter_criteria(mut self, filter_criteria: impl Into<String>) -> Self {
        self.filter_criteria = Some(filter_criteria.into());
        self
    }

    #[must_use]
    pub fn with_localized_title(mut self, localized_title: LocalizedNotificationString) -> Self {
        self.localized_title = Some(localized_title);
        self
    }

    #[must_use]
    pub fn with_localized_subtitle(
        mut self,
        localized_subtitle: LocalizedNotificationString,
    ) -> Self {
        self.localized_subtitle = Some(localized_subtitle);
        self
    }

    #[must_use]
    pub fn with_localized_body(mut self, localized_body: LocalizedNotificationString) -> Self {
        self.localized_body = Some(localized_body);
        self
    }

    #[must_use]
    pub fn with_localized_summary_argument(
        mut self,
        localized_summary_argument: LocalizedNotificationString,
    ) -> Self {
        self.localized_summary_argument = Some(localized_summary_argument);
        self
    }

    pub fn bridge_roundtrip(&self) -> Result<Self, UserNotificationsError> {
        let content = encode_content_json(self)?;
        let content = to_cstring(&content)?;
        let mut error = core::ptr::null_mut();
        let payload =
            unsafe { ffi::content::un_content_roundtrip_json(content.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_content_json(payload)
        }
    }

    pub fn updating_from<P: NotificationContentProviding>(
        &self,
        provider: &P,
    ) -> Result<Self, UserNotificationsError> {
        let content = encode_content_json(self)?;
        let provider = provider.encode_provider_json()?;
        let content = to_cstring(&content)?;
        let provider = to_cstring(&provider)?;
        let mut error = core::ptr::null_mut();
        let payload = unsafe {
            ffi::content::un_content_updating_with_provider_json(
                content.as_ptr(),
                provider.as_ptr(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_content_json(payload)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NotificationSoundPayload {
    kind: String,
    name: Option<String>,
    volume: Option<f32>,
}

impl From<&NotificationSound> for NotificationSoundPayload {
    fn from(value: &NotificationSound) -> Self {
        match value {
            NotificationSound::Default => Self {
                kind: "default".into(),
                name: None,
                volume: None,
            },
            NotificationSound::DefaultCritical => Self {
                kind: "defaultCritical".into(),
                name: None,
                volume: None,
            },
            NotificationSound::DefaultCriticalWithVolume(volume) => Self {
                kind: "defaultCriticalWithVolume".into(),
                name: None,
                volume: Some(*volume),
            },
            NotificationSound::Named(name) => Self {
                kind: "named".into(),
                name: Some(name.clone()),
                volume: None,
            },
            NotificationSound::CriticalNamed { name, volume } => Self {
                kind: "criticalNamed".into(),
                name: Some(name.clone()),
                volume: *volume,
            },
            NotificationSound::Unknown => Self {
                kind: "unknown".into(),
                name: None,
                volume: None,
            },
        }
    }
}

impl From<NotificationSoundPayload> for NotificationSound {
    fn from(value: NotificationSoundPayload) -> Self {
        match value.kind.as_str() {
            "default" => Self::Default,
            "defaultCritical" => Self::DefaultCritical,
            "defaultCriticalWithVolume" => value
                .volume
                .map_or(Self::Unknown, Self::DefaultCriticalWithVolume),
            "named" => value.name.map_or(Self::Unknown, Self::Named),
            "criticalNamed" => value
                .name
                .map_or(Self::Unknown, |name| Self::CriticalNamed {
                    name,
                    volume: value.volume,
                }),
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NotificationContentPayload {
    title: String,
    subtitle: String,
    body: String,
    badge: Option<i64>,
    category_identifier: String,
    thread_identifier: String,
    user_info: Option<Value>,
    sound: Option<NotificationSoundPayload>,
    attachments: Vec<NotificationAttachmentPayload>,
    summary_argument: String,
    summary_argument_count: u64,
    interruption_level: Option<i32>,
    relevance_score: Option<f64>,
    filter_criteria: Option<String>,
    localized_title: Option<LocalizedNotificationString>,
    localized_subtitle: Option<LocalizedNotificationString>,
    localized_body: Option<LocalizedNotificationString>,
    localized_summary_argument: Option<LocalizedNotificationString>,
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
            attachments: value
                .attachments
                .iter()
                .map(NotificationAttachmentPayload::from)
                .collect(),
            summary_argument: value.summary_argument.clone(),
            summary_argument_count: value.summary_argument_count,
            interruption_level: value.interruption_level.map(|level| level as i32),
            relevance_score: value.relevance_score,
            filter_criteria: value.filter_criteria.clone(),
            localized_title: value.localized_title.clone(),
            localized_subtitle: value.localized_subtitle.clone(),
            localized_body: value.localized_body.clone(),
            localized_summary_argument: value.localized_summary_argument.clone(),
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
            attachments: value.attachments.into_iter().map(Into::into).collect(),
            summary_argument: value.summary_argument,
            summary_argument_count: value.summary_argument_count,
            interruption_level: value
                .interruption_level
                .map(NotificationInterruptionLevel::from_raw),
            relevance_score: value.relevance_score,
            filter_criteria: value.filter_criteria,
            localized_title: value.localized_title,
            localized_subtitle: value.localized_subtitle,
            localized_body: value.localized_body,
            localized_summary_argument: value.localized_summary_argument,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NotificationMessagePersonPayload {
    handle: String,
    handle_type: i32,
    display_name: Option<String>,
    contact_identifier: Option<String>,
    custom_identifier: Option<String>,
    is_me: bool,
}

impl From<&NotificationMessagePerson> for NotificationMessagePersonPayload {
    fn from(value: &NotificationMessagePerson) -> Self {
        Self {
            handle: value.handle.clone(),
            handle_type: value.handle_type as i32,
            display_name: value.display_name.clone(),
            contact_identifier: value.contact_identifier.clone(),
            custom_identifier: value.custom_identifier.clone(),
            is_me: value.is_me,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NotificationAttributedMessageContextPayload {
    sender: Option<NotificationMessagePersonPayload>,
    recipients: Vec<NotificationMessagePersonPayload>,
    attributed_content: String,
    content: Option<String>,
    outgoing_message_type: i32,
    conversation_identifier: Option<String>,
    service_name: Option<String>,
    group_name: Option<String>,
}

impl From<&NotificationAttributedMessageContext> for NotificationAttributedMessageContextPayload {
    fn from(value: &NotificationAttributedMessageContext) -> Self {
        Self {
            sender: value
                .sender
                .as_ref()
                .map(NotificationMessagePersonPayload::from),
            recipients: value
                .recipients
                .iter()
                .map(NotificationMessagePersonPayload::from)
                .collect(),
            attributed_content: value.attributed_content.clone(),
            content: value.content.clone(),
            outgoing_message_type: value.outgoing_message_type as i32,
            conversation_identifier: value.conversation_identifier.clone(),
            service_name: value.service_name.clone(),
            group_name: value.group_name.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NotificationContentProviderPayload {
    kind: String,
    attributed_message_context: Option<NotificationAttributedMessageContextPayload>,
}

impl From<&NotificationAttributedMessageContext> for NotificationContentProviderPayload {
    fn from(value: &NotificationAttributedMessageContext) -> Self {
        Self {
            kind: "attributedMessageContext".into(),
            attributed_message_context: Some(NotificationAttributedMessageContextPayload::from(
                value,
            )),
        }
    }
}

fn encode_content_provider_json(
    provider: &NotificationContentProviderPayload,
) -> Result<String, UserNotificationsError> {
    serde_json::to_string(provider).map_err(|error| {
        UserNotificationsError::FrameworkError(format!(
            "failed to encode notification content provider: {error}",
        ))
    })
}

pub(crate) fn encode_content_json(
    content: &NotificationContent,
) -> Result<String, UserNotificationsError> {
    serde_json::to_string(&NotificationContentPayload::from(content)).map_err(|error| {
        UserNotificationsError::FrameworkError(format!(
            "failed to encode notification content: {error}",
        ))
    })
}

pub(crate) fn decode_content_json(
    ptr: *mut c_char,
) -> Result<NotificationContent, UserNotificationsError> {
    decode_json::<NotificationContentPayload>(ptr).map(Into::into)
}
