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
//! | [`RequestAuthorizationFuture`] | Request user notification authorization |
//! | [`AddRequestFuture`] | Add a notification request |
//! | [`GetDeliveredFuture`] | Get delivered notifications |
//! | [`GetPendingFuture`] | Get pending notification requests |
//! | [`GetCategoriesFuture`] | Get notification categories |
//! | [`GetSettingsFuture`] | Get notification settings |
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
        // SAFETY: error is a valid C-string pointer passed from Swift FFI bridge.
        let msg = unsafe { error_from_cstr(error) };
        // SAFETY: ctx is a valid pointer to AsyncCompletion<bool> created by AsyncCompletion::create()
        // and stored in the Swift bridge context. The callback will be called once with this pointer.
        unsafe { AsyncCompletion::<bool>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        let granted = unsafe {
            // SAFETY: result is a pointer to a Swift-passed boolean value (single byte).
            // The result pointer is valid for the lifetime of this callback.
            let ptr = result.cast::<u8>();
            *ptr != 0
        };
        // SAFETY: ctx is a valid pointer to AsyncCompletion<bool> created by AsyncCompletion::create().
        unsafe { AsyncCompletion::<bool>::complete_ok(ctx, granted) };
    } else {
        // SAFETY: ctx is a valid pointer to AsyncCompletion<bool> created by AsyncCompletion::create().
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
        // SAFETY: ctx is a valid pointer to AsyncCompletion<()> created by AsyncCompletion::create().
        unsafe { AsyncCompletion::<()>::complete_ok(ctx, ()) };
    } else {
        // SAFETY: error is a valid C-string pointer passed from Swift FFI bridge.
        let msg = unsafe { error_from_cstr(error) };
        // SAFETY: ctx is a valid pointer to AsyncCompletion<()> created by AsyncCompletion::create().
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
        // SAFETY: error is a valid C-string pointer passed from Swift FFI bridge.
        let msg = unsafe { error_from_cstr(error) };
        // SAFETY: ctx is a valid pointer to AsyncCompletion<Vec<Notification>> created by AsyncCompletion::create().
        unsafe { AsyncCompletion::<Vec<Notification>>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        // SAFETY: result is a valid pointer to a C-string JSON array passed from Swift FFI bridge.
        let json_ptr = result as *mut c_char;
        match decode_notifications_json(json_ptr) {
            Ok(notifications) => {
                // SAFETY: ctx is a valid pointer to AsyncCompletion<Vec<Notification>> created by AsyncCompletion::create().
                unsafe {
                    AsyncCompletion::<Vec<Notification>>::complete_ok(ctx, notifications);
                }
            },
            Err(e) => {
                // SAFETY: ctx is a valid pointer to AsyncCompletion<Vec<Notification>> created by AsyncCompletion::create().
                unsafe {
                    AsyncCompletion::<Vec<Notification>>::complete_err(ctx, e.message().to_string());
                }
            },
        }
    } else {
        // SAFETY: ctx is a valid pointer to AsyncCompletion<Vec<Notification>> created by AsyncCompletion::create().
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
        // SAFETY: error is a valid C-string pointer passed from Swift FFI bridge.
        let msg = unsafe { error_from_cstr(error) };
        // SAFETY: ctx is a valid pointer to AsyncCompletion<Vec<NotificationRequest>> created by AsyncCompletion::create().
        unsafe { AsyncCompletion::<Vec<NotificationRequest>>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        // SAFETY: result is a valid pointer to a C-string JSON array passed from Swift FFI bridge.
        let json_ptr = result as *mut c_char;
        match decode_requests_json(json_ptr) {
            Ok(requests) => {
                // SAFETY: ctx is a valid pointer to AsyncCompletion<Vec<NotificationRequest>> created by AsyncCompletion::create().
                unsafe {
                    AsyncCompletion::<Vec<NotificationRequest>>::complete_ok(ctx, requests);
                }
            },
            Err(e) => {
                // SAFETY: ctx is a valid pointer to AsyncCompletion<Vec<NotificationRequest>> created by AsyncCompletion::create().
                unsafe {
                    AsyncCompletion::<Vec<NotificationRequest>>::complete_err(
                        ctx,
                        e.message().to_string(),
                    );
                }
            },
        }
    } else {
        // SAFETY: ctx is a valid pointer to AsyncCompletion<Vec<NotificationRequest>> created by AsyncCompletion::create().
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
        // SAFETY: error is a valid C-string pointer passed from Swift FFI bridge.
        let msg = unsafe { error_from_cstr(error) };
        // SAFETY: ctx is a valid pointer to AsyncCompletion<Vec<NotificationCategory>> created by AsyncCompletion::create().
        unsafe { AsyncCompletion::<Vec<NotificationCategory>>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        // SAFETY: result is a valid pointer to a C-string JSON array passed from Swift FFI bridge.
        let json_ptr = result as *mut c_char;
        match decode_categories_json(json_ptr) {
            Ok(categories) => {
                // SAFETY: ctx is a valid pointer to AsyncCompletion<Vec<NotificationCategory>> created by AsyncCompletion::create().
                unsafe {
                    AsyncCompletion::<Vec<NotificationCategory>>::complete_ok(ctx, categories);
                }
            },
            Err(e) => {
                // SAFETY: ctx is a valid pointer to AsyncCompletion<Vec<NotificationCategory>> created by AsyncCompletion::create().
                unsafe {
                    AsyncCompletion::<Vec<NotificationCategory>>::complete_err(
                        ctx,
                        e.message().to_string(),
                    );
                }
            },
        }
    } else {
        // SAFETY: ctx is a valid pointer to AsyncCompletion<Vec<NotificationCategory>> created by AsyncCompletion::create().
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
        // SAFETY: error is a valid C-string pointer passed from Swift FFI bridge.
        let msg = unsafe { error_from_cstr(error) };
        // SAFETY: ctx is a valid pointer to AsyncCompletion<NotificationSettings> created by AsyncCompletion::create().
        unsafe { AsyncCompletion::<NotificationSettings>::complete_err(ctx, msg) };
    } else if !result.is_null() {
        // SAFETY: result is a valid pointer to a C-string JSON object passed from Swift FFI bridge.
        let json_ptr = result as *mut c_char;
        match decode_settings_json(json_ptr) {
            Ok(settings) => {
                // SAFETY: ctx is a valid pointer to AsyncCompletion<NotificationSettings> created by AsyncCompletion::create().
                unsafe {
                    AsyncCompletion::<NotificationSettings>::complete_ok(ctx, settings);
                }
            },
            Err(e) => {
                // SAFETY: ctx is a valid pointer to AsyncCompletion<NotificationSettings> created by AsyncCompletion::create().
                unsafe {
                    AsyncCompletion::<NotificationSettings>::complete_err(ctx, e.message().to_string());
                }
            },
        }
    } else {
        // SAFETY: ctx is a valid pointer to AsyncCompletion<NotificationSettings> created by AsyncCompletion::create().
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
        // SAFETY: center.as_raw() returns a valid ObjC handle, ctx is a valid context pointer
        // created by AsyncCompletion::create(), request_authorization_callback is an extern "C" fn
        // that safely handles the callback, and options.bits() is a valid bitfield.
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
        // SAFETY: center.as_raw() returns a valid ObjC handle, request_cstring.as_ptr() is a valid C-string,
        // ctx is a valid context pointer created by AsyncCompletion::create(), and add_request_callback is an
        // extern "C" fn that safely handles the callback.
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
        // SAFETY: center.as_raw() returns a valid ObjC handle, ctx is a valid context pointer
        // created by AsyncCompletion::create(), and get_delivered_callback is an extern "C" fn
        // that safely handles the callback.
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
        // SAFETY: center.as_raw() returns a valid ObjC handle, ctx is a valid context pointer
        // created by AsyncCompletion::create(), and get_pending_callback is an extern "C" fn
        // that safely handles the callback.
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
        // SAFETY: center.as_raw() returns a valid ObjC handle, ctx is a valid context pointer
        // created by AsyncCompletion::create(), and get_categories_callback is an extern "C" fn
        // that safely handles the callback.
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
        // SAFETY: center.as_raw() returns a valid ObjC handle, ctx is a valid context pointer
        // created by AsyncCompletion::create(), and get_settings_callback is an extern "C" fn
        // that safely handles the callback.
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
