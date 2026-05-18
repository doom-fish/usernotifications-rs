use core::ffi::c_char;

use serde::{Deserialize, Serialize};

use crate::content::{NotificationContent, NotificationContentPayload};
use crate::error::{from_swift, UserNotificationsError};
use crate::ffi;
use crate::private::{decode_json, to_cstring};
use crate::trigger::{NotificationTrigger, NotificationTriggerPayload};

/// Wraps `UNNotificationRequest`.
#[derive(Debug, Clone, PartialEq)]
pub struct NotificationRequest {
    /// The identifier.
    pub identifier: String,
    /// The content.
    pub content: NotificationContent,
    /// The trigger.
    pub trigger: Option<NotificationTrigger>,
}

impl NotificationRequest {
    /// Creates a new notification request.
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

    /// Round-trips this value through the Swift bridge.
    pub fn bridge_roundtrip(&self) -> Result<Self, UserNotificationsError> {
        let request = encode_request_json(self)?;
        let request = to_cstring(&request)?;
        let mut error = core::ptr::null_mut();
        let payload =
            unsafe { ffi::request::un_request_roundtrip_json(request.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_request_json(payload)
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

pub(crate) fn encode_request_json(
    request: &NotificationRequest,
) -> Result<String, UserNotificationsError> {
    serde_json::to_string(&NotificationRequestPayload::from(request)).map_err(|error| {
        UserNotificationsError::FrameworkError(format!(
            "failed to encode notification request: {error}",
        ))
    })
}

pub(crate) fn decode_request_json(
    ptr: *mut c_char,
) -> Result<NotificationRequest, UserNotificationsError> {
    decode_json::<NotificationRequestPayload>(ptr).map(Into::into)
}

pub(crate) fn decode_requests_json(
    ptr: *mut c_char,
) -> Result<Vec<NotificationRequest>, UserNotificationsError> {
    decode_json::<Vec<NotificationRequestPayload>>(ptr)
        .map(|payloads| payloads.into_iter().map(Into::into).collect())
}
