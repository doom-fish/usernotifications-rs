use usernotifications::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let calendar = NotificationTrigger::calendar(
        DateComponents::new().with_hour(9).with_minute(30),
        true,
    )
    .bridge_roundtrip()?;
    let interval = NotificationTrigger::time_interval(90.0, false).bridge_roundtrip()?;
    println!("calendar = {calendar:?}");
    println!("interval = {interval:?}");
    println!("✅ UNNotificationTrigger roundtrip OK");
    Ok(())
}
