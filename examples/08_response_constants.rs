use std::time::{Duration, UNIX_EPOCH};

use usernotifications::prelude::*;

fn main() {
    let response = NotificationResponse {
        action_identifier: default_action_identifier().to_string(),
        notification: Notification {
            date: UNIX_EPOCH + Duration::from_secs(42),
            request: NotificationRequest::new(
                "response-request",
                NotificationContent::new("Response title", "Response body"),
                None,
            ),
        },
        user_text: None,
    };
    println!("default = {}", default_action_identifier());
    println!("dismiss = {}", dismiss_action_identifier());
    println!("is_default_action = {}", response.is_default_action());
    println!("✅ UNNotificationResponse constants OK");
}
