use usernotifications::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let action = NotificationAction::new_text_input(
        "reply",
        "Reply",
        NotificationActionOptions::FOREGROUND,
        "Send",
        "Type a reply",
    )
    .with_icon(NotificationActionIcon::SystemImage("paperplane".into()));
    let roundtrip = action.bridge_roundtrip()?;
    println!("identifier = {}", roundtrip.identifier);
    println!("icon = {:?}", roundtrip.icon);
    println!("button_title = {:?}", roundtrip.text_input_button_title);
    println!("✅ UNNotificationAction roundtrip OK");
    Ok(())
}
