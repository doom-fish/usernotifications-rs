use core::ffi::c_char;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::error::UserNotificationsError;
use crate::private::decode_json;
use crate::request::{NotificationRequest, NotificationRequestPayload};

#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub date: SystemTime,
    pub request: NotificationRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            date: UNIX_EPOCH + Duration::from_secs_f64(value.date.max(0.0)),
            request: value.request.into(),
        }
    }
}

pub(crate) fn encode_notification_json(
    notification: &Notification,
) -> Result<String, UserNotificationsError> {
    serde_json::to_string(&NotificationPayload::from(notification)).map_err(|error| {
        UserNotificationsError::FrameworkError(format!(
            "failed to encode notification payload: {error}",
        ))
    })
}

#[allow(dead_code)]
pub(crate) fn decode_notification_json(
    ptr: *mut c_char,
) -> Result<Notification, UserNotificationsError> {
    decode_json::<NotificationPayload>(ptr).map(Into::into)
}

pub(crate) fn decode_notifications_json(
    ptr: *mut c_char,
) -> Result<Vec<Notification>, UserNotificationsError> {
    decode_json::<Vec<NotificationPayload>>(ptr)
        .map(|payloads| payloads.into_iter().map(Into::into).collect())
}
