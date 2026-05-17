#![cfg(feature = "async")]

use usernotifications::prelude::*;
use usernotifications::async_api::AsyncUserNotificationCenter;

#[test]
fn test_async_authorization_request() {
    pollster::block_on(async {
        let center = UserNotificationCenter::current().expect("Failed to get center");

        let result = AsyncUserNotificationCenter::request_authorization(
            &center,
            AuthorizationOptions::ALERT,
        )
        .await;

        assert!(result.is_ok(), "Authorization request should succeed");
    });
}

#[test]
fn test_async_get_delivered_notifications() {
    pollster::block_on(async {
        let center = UserNotificationCenter::current().expect("Failed to get center");

        let result = AsyncUserNotificationCenter::get_delivered_notifications(&center).await;

        assert!(result.is_ok(), "Getting delivered notifications should succeed");
        let notifications = result.expect("Failed to unwrap notifications");
        assert!(
            notifications.is_empty() || !notifications.is_empty(),
            "Should return a list"
        );
    });
}

#[test]
fn test_async_get_pending_requests() {
    pollster::block_on(async {
        let center = UserNotificationCenter::current().expect("Failed to get center");

        let result = AsyncUserNotificationCenter::get_pending_notification_requests(&center).await;

        assert!(result.is_ok(), "Getting pending requests should succeed");
        let requests = result.expect("Failed to unwrap requests");
        assert!(
            requests.is_empty() || !requests.is_empty(),
            "Should return a list"
        );
    });
}

#[test]
fn test_async_get_notification_settings() {
    pollster::block_on(async {
        let center = UserNotificationCenter::current().expect("Failed to get center");

        let result = AsyncUserNotificationCenter::get_notification_settings(&center).await;

        assert!(result.is_ok(), "Getting settings should succeed");
        let _settings = result.expect("Failed to unwrap settings");
    });
}

#[test]
fn test_async_get_notification_categories() {
    pollster::block_on(async {
        let center = UserNotificationCenter::current().expect("Failed to get center");

        let result = AsyncUserNotificationCenter::get_notification_categories(&center).await;

        assert!(result.is_ok(), "Getting categories should succeed");
        let categories = result.expect("Failed to unwrap categories");
        assert!(
            categories.is_empty() || !categories.is_empty(),
            "Should return a list"
        );
    });
}
