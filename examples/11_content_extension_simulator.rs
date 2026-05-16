use std::sync::{Arc, Mutex};
use std::time::{Duration, UNIX_EPOCH};

use usernotifications::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let seen_notification = Arc::clone(&seen);
    let seen_response = Arc::clone(&seen);
    let simulator = NotificationContentExtensionSimulator::new(
        NotificationContentExtensionCallbacks::new()
            .on_notification(move |notification| {
                seen_notification
                    .lock()
                    .expect("notification log lock should not be poisoned")
                    .push(notification.request.identifier);
            })
            .on_response(move |response| {
                seen_response
                    .lock()
                    .expect("response log lock should not be poisoned")
                    .push(response.action_identifier);
                NotificationContentExtensionResponseOption::Dismiss
            })
            .on_media_play(|| println!("media play"))
            .on_media_pause(|| println!("media pause")),
    )?;
    simulator.set_media_play_pause_button_type(
        NotificationContentExtensionMediaPlayPauseButtonType::Overlay,
    );
    simulator.set_media_play_pause_button_frame(NotificationContentExtensionRect::new(
        1.0, 2.0, 3.0, 4.0,
    ));
    let context = NotificationContentExtensionContext::new()?;
    context.set_notification_actions(&[NotificationAction::new(
        "open",
        "Open",
        NotificationActionOptions::FOREGROUND,
    )])?;
    let notification = Notification {
        date: UNIX_EPOCH + Duration::from_secs(7),
        request: NotificationRequest::new(
            "content-extension",
            NotificationContent::new("Title", "Body"),
            None,
        ),
    };
    simulator.receive_notification(&notification)?;
    let option = simulator.receive_notification_response(&NotificationResponse {
        action_identifier: default_action_identifier().to_string(),
        notification,
        user_text: None,
    })?;
    simulator.media_play();
    simulator.media_pause();
    println!("actions = {}", context.notification_actions()?.len());
    println!("response_option = {option:?}");
    println!("events = {:?}", seen.lock().expect("event log lock should not be poisoned"));
    println!("✅ UNNotificationContentExtension simulator OK");
    Ok(())
}
