mod common;

use usernotifications::prelude::*;

#[test]
fn attachment_from_file_detects_type() {
    let png = common::write_test_png("attachment_tests.png").expect("test png should be written");
    let attachment = NotificationAttachment::from_file_with_options(
        "preview",
        &png,
        NotificationAttachmentOptions::new().with_type_hint("public.png"),
    )
    .expect("attachment creation should succeed");
    assert_eq!(attachment.file_path(), png.as_path());
    assert!(attachment.attachment_type.is_some());
}
