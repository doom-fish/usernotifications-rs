use usernotifications::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = NotificationRequest::new(
        "roundtrip-request",
        NotificationContent::new("Request title", "Request body"),
        Some(NotificationTrigger::time_interval(60.0, false)),
    );
    let roundtrip = request.bridge_roundtrip()?;
    println!("identifier = {}", roundtrip.identifier);
    println!("trigger = {:?}", roundtrip.trigger);
    println!("✅ UNNotificationRequest roundtrip OK");
    Ok(())
}
