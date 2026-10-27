use crate::binary::decode_binary;
use crate::content::{NotificationContent, NotificationContentPayload};
use crate::error::{from_swift, UserNotificationsError};
use crate::ffi;
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
        let request = encode_request_binary(self)?;

        let mut error = core::ptr::null_mut();
        let payload =
            unsafe { ffi::request::un_request_roundtrip_binary(request.as_ptr(), &raw mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_request_binary(payload)
        }
    }
}

#[derive(Debug, Clone)]
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

pub(crate) fn encode_request_binary(
    request: &NotificationRequest,
) -> Result<crate::binary::Input, UserNotificationsError> {
    if let Some(trigger) = &request.trigger {
        trigger.validate()?;
    }
    Ok(crate::binary::Input::new(
        &NotificationRequestPayload::from(request),
    ))
}

pub(crate) fn decode_request_binary(
    ptr: *mut crate::ffi::abi::UNBufferRaw,
) -> Result<NotificationRequest, UserNotificationsError> {
    decode_binary::<NotificationRequestPayload>(ptr).map(Into::into)
}

pub(crate) fn decode_requests_binary(
    ptr: *mut crate::ffi::abi::UNBufferRaw,
) -> Result<Vec<NotificationRequest>, UserNotificationsError> {
    decode_binary::<Vec<NotificationRequestPayload>>(ptr)
        .map(|payloads| payloads.into_iter().map(Into::into).collect())
}

impl crate::binary::Record for NotificationRequestPayload {
    type Raw = crate::ffi::abi::UNNotificationRequestPayloadRaw;
    fn raw(&self, arena: &mut crate::binary::Arena) -> Self::Raw {
        Self::Raw {
            identifier: crate::binary::Record::raw(&self.identifier, arena),
            content: crate::binary::Record::raw(&self.content, arena),
            trigger: self.trigger.as_ref().map_or(core::ptr::null(), |value| {
                let raw = crate::binary::Record::raw(value, arena);
                arena.keep(raw)
            }),
        }
    }
    unsafe fn read(raw: &Self::Raw) -> Result<Self, UserNotificationsError> {
        Ok(Self {
            identifier: unsafe { <String as crate::binary::Record>::read(&raw.identifier) }?,
            content: unsafe {
                <NotificationContentPayload as crate::binary::Record>::read(&raw.content)
            }?,
            trigger: unsafe { raw.trigger.as_ref() }
                .map(|value| unsafe {
                    <NotificationTriggerPayload as crate::binary::Record>::read(value)
                })
                .transpose()?,
        })
    }
}
