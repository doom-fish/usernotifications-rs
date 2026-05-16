#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's
//! [UserNotifications](https://developer.apple.com/documentation/usernotifications)
//! framework.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::new_without_default
)]

pub mod center;
pub mod error;
pub mod ffi;
pub mod notification;
mod private;

pub use center::{
    UserNotificationCenter, UserNotificationCenterCallbacks, UserNotificationCenterDelegate,
};
pub use error::UserNotificationsError;
pub use notification::{
    AlertStyle, AuthorizationOptions, AuthorizationStatus, DateComponents, Notification,
    NotificationAction, NotificationActionOptions, NotificationCategory,
    NotificationCategoryOptions, NotificationContent, NotificationRequest, NotificationResponse,
    NotificationSetting, NotificationSettings, NotificationSound, NotificationTrigger,
    ShowPreviewsSetting,
};

/// Common imports.
pub mod prelude {
    pub use crate::center::{
        UserNotificationCenter, UserNotificationCenterCallbacks, UserNotificationCenterDelegate,
    };
    pub use crate::error::UserNotificationsError;
    pub use crate::notification::{
        AlertStyle, AuthorizationOptions, AuthorizationStatus, DateComponents, Notification,
        NotificationAction, NotificationActionOptions, NotificationCategory,
        NotificationCategoryOptions, NotificationContent, NotificationRequest,
        NotificationResponse, NotificationSetting, NotificationSettings, NotificationSound,
        NotificationTrigger, ShowPreviewsSetting,
    };
}
