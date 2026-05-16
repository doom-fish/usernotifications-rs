use usernotifications::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let simulator = NotificationServiceExtensionSimulator::new(
        NotificationServiceExtensionCallbacks::new().on_receive_notification_request(|request| {
            request
                .content
                .with_subtitle("Modified in service extension")
        }),
    )?;
    let delivered = simulator.receive_notification_request(&NotificationRequest::new(
        "service-extension",
        NotificationContent::new("Incoming title", "Incoming body"),
        Some(NotificationTrigger::time_interval(60.0, false)),
    ))?;
    println!("subtitle = {}", delivered.subtitle);
    println!("✅ UNNotificationServiceExtension simulator OK");
    Ok(())
}
