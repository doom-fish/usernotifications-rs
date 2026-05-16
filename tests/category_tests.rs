use usernotifications::prelude::*;

#[test]
fn category_roundtrip_preserves_placeholder() {
    let action = NotificationAction::new(
        "reply",
        "Reply",
        NotificationActionOptions::FOREGROUND,
    );
    let category = NotificationCategory::new(
        "messages",
        vec![action],
        vec![],
        NotificationCategoryOptions::CUSTOM_DISMISS_ACTION,
    )
    .with_hidden_previews_body_placeholder("Placeholder")
    .with_category_summary_format("%u messages");
    let roundtrip = category
        .bridge_roundtrip()
        .expect("category roundtrip should succeed");
    assert_eq!(roundtrip.hidden_previews_body_placeholder.as_deref(), Some("Placeholder"));
    assert_eq!(roundtrip.actions.len(), 1);
}
