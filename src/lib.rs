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

/// Wrappers for `UNNotificationAction`, `UNTextInputNotificationAction`, and related option types.
pub mod action;
/// Wrappers for `UNNotificationAttachment` and attachment option keys.
pub mod attachment;
/// Wrappers for `UNNotificationCategory` and related option types.
pub mod category;
/// Wrappers for `UNUserNotificationCenter` and delegate integration.
pub mod center;
/// Wrappers for `UNNotificationContent`, `UNMutableNotificationContent`, and content-provider helpers.
pub mod content;
/// Helpers for the `UNNotificationContentExtension` protocol and extension context APIs.
pub mod content_extension;
/// Error types and constants for the `UserNotifications` framework.
pub mod error;
/// Low-level FFI bindings that back the safe `UserNotifications` wrappers.
pub mod ffi;
/// Wrappers for `UNNotification`.
pub mod notification;
mod option_set;
mod private;
/// Wrappers for `UNNotificationRequest`.
pub mod request;
/// Wrappers for `UNNotificationResponse` and presentation options.
pub mod response;
/// Helpers for the `UNNotificationServiceExtension` API surface.
pub mod service_extension;
/// Wrappers for `UNNotificationSettings` and authorization-related enums.
pub mod settings;
/// Wrappers for `UNNotificationTrigger` and concrete trigger types.
pub mod trigger;

/// Async wrappers for `UNUserNotificationCenter` completion-handler APIs.
#[cfg(feature = "async")]
pub mod async_api;

pub use action::{NotificationAction, NotificationActionIcon, NotificationActionOptions};
pub use attachment::{
    AttachmentThumbnailClippingRect, AttachmentThumbnailTime, NotificationAttachment,
    NotificationAttachmentOptions, ATTACHMENT_OPTIONS_THUMBNAIL_CLIPPING_RECT_KEY,
    ATTACHMENT_OPTIONS_THUMBNAIL_HIDDEN_KEY, ATTACHMENT_OPTIONS_THUMBNAIL_TIME_KEY,
    ATTACHMENT_OPTIONS_TYPE_HINT_KEY,
};
pub use category::{NotificationCategory, NotificationCategoryOptions};
pub use center::{
    UserNotificationCenter, UserNotificationCenterCallbacks, UserNotificationCenterDelegate,
};
pub use content::{
    LocalizedNotificationString, NotificationAttributedMessageContext, NotificationContent,
    NotificationContentProviding, NotificationInterruptionLevel, NotificationMessagePerson,
    NotificationMessagePersonHandleType, NotificationMessageType, NotificationSound,
};
pub use content_extension::{
    NotificationContentExtensionCallbacks, NotificationContentExtensionContext,
    NotificationContentExtensionHandler, NotificationContentExtensionMediaPlayPauseButtonType,
    NotificationContentExtensionRect, NotificationContentExtensionResponseOption,
    NotificationContentExtensionSimulator, NotificationContentExtensionTintColor,
};
pub use error::{
    UserNotificationsError, UserNotificationsFrameworkErrorCode, USER_NOTIFICATIONS_ERROR_DOMAIN,
};
pub use notification::Notification;
pub use request::NotificationRequest;
pub use response::{
    default_action_identifier, dismiss_action_identifier, NotificationPresentationOptions,
    NotificationResponse,
};
pub use service_extension::{
    NotificationServiceExtensionCallbacks, NotificationServiceExtensionHandler,
    NotificationServiceExtensionSimulator,
};
pub use settings::{
    AlertStyle, AuthorizationOptions, AuthorizationStatus, NotificationSetting,
    NotificationSettings, ShowPreviewsSetting,
};
pub use trigger::{CalendarTrigger, DateComponents, NotificationTrigger, TimeIntervalTrigger};

/// Common imports.
pub mod prelude {
    pub use crate::action::{
        NotificationAction, NotificationActionIcon, NotificationActionOptions,
    };
    pub use crate::attachment::{
        AttachmentThumbnailClippingRect, AttachmentThumbnailTime, NotificationAttachment,
        NotificationAttachmentOptions,
    };
    pub use crate::category::{NotificationCategory, NotificationCategoryOptions};
    pub use crate::center::{
        UserNotificationCenter, UserNotificationCenterCallbacks, UserNotificationCenterDelegate,
    };
    pub use crate::content::{
        LocalizedNotificationString, NotificationAttributedMessageContext, NotificationContent,
        NotificationContentProviding, NotificationInterruptionLevel, NotificationMessagePerson,
        NotificationMessagePersonHandleType, NotificationMessageType, NotificationSound,
    };
    pub use crate::content_extension::{
        NotificationContentExtensionCallbacks, NotificationContentExtensionContext,
        NotificationContentExtensionHandler, NotificationContentExtensionMediaPlayPauseButtonType,
        NotificationContentExtensionRect, NotificationContentExtensionResponseOption,
        NotificationContentExtensionSimulator, NotificationContentExtensionTintColor,
    };
    pub use crate::error::{
        UserNotificationsError, UserNotificationsFrameworkErrorCode,
        USER_NOTIFICATIONS_ERROR_DOMAIN,
    };
    pub use crate::notification::Notification;
    pub use crate::request::NotificationRequest;
    pub use crate::response::{
        default_action_identifier, dismiss_action_identifier, NotificationPresentationOptions,
        NotificationResponse,
    };
    pub use crate::service_extension::{
        NotificationServiceExtensionCallbacks, NotificationServiceExtensionHandler,
        NotificationServiceExtensionSimulator,
    };
    pub use crate::settings::{
        AlertStyle, AuthorizationOptions, AuthorizationStatus, NotificationSetting,
        NotificationSettings, ShowPreviewsSetting,
    };
    pub use crate::trigger::{
        CalendarTrigger, DateComponents, NotificationTrigger, TimeIntervalTrigger,
    };
}
