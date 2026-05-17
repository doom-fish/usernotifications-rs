//! Async notification queries example
//!
//! Demonstrates retrieving delivered notifications, pending requests, and settings
//! asynchronously.
//! Requires `--features async` to build.

use usernotifications::prelude::*;
use usernotifications::async_api::AsyncUserNotificationCenter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(async {
        let center = UserNotificationCenter::current()?;

        // Get delivered notifications
        match AsyncUserNotificationCenter::get_delivered_notifications(&center).await {
            Ok(notifications) => {
                println!("Delivered notifications: {}", notifications.len());
            }
            Err(e) => {
                eprintln!("Error fetching delivered notifications: {e}");
            }
        }

        // Get pending requests
        match AsyncUserNotificationCenter::get_pending_notification_requests(&center).await {
            Ok(requests) => {
                println!("Pending requests: {}", requests.len());
            }
            Err(e) => {
                eprintln!("Error fetching pending requests: {e}");
            }
        }

        // Get notification settings
        match AsyncUserNotificationCenter::get_notification_settings(&center).await {
            Ok(settings) => {
                println!("Authorization status: {:?}", settings.authorization_status);
            }
            Err(e) => {
                eprintln!("Error fetching settings: {e}");
            }
        }

        // Get notification categories
        match AsyncUserNotificationCenter::get_notification_categories(&center).await {
            Ok(categories) => {
                println!("Notification categories: {}", categories.len());
            }
            Err(e) => {
                eprintln!("Error fetching categories: {e}");
            }
        }

        Ok(())
    })
}
