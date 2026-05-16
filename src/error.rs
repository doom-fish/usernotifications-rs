use core::ffi::c_char;
use core::fmt;

use libc::free;

use crate::ffi;

pub const USER_NOTIFICATIONS_ERROR_DOMAIN: &str = "UNErrorDomain";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum UserNotificationsFrameworkErrorCode {
    NotificationsNotAllowed = 1,
    AttachmentInvalidUrl = 100,
    AttachmentUnrecognizedType = 101,
    AttachmentInvalidFileSize = 102,
    AttachmentNotInDataStore = 103,
    AttachmentMoveIntoDataStoreFailed = 104,
    AttachmentCorrupt = 105,
    NotificationInvalidNoDate = 1400,
    NotificationInvalidNoContent = 1401,
    ContentProvidingObjectNotAllowed = 1500,
    ContentProvidingInvalid = 1501,
    BadgeInputInvalid = 1600,
}

impl UserNotificationsFrameworkErrorCode {
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum UserNotificationsError {
    InvalidArgument(String),
    FrameworkError(String),
    Unknown { code: i32, message: String },
}

impl UserNotificationsError {
    #[must_use]
    pub const fn code(&self) -> i32 {
        match self {
            Self::InvalidArgument(_) => ffi::status::INVALID_ARGUMENT,
            Self::FrameworkError(_) => ffi::status::FRAMEWORK_ERROR,
            Self::Unknown { code, .. } => *code,
        }
    }

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
