use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

use usernotifications::prelude::*;

const SMOKE_ENV: &str = "USERNOTIFICATIONS_SMOKE_BUNDLED";
const INFO_PLIST: &str = r#"<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
    <key>CFBundleExecutable</key>
    <string>01_smoke</string>
    <key>CFBundleIdentifier</key>
    <string>fish.doom.usernotifications_rs.smoke</string>
    <key>CFBundleName</key>
    <string>01_smoke</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.14</string>
</dict>
</plist>
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if relaunch_from_app_bundle_if_needed()? {
        return Ok(());
    }

    let center = UserNotificationCenter::current()?;
    let settings = center.notification_settings()?;
    let categories = center.notification_categories()?;

    println!("authorization = {:?}", settings.authorization_status);
    println!("categories = {}", categories.len());
    println!("✅ usernotifications settings + categories OK");
    Ok(())
}

fn relaunch_from_app_bundle_if_needed() -> Result<bool, Box<dyn std::error::Error>> {
    if std::env::var_os(SMOKE_ENV).is_some() {
        return Ok(false);
    }

    let current_exe = std::env::current_exe()?;
    if is_inside_app_bundle(&current_exe) {
        return Ok(false);
    }

    let examples_dir = current_exe
        .parent()
        .ok_or("example binary must have a parent directory")?;
    let bundle_root = examples_dir.join("01_smoke.app");
    let contents_dir = bundle_root.join("Contents");
    let macos_dir = contents_dir.join("MacOS");
    fs::create_dir_all(&macos_dir)?;
    fs::write(contents_dir.join("Info.plist"), INFO_PLIST)?;

    let bundled_executable = macos_dir.join("01_smoke");
    if bundled_executable.exists() {
        fs::remove_file(&bundled_executable)?;
    }
    fs::copy(&current_exe, &bundled_executable)?;
    let mut permissions = fs::metadata(&bundled_executable)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&bundled_executable, permissions)?;

    let status = Command::new(&bundled_executable)
        .env(SMOKE_ENV, "1")
        .status()?;
    if !status.success() {
        return Err(format!("bundled smoke exited with status {status}").into());
    }
    Ok(true)
}

fn is_inside_app_bundle(path: &Path) -> bool {
    path.ancestors()
        .any(|ancestor| ancestor.extension().is_some_and(|ext| ext == "app"))
}
