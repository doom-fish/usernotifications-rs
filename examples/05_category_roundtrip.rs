use usernotifications::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let action = NotificationAction::new(
        "reply",
        "Reply",
        NotificationActionOptions::FOREGROUND,
    );
    let category = NotificationCategory::new(
        "messages",
        vec![action],
        vec!["INSendMessageIntent".into()],
        NotificationCategoryOptions::CUSTOM_DISMISS_ACTION,
    )
    .with_hidden_previews_body_placeholder("New message")
    .with_category_summary_format("%u new messages");
    let roundtrip = category.bridge_roundtrip()?;
    println!("identifier = {}", roundtrip.identifier);
    println!("actions = {}", roundtrip.actions.len());
    println!("summary = {:?}", roundtrip.category_summary_format);
    println!("✅ UNNotificationCategory roundtrip OK");
    Ok(())
}
