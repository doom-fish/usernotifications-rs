use usernotifications::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = NotificationContent::new("Content title", "Content body")
        .with_subtitle("Subtitle")
        .with_summary_argument("Summary")
        .with_summary_argument_count(2)
        .with_interruption_level(NotificationInterruptionLevel::TimeSensitive)
        .with_relevance_score(0.75)
        .with_filter_criteria("example-filter")
        .with_sound(NotificationSound::DefaultCritical);
    let roundtrip = content.bridge_roundtrip()?;
    println!("title = {}", roundtrip.title);
    println!(
        "summary_argument_count = {}",
        roundtrip.summary_argument_count
    );
    println!("interruption_level = {:?}", roundtrip.interruption_level);

    let provider = NotificationAttributedMessageContext::new("Content body")
        .with_content("Content body")
        .with_sender(
            NotificationMessagePerson::new(
                "alice@example.com",
                NotificationMessagePersonHandleType::EmailAddress,
            )
            .with_display_name("Alice"),
        )
        .with_recipient(
            NotificationMessagePerson::new(
                "bob@example.com",
                NotificationMessagePersonHandleType::EmailAddress,
            )
            .with_display_name("Bob"),
        )
        .with_conversation_identifier("content-thread")
        .with_service_name("Messages");

    match roundtrip.updating_from(&provider) {
        Ok(updated) => println!("provider_updated_body = {}", updated.body),
        Err(UserNotificationsError::FrameworkError(message))
            if message.contains("macOS 15 or newer") =>
        {
            println!("provider update skipped: {message}");
        }
        Err(error) => return Err(Box::new(error)),
    }

    println!("✅ UNNotificationContent roundtrip OK");
    Ok(())
}
