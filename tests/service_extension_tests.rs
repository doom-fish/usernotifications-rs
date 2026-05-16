use std::sync::{Arc, Mutex};

use usernotifications::prelude::*;

#[test]
fn service_extension_simulator_invokes_callbacks() {
    let expired = Arc::new(Mutex::new(false));
    let expired_flag = Arc::clone(&expired);
    let simulator = NotificationServiceExtensionSimulator::new(
        NotificationServiceExtensionCallbacks::new()
            .on_receive_notification_request(|request| request.content.with_subtitle("Modified"))
            .on_time_will_expire(move || {
                *expired_flag
                    .lock()
                    .expect("expire lock should not be poisoned") = true;
            }),
    )
    .expect("service extension simulator should be created");

    let delivered = simulator
        .receive_notification_request(&NotificationRequest::new(
            "service-extension",
            NotificationContent::new("Title", "Body"),
            Some(NotificationTrigger::time_interval(60.0, false)),
        ))
        .expect("service extension should produce content");
    simulator.service_extension_time_will_expire();

    assert_eq!(delivered.subtitle, "Modified");
    assert!(*expired.lock().expect("expire lock should not be poisoned"));
}
