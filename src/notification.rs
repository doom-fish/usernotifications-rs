use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::binary::decode_binary;
use crate::error::UserNotificationsError;
use crate::request::{NotificationRequest, NotificationRequestPayload};

/// Wraps `UNNotification`.
#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    /// The delivery date.
    pub date: SystemTime,
    /// The originating notification request.
    pub request: NotificationRequest,
}

#[derive(Debug, Clone)]
pub(crate) struct NotificationPayload {
    date: f64,
    request: NotificationRequestPayload,
}

impl From<&Notification> for NotificationPayload {
    fn from(value: &Notification) -> Self {
        Self {
            date: value
                .date
                .duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::ZERO)
                .as_secs_f64(),
            request: NotificationRequestPayload::from(&value.request),
        }
    }
}

impl From<NotificationPayload> for Notification {
    fn from(value: NotificationPayload) -> Self {
        Self {
            date: Duration::try_from_secs_f64(value.date.max(0.0))
                .ok()
                .and_then(|offset| UNIX_EPOCH.checked_add(offset))
                .unwrap_or(UNIX_EPOCH),
            request: value.request.into(),
        }
    }
}

pub(crate) fn encode_notification_binary(notification: &Notification) -> crate::binary::Input {
    crate::binary::Input::new(&NotificationPayload::from(notification))
}

#[allow(dead_code)]
pub(crate) fn decode_notification_binary(
    ptr: *mut crate::ffi::abi::UNBufferRaw,
) -> Result<Notification, UserNotificationsError> {
    decode_binary::<NotificationPayload>(ptr).map(Into::into)
}

pub(crate) fn decode_notifications_binary(
    ptr: *mut crate::ffi::abi::UNBufferRaw,
) -> Result<Vec<Notification>, UserNotificationsError> {
    decode_binary::<Vec<NotificationPayload>>(ptr)
        .map(|payloads| payloads.into_iter().map(Into::into).collect())
}

impl crate::binary::Record for NotificationPayload {
    type Raw = crate::ffi::abi::UNNotificationPayloadRaw;
    fn raw(&self, arena: &mut crate::binary::Arena) -> Self::Raw {
        Self::Raw {
            date: self.date,
            request: crate::binary::Record::raw(&self.request, arena),
        }
    }
    unsafe fn read(raw: &Self::Raw) -> Result<Self, UserNotificationsError> {
        Ok(Self {
            date: raw.date,
            request: unsafe {
                <NotificationRequestPayload as crate::binary::Record>::read(&raw.request)
            }?,
        })
    }
}
