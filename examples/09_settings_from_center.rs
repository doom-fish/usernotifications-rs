#[path = "common/mod.rs"]
mod common;

use usernotifications::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if common::relaunch_from_app_bundle_if_needed(
        "09_settings_from_center",
        "fish.doom.usernotifications_rs.notification_settings",
    )? {
        return Ok(());
    }

    let center = UserNotificationCenter::current()?;
    let settings = center.notification_settings()?;
    println!("authorization = {:?}", settings.authorization_status);
    println!("alert = {:?}", settings.alert_setting);
    println!("show_previews = {:?}", settings.show_previews_setting);
    println!("✅ UNNotificationSettings query OK");
    Ok(())
}
