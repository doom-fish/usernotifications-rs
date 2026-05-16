#[path = "common/mod.rs"]
mod common;

use usernotifications::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let png = common::write_test_png("07_attachment_roundtrip.png")?;
    let attachment = NotificationAttachment::from_file_with_options(
        "preview",
        &png,
        NotificationAttachmentOptions::new().with_type_hint("public.png"),
    )?;
    println!("path = {}", attachment.file_path().display());
    println!("type = {:?}", attachment.attachment_type);
    println!("✅ UNNotificationAttachment roundtrip OK");
    Ok(())
}
