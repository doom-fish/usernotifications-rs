use core::ffi::c_char;
use core::fmt;

use libc::free;

use crate::ffi;

/// Matches `UNErrorDomain`.
pub const USER_NOTIFICATIONS_ERROR_DOMAIN: &str = "UNErrorDomain";

/// Wraps documented `UNErrorCode` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum UserNotificationsFrameworkErrorCode {
    /// The notifications not allowed variant.
    NotificationsNotAllowed = 1,
    /// The attachment invalid url variant.
    AttachmentInvalidUrl = 100,
    /// The attachment unrecognized type variant.
    AttachmentUnrecognizedType = 101,
    /// The attachment invalid file size variant.
    AttachmentInvalidFileSize = 102,
    /// The attachment not in data store variant.
    AttachmentNotInDataStore = 103,
    /// The attachment move into data store failed variant.
    AttachmentMoveIntoDataStoreFailed = 104,
    /// The attachment corrupt variant.
    AttachmentCorrupt = 105,
    /// The notification invalid no date variant.
    NotificationInvalidNoDate = 1400,
    /// The notification invalid no content variant.
    NotificationInvalidNoContent = 1401,
    /// The content providing object not allowed variant.
    ContentProvidingObjectNotAllowed = 1500,
    /// The content providing invalid variant.
    ContentProvidingInvalid = 1501,
    /// The badge input invalid variant.
    BadgeInputInvalid = 1600,
}

impl UserNotificationsFrameworkErrorCode {
    /// Converts a raw framework value into a documented error code.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            1 => Some(Self::NotificationsNotAllowed),
            100 => Some(Self::AttachmentInvalidUrl),
            101 => Some(Self::AttachmentUnrecognizedType),
            102 => Some(Self::AttachmentInvalidFileSize),
            103 => Some(Self::AttachmentNotInDataStore),
            104 => Some(Self::AttachmentMoveIntoDataStoreFailed),
            105 => Some(Self::AttachmentCorrupt),
            1400 => Some(Self::NotificationInvalidNoDate),
            1401 => Some(Self::NotificationInvalidNoContent),
            1500 => Some(Self::ContentProvidingObjectNotAllowed),
            1501 => Some(Self::ContentProvidingInvalid),
            1600 => Some(Self::BadgeInputInvalid),
            _ => None,
        }
    }
}

/// Errors returned by the safe `UserNotifications` wrappers.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum UserNotificationsError {
    /// Represents an invalid argument reported by the bridge.
    InvalidArgument(String),
    /// Represents an error returned by the `UserNotifications` framework.
    FrameworkError(String),
    /// Represents an unknown framework or bridge error code.
    Unknown {
        /// The raw error code.
        code: i32,
        /// The associated error message.
        message: String,
    },
}

impl UserNotificationsError {
    /// Returns the raw status code for this error.
    #[must_use]
    pub const fn code(&self) -> i32 {
        match self {
            Self::InvalidArgument(_) => ffi::status::INVALID_ARGUMENT,
            Self::FrameworkError(_) => ffi::status::FRAMEWORK_ERROR,
            Self::Unknown { code, .. } => *code,
        }
    }

    /// Returns the human-readable message for this error.
    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidArgument(message)
            | Self::FrameworkError(message)
            | Self::Unknown { message, .. } => message,
        }
    }
}

impl fmt::Display for UserNotificationsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (code {})", self.message(), self.code())
    }
}

impl std::error::Error for UserNotificationsError {}

pub(crate) fn take_owned_c_string(ptr: *mut c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }

    let string = unsafe { core::ffi::CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    unsafe { free(ptr.cast()) };
    string
}

pub(crate) fn from_swift(status: i32, error_str: *mut c_char) -> UserNotificationsError {
    from_status_message(status, take_owned_c_string(error_str))
}

pub(crate) fn from_status_message(status: i32, message: String) -> UserNotificationsError {
    match status {
        ffi::status::INVALID_ARGUMENT => UserNotificationsError::InvalidArgument(message),
        ffi::status::FRAMEWORK_ERROR => UserNotificationsError::FrameworkError(message),
        code => UserNotificationsError::Unknown { code, message },
    }
}

#[cfg(test)]
mod tests {
    use super::{from_status_message, UserNotificationsError, UserNotificationsFrameworkErrorCode};
    use crate::ffi;

    #[test]
    fn framework_error_code_round_trips_known_values() {
        let codes = [
            UserNotificationsFrameworkErrorCode::NotificationsNotAllowed,
            UserNotificationsFrameworkErrorCode::AttachmentInvalidUrl,
            UserNotificationsFrameworkErrorCode::AttachmentUnrecognizedType,
            UserNotificationsFrameworkErrorCode::AttachmentInvalidFileSize,
            UserNotificationsFrameworkErrorCode::AttachmentNotInDataStore,
            UserNotificationsFrameworkErrorCode::AttachmentMoveIntoDataStoreFailed,
            UserNotificationsFrameworkErrorCode::AttachmentCorrupt,
            UserNotificationsFrameworkErrorCode::NotificationInvalidNoDate,
            UserNotificationsFrameworkErrorCode::NotificationInvalidNoContent,
            UserNotificationsFrameworkErrorCode::ContentProvidingObjectNotAllowed,
            UserNotificationsFrameworkErrorCode::ContentProvidingInvalid,
            UserNotificationsFrameworkErrorCode::BadgeInputInvalid,
        ];

        for code in codes {
            assert_eq!(
                UserNotificationsFrameworkErrorCode::from_raw(code as i32),
                Some(code)
            );
        }
    }

    #[test]
    fn framework_error_code_from_raw_returns_none_for_unknown_value() {
        assert_eq!(UserNotificationsFrameworkErrorCode::from_raw(-1), None);
    }

    #[test]
    fn user_notifications_error_code_returns_expected_values() {
        assert_eq!(
            UserNotificationsError::InvalidArgument("bad input".into()).code(),
            ffi::status::INVALID_ARGUMENT,
        );
        assert_eq!(
            UserNotificationsError::FrameworkError("bridge failed".into()).code(),
            ffi::status::FRAMEWORK_ERROR,
        );
        assert_eq!(
            UserNotificationsError::Unknown {
                code: 77,
                message: "unknown".into(),
            }
            .code(),
            77,
        );
    }

    #[test]
    fn user_notifications_error_message_returns_inner_message() {
        assert_eq!(
            UserNotificationsError::InvalidArgument("bad input".into()).message(),
            "bad input",
        );
        assert_eq!(
            UserNotificationsError::FrameworkError("bridge failed".into()).message(),
            "bridge failed",
        );
        assert_eq!(
            UserNotificationsError::Unknown {
                code: 77,
                message: "unknown".into(),
            }
            .message(),
            "unknown",
        );
    }

    #[test]
    fn from_status_message_maps_statuses_to_variants() {
        assert_eq!(
            from_status_message(ffi::status::INVALID_ARGUMENT, "bad input".into()),
            UserNotificationsError::InvalidArgument("bad input".into()),
        );
        assert_eq!(
            from_status_message(ffi::status::FRAMEWORK_ERROR, "bridge failed".into()),
            UserNotificationsError::FrameworkError("bridge failed".into()),
        );
        assert_eq!(
            from_status_message(77, "unknown".into()),
            UserNotificationsError::Unknown {
                code: 77,
                message: "unknown".into(),
            },
        );
    }

    #[test]
    fn user_notifications_error_display_includes_message_and_code() {
        let error = UserNotificationsError::FrameworkError("bridge failed".into());

        assert_eq!(
            error.to_string(),
            format!("bridge failed (code {})", ffi::status::FRAMEWORK_ERROR),
        );
    }
}
