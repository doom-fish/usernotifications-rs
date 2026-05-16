use std::sync::{Arc, Mutex};
use std::time::{Duration, UNIX_EPOCH};

use usernotifications::prelude::*;

#[test]
fn content_extension_simulator_tracks_callbacks_and_context() {
    let events = Arc::new(Mutex::new(Vec::new()));
    let notification_events = Arc::clone(&events);
    let response_events = Arc::clone(&events);
    let simulator = NotificationContentExtensionSimulator::new(
        NotificationContentExtensionCallbacks::new()
            .on_notification(move |notification| {
                notification_events
                    .lock()
                    .expect("notification log lock should not be poisoned")
                    .push(notification.request.identifier);
            })
            .on_response(move |response| {
                response_events
                    .lock()
                    .expect("response log lock should not be poisoned")
                    .push(response.action_identifier);
                NotificationContentExtensionResponseOption::Dismiss
            }),
    )
    .expect("content extension simulator should be created");

    simulator.set_media_play_pause_button_type(
        NotificationContentExtensionMediaPlayPauseButtonType::Overlay,
    );
    simulator.set_media_play_pause_button_frame(NotificationContentExtensionRect::new(
        0.0, 1.0, 2.0, 3.0,
    ));
    let context = NotificationContentExtensionContext::new()
        .expect("content extension context should be created");
    context
        .set_notification_actions(&[NotificationAction::new(
            "open",
            "Open",
            NotificationActionOptions::FOREGROUND,
        )])
        .expect("content extension actions should be set");

    let notification = Notification {
        date: UNIX_EPOCH + Duration::from_secs(5),
        request: NotificationRequest::new(
            "content-extension",
            NotificationContent::new("Title", "Body"),
            None,
        ),
    };
    let option = simulator
        .receive_notification_response(&NotificationResponse {
            action_identifier: default_action_identifier().to_string(),
            notification: notification.clone(),
            user_text: None,
        })
        .expect("content extension response should succeed");
    simulator
        .receive_notification(&notification)
        .expect("content extension notification should succeed");

    assert_eq!(
        simulator.media_play_pause_button_type(),
        NotificationContentExtensionMediaPlayPauseButtonType::Overlay
    );
    assert_eq!(context.notification_actions().expect("actions should load").len(), 1);
    assert_eq!(option, NotificationContentExtensionResponseOption::Dismiss);
    assert_eq!(
        events.lock().expect("event log lock should not be poisoned").len(),
        2
    );
}
