//! Async API for `UserNotifications` framework
//!
//! This module provides async/await wrappers for `UserNotifications` APIs
//! when the `async` feature is enabled. The async API is executor-agnostic
//! and works with any async runtime (Tokio, async-std, smol, etc.).
//!
//! # Available Types
//!
//! | Type | Description |
//! |------|-------------|
//! | [`AsyncRequestAuthorizationFuture`] | Request user notification authorization |
//! | [`AsyncAddRequestFuture`] | Add a notification request |
//! | [`AsyncGetDeliveredFuture`] | Get delivered notifications |
//! | [`AsyncGetPendingFuture`] | Get pending notification requests |
//! | [`AsyncGetCategoriesFuture`] | Get notification categories |
//! | [`AsyncGetSettingsFuture`] | Get notification settings |
//!
//! # Examples
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use usernotifications::prelude::*;
//! use usernotifications::async_api::AsyncUserNotificationCenter;
//!
//! let center = UserNotificationCenter::current()?;
//! let granted = AsyncUserNotificationCenter::request_authorization(
//!     &center,
//!     AuthorizationOptions::ALERT | AuthorizationOptions::BADGE
//! ).await?;
//! println!("Authorization granted: {}", granted);
//! # Ok(())
//! # }
//! ```

#![allow(unknown_lints, clippy::unnecessary_not)]

use crate::category::NotificationCategory;
use crate::error::UserNotificationsError;
use crate::notification::Notification;
use crate::request::NotificationRequest;
use crate::settings::NotificationSettings;
use crate::{
    category::decode_categories_json, ffi, notification::decode_notifications_json,
    request::decode_requests_json, settings::decode_settings_json,
};
use doom_fish_utils::completion::{error_from_cstr, AsyncCompletion, AsyncCompletionFuture};
use std::ffi::c_void;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// ============================================================================
// Request Authorization
// ============================================================================

extern "C" fn request_authorization_callback(
    result: *const c_void,
    error: *const i8,
    ctx: *mut c_void,
) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<bool>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let granted = unsafe {
            let ptr = result.cast::<u8>();
            *ptr != 0
        };
        unsafe { AsyncCompletion::<bool>::complete_ok(ctx, granted) };
    } else {
        unsafe { AsyncCompletion::<bool>::complete_err(ctx, "Unknown error".to_string()) };
    }
}

pub struct RequestAuthorizationFuture {
    inner: AsyncCompletionFuture<bool>,
}

impl Future for RequestAuthorizationFuture {
    type Output = Result<bool, UserNotificationsError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(UserNotificationsError::FrameworkError))
    }
}

// ============================================================================
// Add Notification Request
// ============================================================================

extern "C" fn add_request_callback(_result: *const c_void, error: *const i8, ctx: *mut c_void) {
    if error.is_null() {
        unsafe { AsyncCompletion::<()>::complete_ok(ctx, ()) };
    } else {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<()>::complete_err(ctx, msg) };
    }
}

pub struct AddRequestFuture {
    inner: AsyncCompletionFuture<()>,
}

impl Future for AddRequestFuture {
    type Output = Result<(), UserNotificationsError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(UserNotificationsError::FrameworkError))
    }
}

// ============================================================================
// Get Delivered Notifications
// ============================================================================

extern "C" fn get_delivered_callback(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    use std::ffi::c_char;
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<Vec<Notification>>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let json_ptr = result as *mut c_char;
        match decode_notifications_json(json_ptr) {
            Ok(notifications) => unsafe {
                AsyncCompletion::<Vec<Notification>>::complete_ok(ctx, notifications);
            },
            Err(e) => unsafe {
                AsyncCompletion::<Vec<Notification>>::complete_err(ctx, e.message().to_string());
            },
        }
    } else {
        unsafe { AsyncCompletion::<Vec<Notification>>::complete_err(ctx, "Unknown error".to_string()) };
    }
}

pub struct GetDeliveredFuture {
    inner: AsyncCompletionFuture<Vec<Notification>>,
}

impl Future for GetDeliveredFuture {
    type Output = Result<Vec<Notification>, UserNotificationsError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(UserNotificationsError::FrameworkError))
    }
}

// ============================================================================
// Get Pending Notification Requests
// ============================================================================

extern "C" fn get_pending_callback(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    use std::ffi::c_char;
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<Vec<NotificationRequest>>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let json_ptr = result as *mut c_char;
        match decode_requests_json(json_ptr) {
            Ok(requests) => unsafe {
                AsyncCompletion::<Vec<NotificationRequest>>::complete_ok(ctx, requests);
            },
            Err(e) => unsafe {
                AsyncCompletion::<Vec<NotificationRequest>>::complete_err(
                    ctx,
                    e.message().to_string(),
                );
            },
        }
    } else {
        unsafe {
            AsyncCompletion::<Vec<NotificationRequest>>::complete_err(ctx, "Unknown error".to_string());
        };
    }
}

pub struct GetPendingFuture {
    inner: AsyncCompletionFuture<Vec<NotificationRequest>>,
}

impl Future for GetPendingFuture {
    type Output = Result<Vec<NotificationRequest>, UserNotificationsError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(UserNotificationsError::FrameworkError))
    }
}

// ============================================================================
// Get Notification Categories
// ============================================================================

extern "C" fn get_categories_callback(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    use std::ffi::c_char;
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<Vec<NotificationCategory>>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let json_ptr = result as *mut c_char;
        match decode_categories_json(json_ptr) {
            Ok(categories) => unsafe {
                AsyncCompletion::<Vec<NotificationCategory>>::complete_ok(ctx, categories);
            },
            Err(e) => unsafe {
                AsyncCompletion::<Vec<NotificationCategory>>::complete_err(
                    ctx,
                    e.message().to_string(),
                );
            },
        }
    } else {
        unsafe {
            AsyncCompletion::<Vec<NotificationCategory>>::complete_err(ctx, "Unknown error".to_string());
        };
    }
}

pub struct GetCategoriesFuture {
    inner: AsyncCompletionFuture<Vec<NotificationCategory>>,
}

impl Future for GetCategoriesFuture {
    type Output = Result<Vec<NotificationCategory>, UserNotificationsError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(UserNotificationsError::FrameworkError))
    }
}

// ============================================================================
// Get Notification Settings
// ============================================================================

extern "C" fn get_settings_callback(result: *const c_void, error: *const i8, ctx: *mut c_void) {
    use std::ffi::c_char;
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<NotificationSettings>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let json_ptr = result as *mut c_char;
        match decode_settings_json(json_ptr) {
            Ok(settings) => unsafe {
                AsyncCompletion::<NotificationSettings>::complete_ok(ctx, settings);
            },
            Err(e) => unsafe {
                AsyncCompletion::<NotificationSettings>::complete_err(ctx, e.message().to_string());
            },
        }
    } else {
        unsafe {
            AsyncCompletion::<NotificationSettings>::complete_err(ctx, "Unknown error".to_string());
        };
    }
}

pub struct GetSettingsFuture {
    inner: AsyncCompletionFuture<NotificationSettings>,
}

impl Future for GetSettingsFuture {
    type Output = Result<NotificationSettings, UserNotificationsError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(UserNotificationsError::FrameworkError))
    }
}

// ============================================================================
// Public API
// ============================================================================

/// Async wrapper for `UserNotificationCenter` operations
pub struct AsyncUserNotificationCenter;

impl AsyncUserNotificationCenter {
    /// Request authorization to send notifications asynchronously
    ///
    /// # Errors
    ///
    /// Returns an error if the authorization request fails.
    pub fn request_authorization(
        center: &crate::UserNotificationCenter,
        options: crate::settings::AuthorizationOptions,
    ) -> RequestAuthorizationFuture {
        let (future, ctx) = AsyncCompletion::<bool>::create();
        unsafe {
            ffi::r#async::un_center_request_authorization_async(
                center.as_raw(),
                options.bits(),
                request_authorization_callback,
                ctx,
            );
        }
        RequestAuthorizationFuture { inner: future }
    }

    /// Add a notification request asynchronously
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be added.
    pub fn add_notification_request(
        center: &crate::UserNotificationCenter,
        request: &crate::NotificationRequest,
    ) -> Result<AddRequestFuture, UserNotificationsError> {
        use crate::request::encode_request_json;
        use crate::private::to_cstring;

        let request_json = encode_request_json(request)?;
        let request_cstring = to_cstring(&request_json)?;

        let (future, ctx) = AsyncCompletion::<()>::create();
        unsafe {
            ffi::r#async::un_center_add_notification_request_async(
                center.as_raw(),
                request_cstring.as_ptr(),
                add_request_callback,
                ctx,
            );
        }
        Ok(AddRequestFuture { inner: future })
    }

    /// Get delivered notifications asynchronously
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub fn get_delivered_notifications(
        center: &crate::UserNotificationCenter,
    ) -> GetDeliveredFuture {
        let (future, ctx) = AsyncCompletion::<Vec<Notification>>::create();
        unsafe {
            ffi::r#async::un_center_get_delivered_notifications_async(
                center.as_raw(),
                get_delivered_callback,
                ctx,
            );
        }
        GetDeliveredFuture { inner: future }
    }

    /// Get pending notification requests asynchronously
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub fn get_pending_notification_requests(
        center: &crate::UserNotificationCenter,
    ) -> GetPendingFuture {
        let (future, ctx) = AsyncCompletion::<Vec<NotificationRequest>>::create();
        unsafe {
            ffi::r#async::un_center_get_pending_notification_requests_async(
                center.as_raw(),
                get_pending_callback,
                ctx,
            );
        }
        GetPendingFuture { inner: future }
    }

    /// Get notification categories asynchronously
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub fn get_notification_categories(
        center: &crate::UserNotificationCenter,
    ) -> GetCategoriesFuture {
        let (future, ctx) = AsyncCompletion::<Vec<NotificationCategory>>::create();
        unsafe {
            ffi::r#async::un_center_get_notification_categories_async(
                center.as_raw(),
                get_categories_callback,
                ctx,
            );
        }
        GetCategoriesFuture { inner: future }
    }

    /// Get notification settings asynchronously
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub fn get_notification_settings(
        center: &crate::UserNotificationCenter,
    ) -> GetSettingsFuture {
        let (future, ctx) = AsyncCompletion::<NotificationSettings>::create();
        unsafe {
            ffi::r#async::un_center_get_notification_settings_async(
                center.as_raw(),
                get_settings_callback,
                ctx,
            );
        }
        GetSettingsFuture { inner: future }
    }
}
