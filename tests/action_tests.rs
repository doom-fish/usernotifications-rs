use usernotifications::prelude::*;

#[test]
fn action_roundtrip_preserves_text_input_metadata() {
    let action = NotificationAction::new_text_input(
        "reply",
        "Reply",
        NotificationActionOptions::FOREGROUND,
        "Send",
        "Type here",
    )
    .with_icon(NotificationActionIcon::SystemImage("paperplane".into()));
    let roundtrip = action
        .bridge_roundtrip()
        .expect("action roundtrip should succeed");
    assert_eq!(roundtrip.text_input_button_title.as_deref(), Some("Send"));
    assert!(roundtrip.icon.is_some());
}
