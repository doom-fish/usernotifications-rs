#![allow(dead_code)]

use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, UNIX_EPOCH};

use usernotifications::prelude::*;

const PNG_BYTES: &[u8; 70] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0,
    1, 8, 6, 0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 13, 73, 68, 65, 84, 8, 29, 99, 248, 255,
    255, 63, 3, 0, 8, 252, 2, 254, 94, 115, 247, 39, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66,
    96, 130,
];

pub fn write_test_png(file_name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let output_dir = std::env::current_exe()?
        .parent()
        .ok_or("current executable must have a parent directory")?
        .to_path_buf();
    fs::create_dir_all(&output_dir)?;
    let path = output_dir.join(file_name);
    fs::write(&path, PNG_BYTES)?;
    Ok(path)
}

pub fn sample_content() -> NotificationContent {
    NotificationContent::new("Title", "Body")
        .with_subtitle("Subtitle")
        .with_summary_argument("Summary")
        .with_summary_argument_count(1)
        .with_sound(NotificationSound::Default)
        .with_thread_identifier("thread")
}

pub fn sample_request() -> NotificationRequest {
    NotificationRequest::new(
        "request-id",
        sample_content(),
        Some(NotificationTrigger::time_interval(60.0, false)),
    )
}

pub fn sample_notification() -> Notification {
    Notification {
        date: UNIX_EPOCH + Duration::from_secs(1_234),
        request: sample_request(),
    }
}

pub fn sample_response() -> NotificationResponse {
    NotificationResponse {
        action_identifier: default_action_identifier().to_string(),
        notification: sample_notification(),
        user_text: None,
    }
}
