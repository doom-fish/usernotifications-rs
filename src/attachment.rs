use core::ffi::c_char;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{from_swift, UserNotificationsError};
use crate::ffi;
use crate::private::{decode_json, to_cstring};

pub const ATTACHMENT_OPTIONS_TYPE_HINT_KEY: &str = "UNNotificationAttachmentOptionsTypeHintKey";
pub const ATTACHMENT_OPTIONS_THUMBNAIL_HIDDEN_KEY: &str =
    "UNNotificationAttachmentOptionsThumbnailHiddenKey";
pub const ATTACHMENT_OPTIONS_THUMBNAIL_CLIPPING_RECT_KEY: &str =
    "UNNotificationAttachmentOptionsThumbnailClippingRectKey";
pub const ATTACHMENT_OPTIONS_THUMBNAIL_TIME_KEY: &str =
    "UNNotificationAttachmentOptionsThumbnailTimeKey";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttachmentThumbnailClippingRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl AttachmentThumbnailClippingRect {
    #[must_use]
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttachmentThumbnailTime {
    Seconds(f64),
    Frame(u64),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NotificationAttachmentOptions {
    pub type_hint: Option<String>,
    pub thumbnail_hidden: bool,
    pub thumbnail_clipping_rect: Option<AttachmentThumbnailClippingRect>,
    pub thumbnail_time: Option<AttachmentThumbnailTime>,
}

impl NotificationAttachmentOptions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_type_hint(mut self, type_hint: impl Into<String>) -> Self {
        self.type_hint = Some(type_hint.into());
        self
    }

    #[must_use]
    pub fn with_thumbnail_hidden(mut self, thumbnail_hidden: bool) -> Self {
        self.thumbnail_hidden = thumbnail_hidden;
        self
    }

    #[must_use]
    pub fn with_thumbnail_clipping_rect(
        mut self,
        thumbnail_clipping_rect: AttachmentThumbnailClippingRect,
    ) -> Self {
        self.thumbnail_clipping_rect = Some(thumbnail_clipping_rect);
        self
    }

    #[must_use]
    pub fn with_thumbnail_time(mut self, thumbnail_time: AttachmentThumbnailTime) -> Self {
        self.thumbnail_time = Some(thumbnail_time);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NotificationAttachment {
    pub identifier: String,
    pub file_path: PathBuf,
    pub attachment_type: Option<String>,
    pub options: NotificationAttachmentOptions,
}

impl NotificationAttachment {
    pub fn from_file(
        identifier: impl Into<String>,
        file_path: impl AsRef<Path>,
    ) -> Result<Self, UserNotificationsError> {
        Self::from_file_with_options(identifier, file_path, NotificationAttachmentOptions::new())
    }

    pub fn from_file_with_options(
        identifier: impl Into<String>,
        file_path: impl AsRef<Path>,
        options: NotificationAttachmentOptions,
    ) -> Result<Self, UserNotificationsError> {
        let attachment = Self {
            identifier: identifier.into(),
            file_path: file_path.as_ref().to_path_buf(),
            attachment_type: None,
            options,
        };
        attachment.bridge_roundtrip()
    }

    #[must_use]
    pub fn file_path(&self) -> &Path {
        &self.file_path
    }

    pub fn bridge_roundtrip(&self) -> Result<Self, UserNotificationsError> {
        let attachment = encode_attachment_json(self)?;
        let attachment = to_cstring(&attachment)?;
        let mut error = core::ptr::null_mut();
        let payload = unsafe { ffi::attachment::un_attachment_roundtrip_json(attachment.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_attachment_json(payload)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AttachmentThumbnailClippingRectPayload {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl From<&AttachmentThumbnailClippingRect> for AttachmentThumbnailClippingRectPayload {
    fn from(value: &AttachmentThumbnailClippingRect) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
}

impl From<AttachmentThumbnailClippingRectPayload> for AttachmentThumbnailClippingRect {
    fn from(value: AttachmentThumbnailClippingRectPayload) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum AttachmentThumbnailTimePayload {
    Seconds(f64),
    Frame(u64),
}

impl From<&AttachmentThumbnailTime> for AttachmentThumbnailTimePayload {
    fn from(value: &AttachmentThumbnailTime) -> Self {
        match value {
            AttachmentThumbnailTime::Seconds(seconds) => Self::Seconds(*seconds),
            AttachmentThumbnailTime::Frame(frame) => Self::Frame(*frame),
        }
    }
}

impl From<AttachmentThumbnailTimePayload> for AttachmentThumbnailTime {
    fn from(value: AttachmentThumbnailTimePayload) -> Self {
        match value {
            AttachmentThumbnailTimePayload::Seconds(seconds) => Self::Seconds(seconds),
            AttachmentThumbnailTimePayload::Frame(frame) => Self::Frame(frame),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NotificationAttachmentOptionsPayload {
    type_hint: Option<String>,
    thumbnail_hidden: bool,
    thumbnail_clipping_rect: Option<AttachmentThumbnailClippingRectPayload>,
    thumbnail_time: Option<AttachmentThumbnailTimePayload>,
}

impl From<&NotificationAttachmentOptions> for NotificationAttachmentOptionsPayload {
    fn from(value: &NotificationAttachmentOptions) -> Self {
        Self {
            type_hint: value.type_hint.clone(),
            thumbnail_hidden: value.thumbnail_hidden,
            thumbnail_clipping_rect: value
                .thumbnail_clipping_rect
                .as_ref()
                .map(AttachmentThumbnailClippingRectPayload::from),
            thumbnail_time: value.thumbnail_time.as_ref().map(AttachmentThumbnailTimePayload::from),
        }
    }
}

impl From<NotificationAttachmentOptionsPayload> for NotificationAttachmentOptions {
    fn from(value: NotificationAttachmentOptionsPayload) -> Self {
        Self {
            type_hint: value.type_hint,
            thumbnail_hidden: value.thumbnail_hidden,
            thumbnail_clipping_rect: value.thumbnail_clipping_rect.map(Into::into),
            thumbnail_time: value.thumbnail_time.map(Into::into),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NotificationAttachmentPayload {
    identifier: String,
    file_path: String,
    attachment_type: Option<String>,
    options: NotificationAttachmentOptionsPayload,
}

impl From<&NotificationAttachment> for NotificationAttachmentPayload {
    fn from(value: &NotificationAttachment) -> Self {
        Self {
            identifier: value.identifier.clone(),
            file_path: value.file_path.to_string_lossy().into_owned(),
            attachment_type: value.attachment_type.clone(),
            options: NotificationAttachmentOptionsPayload::from(&value.options),
        }
    }
}

impl From<NotificationAttachmentPayload> for NotificationAttachment {
    fn from(value: NotificationAttachmentPayload) -> Self {
        Self {
            identifier: value.identifier,
            file_path: PathBuf::from(value.file_path),
            attachment_type: value.attachment_type,
            options: value.options.into(),
        }
    }
}

pub(crate) fn encode_attachment_json(
    attachment: &NotificationAttachment,
) -> Result<String, UserNotificationsError> {
    serde_json::to_string(&NotificationAttachmentPayload::from(attachment)).map_err(|error| {
        UserNotificationsError::FrameworkError(format!(
            "failed to encode notification attachment: {error}",
        ))
    })
}

pub(crate) fn decode_attachment_json(
    ptr: *mut c_char,
) -> Result<NotificationAttachment, UserNotificationsError> {
    decode_json::<NotificationAttachmentPayload>(ptr).map(Into::into)
}
