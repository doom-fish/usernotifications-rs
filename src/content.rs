use crate::{NotificationUserInfo, NotificationValue};

use crate::attachment::{NotificationAttachment, NotificationAttachmentPayload};
use crate::binary::decode_binary;
use crate::error::{from_swift, UserNotificationsError};
use crate::ffi;

mod content_provider_private {
    pub trait Sealed {}
}

/// Wraps localized string inputs consumed by `UNMutableNotificationContent` text properties.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalizedNotificationString {
    /// The key.
    pub key: String,
    /// The arguments.
    pub arguments: Vec<NotificationValue>,
}

impl LocalizedNotificationString {
    /// Creates a localized notification string.
    #[must_use]
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            arguments: Vec::new(),
        }
    }

    /// Adds a localization argument.
    #[must_use]
    pub fn with_argument(mut self, argument: impl Into<NotificationValue>) -> Self {
        self.arguments.push(argument.into());
        self
    }
}

/// Wraps the `UNNotificationInterruptionLevel` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NotificationInterruptionLevel {
    /// Delivers the notification passively.
    Passive = 0,
    /// Delivers the notification with the default active behavior.
    Active = 1,
    /// Marks the notification as time sensitive.
    TimeSensitive = 2,
    /// Marks the notification as critical.
    Critical = 3,
}

impl NotificationInterruptionLevel {
    /// Converts a raw framework value into an interruption level.
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

/// Wraps `UNNotificationSound`.
#[derive(Debug, Clone, PartialEq)]
pub enum NotificationSound {
    /// Uses the default notification sound.
    Default,
    /// Uses the default critical notification sound.
    DefaultCritical,
    /// Uses the default critical notification sound with an explicit volume.
    DefaultCriticalWithVolume(f32),
    /// Uses a custom sound resource name.
    Named(String),
    /// Uses a named critical sound.
    CriticalNamed {
        /// The custom sound file name.
        name: String,
        /// The optional playback volume.
        volume: Option<f32>,
    },
    /// Fallback for sound values the framework returns but this crate does not model yet.
    Unknown,
}

/// Wraps the handle-type values used by `UNNotificationAttributedMessageContext`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NotificationMessagePersonHandleType {
    /// Represents an unknown handle type.
    Unknown = 0,
    /// Uses an email-address handle.
    EmailAddress = 1,
    /// Uses a phone-number handle.
    PhoneNumber = 2,
}

impl NotificationMessagePersonHandleType {
    /// Converts a raw framework value into a person-handle type.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::EmailAddress,
            2 => Self::PhoneNumber,
            _ => Self::Unknown,
        }
    }
}

/// Wraps the outgoing message type used by `UNNotificationAttributedMessageContext`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NotificationMessageType {
    /// Represents an unknown outgoing message type.
    Unknown = 0,
    /// Represents a text message.
    Text = 1,
    /// Represents an audio message.
    Audio = 2,
}

impl NotificationMessageType {
    /// Converts a raw framework value into a message type.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Text,
            2 => Self::Audio,
            _ => Self::Unknown,
        }
    }
}

/// Wraps a person record used with `UNNotificationAttributedMessageContext`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationMessagePerson {
    /// The handle.
    pub handle: String,
    /// The handle type.
    pub handle_type: NotificationMessagePersonHandleType,
    /// The display name.
    pub display_name: Option<String>,
    /// The contact identifier.
    pub contact_identifier: Option<String>,
    /// The custom identifier.
    pub custom_identifier: Option<String>,
    /// Whether this person represents the current user.
    pub is_me: bool,
}

impl NotificationMessagePerson {
    /// Creates a notification message person.
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

    /// Sets display name.
    #[must_use]
    pub fn with_display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = Some(display_name.into());
        self
    }

    /// Sets contact identifier.
    #[must_use]
    pub fn with_contact_identifier(mut self, contact_identifier: impl Into<String>) -> Self {
        self.contact_identifier = Some(contact_identifier.into());
        self
    }

    /// Sets custom identifier.
    #[must_use]
    pub fn with_custom_identifier(mut self, custom_identifier: impl Into<String>) -> Self {
        self.custom_identifier = Some(custom_identifier.into());
        self
    }

    /// Sets whether this person represents the current user.
    #[must_use]
    pub const fn with_is_me(mut self, is_me: bool) -> Self {
        self.is_me = is_me;
        self
    }
}

/// Wraps `UNNotificationAttributedMessageContext`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationAttributedMessageContext {
    /// The sender.
    pub sender: Option<NotificationMessagePerson>,
    /// The recipients.
    pub recipients: Vec<NotificationMessagePerson>,
    /// The attributed content.
    pub attributed_content: String,
    /// The content.
    pub content: Option<String>,
    /// The outgoing message type.
    pub outgoing_message_type: NotificationMessageType,
    /// The conversation identifier.
    pub conversation_identifier: Option<String>,
    /// The service name.
    pub service_name: Option<String>,
    /// The group name.
    pub group_name: Option<String>,
}

impl NotificationAttributedMessageContext {
    /// Creates a notification attributed message context.
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

    /// Sets sender.
    #[must_use]
    pub fn with_sender(mut self, sender: NotificationMessagePerson) -> Self {
        self.sender = Some(sender);
        self
    }

    /// Sets recipient.
    #[must_use]
    pub fn with_recipient(mut self, recipient: NotificationMessagePerson) -> Self {
        self.recipients.push(recipient);
        self
    }

    /// Sets recipients.
    #[must_use]
    pub fn with_recipients(mut self, recipients: Vec<NotificationMessagePerson>) -> Self {
        self.recipients = recipients;
        self
    }

    /// Sets content.
    #[must_use]
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Sets outgoing message type.
    #[must_use]
    pub const fn with_outgoing_message_type(
        mut self,
        outgoing_message_type: NotificationMessageType,
    ) -> Self {
        self.outgoing_message_type = outgoing_message_type;
        self
    }

    /// Sets conversation identifier.
    #[must_use]
    pub fn with_conversation_identifier(
        mut self,
        conversation_identifier: impl Into<String>,
    ) -> Self {
        self.conversation_identifier = Some(conversation_identifier.into());
        self
    }

    /// Sets service name.
    #[must_use]
    pub fn with_service_name(mut self, service_name: impl Into<String>) -> Self {
        self.service_name = Some(service_name.into());
        self
    }

    /// Sets group name.
    #[must_use]
    pub fn with_group_name(mut self, group_name: impl Into<String>) -> Self {
        self.group_name = Some(group_name.into());
        self
    }
}

/// Sealed trait for values accepted by `UNMutableNotificationContent::updating(from:)`.
pub trait NotificationContentProviding: content_provider_private::Sealed {
    #[doc(hidden)]
    fn attributed_message_context(&self) -> &NotificationAttributedMessageContext;
}

impl content_provider_private::Sealed for NotificationAttributedMessageContext {}

impl NotificationContentProviding for NotificationAttributedMessageContext {
    fn attributed_message_context(&self) -> &NotificationAttributedMessageContext {
        self
    }
}

/// Wraps `UNMutableNotificationContent` and decoded `UNNotificationContent` values.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NotificationContent {
    /// The title.
    pub title: String,
    /// The subtitle.
    pub subtitle: String,
    /// The body.
    pub body: String,
    /// The badge.
    pub badge: Option<i64>,
    /// The category identifier.
    pub category_identifier: String,
    /// The thread identifier.
    pub thread_identifier: String,
    /// The user info.
    pub user_info: Option<NotificationUserInfo>,
    /// The sound.
    pub sound: Option<NotificationSound>,
    /// The attachments.
    pub attachments: Vec<NotificationAttachment>,
    /// The summary argument.
    pub summary_argument: String,
    /// The summary argument count.
    pub summary_argument_count: u64,
    /// The interruption level.
    pub interruption_level: Option<NotificationInterruptionLevel>,
    /// The relevance score.
    pub relevance_score: Option<f64>,
    /// The filter criteria.
    pub filter_criteria: Option<String>,
    /// The localized title.
    pub localized_title: Option<LocalizedNotificationString>,
    /// The localized subtitle.
    pub localized_subtitle: Option<LocalizedNotificationString>,
    /// The localized body.
    pub localized_body: Option<LocalizedNotificationString>,
    /// The localized summary argument.
    pub localized_summary_argument: Option<LocalizedNotificationString>,
}

impl NotificationContent {
    /// Creates mutable notification content with a title and body.
    #[must_use]
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            summary_argument_count: 1,
            ..Self::default()
        }
    }

    /// Sets subtitle.
    #[must_use]
    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = subtitle.into();
        self
    }

    /// Sets badge.
    #[must_use]
    pub fn with_badge(mut self, badge: i64) -> Self {
        self.badge = Some(badge);
        self
    }

    /// Sets category identifier.
    #[must_use]
    pub fn with_category_identifier(mut self, category_identifier: impl Into<String>) -> Self {
        self.category_identifier = category_identifier.into();
        self
    }

    /// Sets thread identifier.
    #[must_use]
    pub fn with_thread_identifier(mut self, thread_identifier: impl Into<String>) -> Self {
        self.thread_identifier = thread_identifier.into();
        self
    }

    /// Sets user info.
    #[must_use]
    pub fn with_user_info(mut self, user_info: NotificationUserInfo) -> Self {
        self.user_info = Some(user_info);
        self
    }

    /// Sets sound.
    #[must_use]
    pub fn with_sound(mut self, sound: NotificationSound) -> Self {
        self.sound = Some(sound);
        self
    }

    /// Sets attachment.
    #[must_use]
    pub fn with_attachment(mut self, attachment: NotificationAttachment) -> Self {
        self.attachments.push(attachment);
        self
    }

    /// Sets attachments.
    #[must_use]
    pub fn with_attachments(mut self, attachments: Vec<NotificationAttachment>) -> Self {
        self.attachments = attachments;
        self
    }

    /// Sets summary argument.
    #[must_use]
    pub fn with_summary_argument(mut self, summary_argument: impl Into<String>) -> Self {
        self.summary_argument = summary_argument.into();
        self
    }

    /// Sets summary argument count.
    #[must_use]
    pub fn with_summary_argument_count(mut self, summary_argument_count: u64) -> Self {
        self.summary_argument_count = summary_argument_count;
        self
    }

    /// Sets interruption level.
    #[must_use]
    pub fn with_interruption_level(
        mut self,
        interruption_level: NotificationInterruptionLevel,
    ) -> Self {
        self.interruption_level = Some(interruption_level);
        self
    }

    /// Sets relevance score.
    #[must_use]
    pub fn with_relevance_score(mut self, relevance_score: f64) -> Self {
        self.relevance_score = Some(relevance_score);
        self
    }

    /// Sets filter criteria.
    #[must_use]
    pub fn with_filter_criteria(mut self, filter_criteria: impl Into<String>) -> Self {
        self.filter_criteria = Some(filter_criteria.into());
        self
    }

    /// Sets localized title.
    #[must_use]
    pub fn with_localized_title(mut self, localized_title: LocalizedNotificationString) -> Self {
        self.localized_title = Some(localized_title);
        self
    }

    /// Sets localized subtitle.
    #[must_use]
    pub fn with_localized_subtitle(
        mut self,
        localized_subtitle: LocalizedNotificationString,
    ) -> Self {
        self.localized_subtitle = Some(localized_subtitle);
        self
    }

    /// Sets localized body.
    #[must_use]
    pub fn with_localized_body(mut self, localized_body: LocalizedNotificationString) -> Self {
        self.localized_body = Some(localized_body);
        self
    }

    /// Sets localized summary argument.
    #[must_use]
    pub fn with_localized_summary_argument(
        mut self,
        localized_summary_argument: LocalizedNotificationString,
    ) -> Self {
        self.localized_summary_argument = Some(localized_summary_argument);
        self
    }

    /// Round-trips this content through the Swift bridge.
    pub fn bridge_roundtrip(&self) -> Result<Self, UserNotificationsError> {
        let content = encode_content_binary(self);

        let mut error = core::ptr::null_mut();
        let payload =
            unsafe { ffi::content::un_content_roundtrip_binary(content.as_ptr(), &raw mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_content_binary(payload)
        }
    }

    /// Applies a content provider using `UNMutableNotificationContent::updating(from:)`.
    pub fn updating_from<P: NotificationContentProviding>(
        &self,
        provider: &P,
    ) -> Result<Self, UserNotificationsError> {
        let content = encode_content_binary(self);
        let provider = crate::binary::Input::new(&NotificationContentProviderPayload::from(
            provider.attributed_message_context(),
        ));

        let mut error = core::ptr::null_mut();
        let payload = unsafe {
            ffi::content::un_content_updating_with_provider_binary(
                content.as_ptr(),
                provider.as_ptr(),
                &raw mut error,
            )
        };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_content_binary(payload)
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub(crate) struct NotificationContentPayload {
    title: String,
    subtitle: String,
    body: String,
    badge: Option<i64>,
    category_identifier: String,
    thread_identifier: String,
    user_info: Option<NotificationUserInfo>,
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

pub(crate) fn encode_content_binary(content: &NotificationContent) -> crate::binary::Input {
    crate::binary::Input::new(&NotificationContentPayload::from(content))
}

pub(crate) fn decode_content_binary(
    ptr: *mut crate::ffi::abi::UNBufferRaw,
) -> Result<NotificationContent, UserNotificationsError> {
    decode_binary::<NotificationContentPayload>(ptr).map(Into::into)
}

impl crate::binary::Record for LocalizedNotificationString {
    type Raw = crate::ffi::abi::UNLocalizedNotificationStringRaw;
    fn raw(&self, arena: &mut crate::binary::Arena) -> Self::Raw {
        Self::Raw {
            key: crate::binary::Record::raw(&self.key, arena),
            arguments: crate::binary::Record::raw(&self.arguments, arena),
        }
    }
    unsafe fn read(raw: &Self::Raw) -> Result<Self, UserNotificationsError> {
        Ok(Self {
            key: unsafe { <String as crate::binary::Record>::read(&raw.key) }?,
            arguments: unsafe {
                <Vec<NotificationValue> as crate::binary::Record>::read(&raw.arguments)
            }?,
        })
    }
}

impl crate::binary::Record for NotificationSoundPayload {
    type Raw = crate::ffi::abi::UNNotificationSoundPayloadRaw;
    fn raw(&self, arena: &mut crate::binary::Arena) -> Self::Raw {
        Self::Raw {
            kind: match self.kind.as_str() {
                "default" => 1,
                "defaultCritical" => 2,
                "defaultCriticalWithVolume" => 3,
                "named" => 4,
                "criticalNamed" => 5,
                _ => 0,
            },
            name: self.name.as_ref().map_or(core::ptr::null(), |value| {
                let raw = crate::binary::Record::raw(value, arena);
                arena.keep(raw)
            }),
            volume: self.volume.as_ref().map_or(core::ptr::null(), |value| {
                let raw = *value;
                arena.keep(raw)
            }),
        }
    }
    unsafe fn read(raw: &Self::Raw) -> Result<Self, UserNotificationsError> {
        Ok(Self {
            kind: match raw.kind {
                0 => "unknown",
                1 => "default",
                2 => "defaultCritical",
                3 => "defaultCriticalWithVolume",
                4 => "named",
                5 => "criticalNamed",
                _ => return Err(crate::binary::invalid_data()),
            }
            .to_owned(),
            name: unsafe { raw.name.as_ref() }
                .map(|value| unsafe { <String as crate::binary::Record>::read(value) })
                .transpose()?,
            volume: unsafe { raw.volume.as_ref() }.copied(),
        })
    }
}

impl crate::binary::Record for NotificationContentPayload {
    type Raw = crate::ffi::abi::UNNotificationContentPayloadRaw;
    fn raw(&self, arena: &mut crate::binary::Arena) -> Self::Raw {
        Self::Raw {
            title: crate::binary::Record::raw(&self.title, arena),
            subtitle: crate::binary::Record::raw(&self.subtitle, arena),
            body: crate::binary::Record::raw(&self.body, arena),
            badge: self.badge.as_ref().map_or(core::ptr::null(), |value| {
                let raw = *value;
                arena.keep(raw)
            }),
            category_identifier: crate::binary::Record::raw(&self.category_identifier, arena),
            thread_identifier: crate::binary::Record::raw(&self.thread_identifier, arena),
            user_info: self.user_info.as_ref().map_or(core::ptr::null(), |value| {
                let raw = crate::binary::Record::raw(value, arena);
                arena.keep(raw)
            }),
            sound: self.sound.as_ref().map_or(core::ptr::null(), |value| {
                let raw = crate::binary::Record::raw(value, arena);
                arena.keep(raw)
            }),
            attachments: crate::binary::Record::raw(&self.attachments, arena),
            summary_argument: crate::binary::Record::raw(&self.summary_argument, arena),
            summary_argument_count: self.summary_argument_count,
            interruption_level: self.interruption_level.as_ref().map_or(
                core::ptr::null(),
                |value| {
                    let raw = *value;
                    arena.keep(raw)
                },
            ),
            relevance_score: self
                .relevance_score
                .as_ref()
                .map_or(core::ptr::null(), |value| {
                    let raw = *value;
                    arena.keep(raw)
                }),
            filter_criteria: self
                .filter_criteria
                .as_ref()
                .map_or(core::ptr::null(), |value| {
                    let raw = crate::binary::Record::raw(value, arena);
                    arena.keep(raw)
                }),
            localized_title: self
                .localized_title
                .as_ref()
                .map_or(core::ptr::null(), |value| {
                    let raw = crate::binary::Record::raw(value, arena);
                    arena.keep(raw)
                }),
            localized_subtitle: self.localized_subtitle.as_ref().map_or(
                core::ptr::null(),
                |value| {
                    let raw = crate::binary::Record::raw(value, arena);
                    arena.keep(raw)
                },
            ),
            localized_body: self
                .localized_body
                .as_ref()
                .map_or(core::ptr::null(), |value| {
                    let raw = crate::binary::Record::raw(value, arena);
                    arena.keep(raw)
                }),
            localized_summary_argument: self.localized_summary_argument.as_ref().map_or(
                core::ptr::null(),
                |value| {
                    let raw = crate::binary::Record::raw(value, arena);
                    arena.keep(raw)
                },
            ),
        }
    }
    unsafe fn read(raw: &Self::Raw) -> Result<Self, UserNotificationsError> {
        Ok(Self {
            title: unsafe { <String as crate::binary::Record>::read(&raw.title) }?,
            subtitle: unsafe { <String as crate::binary::Record>::read(&raw.subtitle) }?,
            body: unsafe { <String as crate::binary::Record>::read(&raw.body) }?,
            badge: unsafe { raw.badge.as_ref() }.copied(),
            category_identifier: unsafe {
                <String as crate::binary::Record>::read(&raw.category_identifier)
            }?,
            thread_identifier: unsafe {
                <String as crate::binary::Record>::read(&raw.thread_identifier)
            }?,
            user_info: unsafe { raw.user_info.as_ref() }
                .map(|value| unsafe {
                    <NotificationUserInfo as crate::binary::Record>::read(value)
                })
                .transpose()?,
            sound: unsafe { raw.sound.as_ref() }
                .map(|value| unsafe {
                    <NotificationSoundPayload as crate::binary::Record>::read(value)
                })
                .transpose()?,
            attachments: unsafe {
                <Vec<NotificationAttachmentPayload> as crate::binary::Record>::read(
                    &raw.attachments,
                )
            }?,
            summary_argument: unsafe {
                <String as crate::binary::Record>::read(&raw.summary_argument)
            }?,
            summary_argument_count: raw.summary_argument_count,
            interruption_level: unsafe { raw.interruption_level.as_ref() }.copied(),
            relevance_score: unsafe { raw.relevance_score.as_ref() }.copied(),
            filter_criteria: unsafe { raw.filter_criteria.as_ref() }
                .map(|value| unsafe { <String as crate::binary::Record>::read(value) })
                .transpose()?,
            localized_title: unsafe { raw.localized_title.as_ref() }
                .map(|value| unsafe {
                    <LocalizedNotificationString as crate::binary::Record>::read(value)
                })
                .transpose()?,
            localized_subtitle: unsafe { raw.localized_subtitle.as_ref() }
                .map(|value| unsafe {
                    <LocalizedNotificationString as crate::binary::Record>::read(value)
                })
                .transpose()?,
            localized_body: unsafe { raw.localized_body.as_ref() }
                .map(|value| unsafe {
                    <LocalizedNotificationString as crate::binary::Record>::read(value)
                })
                .transpose()?,
            localized_summary_argument: unsafe { raw.localized_summary_argument.as_ref() }
                .map(|value| unsafe {
                    <LocalizedNotificationString as crate::binary::Record>::read(value)
                })
                .transpose()?,
        })
    }
}

impl crate::binary::Record for NotificationMessagePersonPayload {
    type Raw = crate::ffi::abi::UNNotificationMessagePersonPayloadRaw;
    fn raw(&self, arena: &mut crate::binary::Arena) -> Self::Raw {
        Self::Raw {
            handle: crate::binary::Record::raw(&self.handle, arena),
            handle_type: self.handle_type,
            display_name: self
                .display_name
                .as_ref()
                .map_or(core::ptr::null(), |value| {
                    let raw = crate::binary::Record::raw(value, arena);
                    arena.keep(raw)
                }),
            contact_identifier: self.contact_identifier.as_ref().map_or(
                core::ptr::null(),
                |value| {
                    let raw = crate::binary::Record::raw(value, arena);
                    arena.keep(raw)
                },
            ),
            custom_identifier: self
                .custom_identifier
                .as_ref()
                .map_or(core::ptr::null(), |value| {
                    let raw = crate::binary::Record::raw(value, arena);
                    arena.keep(raw)
                }),
            is_me: u8::from(self.is_me),
        }
    }
    unsafe fn read(raw: &Self::Raw) -> Result<Self, UserNotificationsError> {
        Ok(Self {
            handle: unsafe { <String as crate::binary::Record>::read(&raw.handle) }?,
            handle_type: raw.handle_type,
            display_name: unsafe { raw.display_name.as_ref() }
                .map(|value| unsafe { <String as crate::binary::Record>::read(value) })
                .transpose()?,
            contact_identifier: unsafe { raw.contact_identifier.as_ref() }
                .map(|value| unsafe { <String as crate::binary::Record>::read(value) })
                .transpose()?,
            custom_identifier: unsafe { raw.custom_identifier.as_ref() }
                .map(|value| unsafe { <String as crate::binary::Record>::read(value) })
                .transpose()?,
            is_me: raw.is_me != 0,
        })
    }
}

impl crate::binary::Record for NotificationAttributedMessageContextPayload {
    type Raw = crate::ffi::abi::UNNotificationAttributedMessageContextPayloadRaw;
    fn raw(&self, arena: &mut crate::binary::Arena) -> Self::Raw {
        Self::Raw {
            sender: self.sender.as_ref().map_or(core::ptr::null(), |value| {
                let raw = crate::binary::Record::raw(value, arena);
                arena.keep(raw)
            }),
            recipients: crate::binary::Record::raw(&self.recipients, arena),
            attributed_content: crate::binary::Record::raw(&self.attributed_content, arena),
            content: self.content.as_ref().map_or(core::ptr::null(), |value| {
                let raw = crate::binary::Record::raw(value, arena);
                arena.keep(raw)
            }),
            outgoing_message_type: self.outgoing_message_type,
            conversation_identifier: self.conversation_identifier.as_ref().map_or(
                core::ptr::null(),
                |value| {
                    let raw = crate::binary::Record::raw(value, arena);
                    arena.keep(raw)
                },
            ),
            service_name: self
                .service_name
                .as_ref()
                .map_or(core::ptr::null(), |value| {
                    let raw = crate::binary::Record::raw(value, arena);
                    arena.keep(raw)
                }),
            group_name: self.group_name.as_ref().map_or(core::ptr::null(), |value| {
                let raw = crate::binary::Record::raw(value, arena);
                arena.keep(raw)
            }),
        }
    }
    unsafe fn read(raw: &Self::Raw) -> Result<Self, UserNotificationsError> {
        Ok(Self {
            sender: unsafe { raw.sender.as_ref() }
                .map(|value| unsafe {
                    <NotificationMessagePersonPayload as crate::binary::Record>::read(value)
                })
                .transpose()?,
            recipients: unsafe {
                <Vec<NotificationMessagePersonPayload> as crate::binary::Record>::read(
                    &raw.recipients,
                )
            }?,
            attributed_content: unsafe {
                <String as crate::binary::Record>::read(&raw.attributed_content)
            }?,
            content: unsafe { raw.content.as_ref() }
                .map(|value| unsafe { <String as crate::binary::Record>::read(value) })
                .transpose()?,
            outgoing_message_type: raw.outgoing_message_type,
            conversation_identifier: unsafe { raw.conversation_identifier.as_ref() }
                .map(|value| unsafe { <String as crate::binary::Record>::read(value) })
                .transpose()?,
            service_name: unsafe { raw.service_name.as_ref() }
                .map(|value| unsafe { <String as crate::binary::Record>::read(value) })
                .transpose()?,
            group_name: unsafe { raw.group_name.as_ref() }
                .map(|value| unsafe { <String as crate::binary::Record>::read(value) })
                .transpose()?,
        })
    }
}

impl crate::binary::Record for NotificationContentProviderPayload {
    type Raw = crate::ffi::abi::UNNotificationContentProviderPayloadRaw;
    fn raw(&self, arena: &mut crate::binary::Arena) -> Self::Raw {
        Self::Raw {
            kind: match self.kind.as_str() {
                "attributedMessageContext" => 1,
                _ => 0,
            },
            attributed_message_context: self.attributed_message_context.as_ref().map_or(
                core::ptr::null(),
                |value| {
                    let raw = crate::binary::Record::raw(value, arena);
                    arena.keep(raw)
                },
            ),
        }
    }
    unsafe fn read(raw: &Self::Raw) -> Result<Self, UserNotificationsError> {
        Ok(Self {
            kind: match raw.kind {
                0 => "unknown",
                1 => "attributedMessageContext",
                _ => return Err(crate::binary::invalid_data()),
            }
            .to_owned(),
            attributed_message_context: unsafe { raw.attributed_message_context.as_ref() }
                .map(|value| unsafe {
                    <NotificationAttributedMessageContextPayload as crate::binary::Record>::read(
                        value,
                    )
                })
                .transpose()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        LocalizedNotificationString, NotificationContent, NotificationContentPayload,
        NotificationInterruptionLevel, NotificationSound,
    };
    use crate::{NotificationUserInfo, NotificationValue};

    #[test]
    fn localized_notification_string_round_trip_preserves_key_and_arguments() {
        let localized = LocalizedNotificationString::new("TITLE_KEY")
            .with_argument("Alex")
            .with_argument(3);

        let encoded = crate::binary::Input::new(&localized);
        let roundtrip: LocalizedNotificationString =
            unsafe { crate::binary::read(encoded.as_ptr()) }.unwrap();

        assert_eq!(roundtrip, localized);
        assert_eq!(roundtrip.key, "TITLE_KEY");
        assert_eq!(
            roundtrip.arguments,
            vec![NotificationValue::from("Alex"), NotificationValue::from(3)]
        );
    }

    #[test]
    fn notification_content_new_sets_expected_defaults() {
        let content = NotificationContent::new("Title", "Body");

        assert_eq!(content.title, "Title");
        assert_eq!(content.body, "Body");
        assert_eq!(content.summary_argument_count, 1);
        assert_eq!(content.subtitle, "");
        assert_eq!(content.category_identifier, "");
        assert_eq!(content.thread_identifier, "");
        assert_eq!(content.summary_argument, "");
        assert!(content.badge.is_none());
        assert!(content.user_info.is_none());
        assert!(content.sound.is_none());
        assert_eq!(
            content.attachments,
            [] as [crate::attachment::NotificationAttachment; 0]
        );
        assert!(content.interruption_level.is_none());
        assert!(content.relevance_score.is_none());
        assert!(content.filter_criteria.is_none());
        assert!(content.localized_title.is_none());
        assert!(content.localized_subtitle.is_none());
        assert!(content.localized_body.is_none());
        assert!(content.localized_summary_argument.is_none());
    }

    #[test]
    fn notification_content_builder_sets_expected_fields() {
        let localized_title = LocalizedNotificationString::new("TITLE");
        let localized_subtitle = LocalizedNotificationString::new("SUBTITLE").with_argument("Team");
        let localized_body = LocalizedNotificationString::new("BODY").with_argument(2);
        let localized_summary_argument = LocalizedNotificationString::new("SUMMARY");

        let content = NotificationContent::new("Title", "Body")
            .with_subtitle("Subtitle")
            .with_badge(7)
            .with_category_identifier("messages")
            .with_thread_identifier("thread-1")
            .with_user_info(NotificationUserInfo::from([("unread".into(), 7.into())]))
            .with_sound(NotificationSound::Named("ding.aiff".into()))
            .with_summary_argument("Messages")
            .with_summary_argument_count(7)
            .with_interruption_level(NotificationInterruptionLevel::TimeSensitive)
            .with_relevance_score(0.5)
            .with_filter_criteria("conversation")
            .with_localized_title(localized_title.clone())
            .with_localized_subtitle(localized_subtitle.clone())
            .with_localized_body(localized_body.clone())
            .with_localized_summary_argument(localized_summary_argument.clone());

        assert_eq!(content.subtitle, "Subtitle");
        assert_eq!(content.badge, Some(7));
        assert_eq!(content.category_identifier, "messages");
        assert_eq!(content.thread_identifier, "thread-1");
        assert_eq!(
            content.user_info,
            Some(NotificationUserInfo::from([("unread".into(), 7.into())]))
        );
        assert_eq!(
            content.sound,
            Some(NotificationSound::Named("ding.aiff".into()))
        );
        assert_eq!(content.summary_argument, "Messages");
        assert_eq!(content.summary_argument_count, 7);
        assert_eq!(
            content.interruption_level,
            Some(NotificationInterruptionLevel::TimeSensitive),
        );
        assert!(matches!(
            content.relevance_score,
            Some(score) if (score - 0.5).abs() < f64::EPSILON,
        ));
        assert_eq!(content.filter_criteria.as_deref(), Some("conversation"));
        assert_eq!(content.localized_title, Some(localized_title));
        assert_eq!(content.localized_subtitle, Some(localized_subtitle));
        assert_eq!(content.localized_body, Some(localized_body));
        assert_eq!(
            content.localized_summary_argument,
            Some(localized_summary_argument),
        );
    }

    #[test]
    fn notification_content_payload_round_trip_preserves_values() {
        let content = NotificationContent::new("Title", "Body")
            .with_subtitle("Subtitle")
            .with_badge(3)
            .with_category_identifier("inbox")
            .with_thread_identifier("thread-42")
            .with_user_info(NotificationUserInfo::from([("id".into(), 42.into())]))
            .with_sound(NotificationSound::CriticalNamed {
                name: "alert.aiff".into(),
                volume: Some(0.75),
            })
            .with_summary_argument("Inbox")
            .with_summary_argument_count(3)
            .with_interruption_level(NotificationInterruptionLevel::Critical)
            .with_relevance_score(1.0)
            .with_filter_criteria("important")
            .with_localized_title(LocalizedNotificationString::new("TITLE").with_argument("Alex"))
            .with_localized_subtitle(LocalizedNotificationString::new("SUBTITLE"))
            .with_localized_body(LocalizedNotificationString::new("BODY").with_argument(1))
            .with_localized_summary_argument(LocalizedNotificationString::new("SUMMARY"));

        let roundtrip = NotificationContent::from(NotificationContentPayload::from(&content));

        assert_eq!(roundtrip, content);
    }
}
