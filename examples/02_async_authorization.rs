//! Async authorization request example
//!
//! Demonstrates requesting user notification authorization asynchronously.
//! Requires `--features async` to build.

use usernotifications::prelude::*;
use usernotifications::async_api::AsyncUserNotificationCenter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(async {
        let center = UserNotificationCenter::current()?;

        let options = AuthorizationOptions::ALERT
            | AuthorizationOptions::BADGE
            | AuthorizationOptions::SOUND;

        match AsyncUserNotificationCenter::request_authorization(&center, options).await {
            Ok(granted) => {
                println!("Authorization granted: {granted}");
            }
            Err(e) => {
                eprintln!("Authorization error: {e}");
            }
        }

        Ok(())
    })
}
