use usernotifications::prelude::*;

#[test]
fn content_roundtrip_preserves_summary_and_interruption() {
    let content = NotificationContent::new("Title", "Body")
        .with_summary_argument("Summary")
        .with_summary_argument_count(2)
        .with_interruption_level(NotificationInterruptionLevel::TimeSensitive)
        .with_relevance_score(0.5)
        .with_filter_criteria("filter");
    let roundtrip = content
        .bridge_roundtrip()
        .expect("content roundtrip should succeed");
    assert_eq!(roundtrip.summary_argument, "Summary");
    assert_eq!(roundtrip.summary_argument_count, 2);
    assert_eq!(
        roundtrip.interruption_level,
        Some(NotificationInterruptionLevel::TimeSensitive)
    );
}

#[test]
fn content_updating_from_attributed_message_context_smoke_tests() {
    let content = NotificationContent::new("Title", "Body").with_thread_identifier("thread-1");
    let provider = NotificationAttributedMessageContext::new("Body")
        .with_content("Body")
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
        .with_conversation_identifier("thread-1")
        .with_service_name("Messages");

    match content.updating_from(&provider) {
        Ok(updated) => {
            assert_eq!(updated.title, "Title");
            assert_eq!(updated.body, "Body");
            assert_eq!(updated.thread_identifier, "thread-1");
        }
        Err(UserNotificationsError::FrameworkError(message)) => {
            assert!(
                message.contains("macOS 15 or newer"),
                "unexpected error: {message}"
            );
        }
        Err(error) => panic!("content update should succeed: {error}"),
    }
}
