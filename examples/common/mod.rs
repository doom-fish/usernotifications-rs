#![allow(dead_code)]

use std::error::Error;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

const BUNDLED_ENV: &str = "USERNOTIFICATIONS_BUNDLED";
const PNG_BYTES: &[u8; 70] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0,
    1, 8, 6, 0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 13, 73, 68, 65, 84, 8, 29, 99, 248, 255,
    255, 63, 3, 0, 8, 252, 2, 254, 94, 115, 247, 39, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66,
    96, 130,
];

pub fn relaunch_from_app_bundle_if_needed(
    executable_name: &str,
    bundle_identifier: &str,
) -> Result<bool, Box<dyn Error>> {
    if std::env::var_os(BUNDLED_ENV).is_some() {
        return Ok(false);
    }

    let current_exe = std::env::current_exe()?;
    if is_inside_app_bundle(&current_exe) {
        return Ok(false);
    }

    let examples_dir = current_exe
        .parent()
        .ok_or("example binary must have a parent directory")?;
    let bundle_root = examples_dir.join(format!("{executable_name}.app"));
    let contents_dir = bundle_root.join("Contents");
    let macos_dir = contents_dir.join("MacOS");
    fs::create_dir_all(&macos_dir)?;
    fs::write(
        contents_dir.join("Info.plist"),
        info_plist(executable_name, bundle_identifier),
    )?;

    let bundled_executable = macos_dir.join(executable_name);
    if bundled_executable.exists() {
        fs::remove_file(&bundled_executable)?;
    }
    fs::copy(&current_exe, &bundled_executable)?;
    let mut permissions = fs::metadata(&bundled_executable)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&bundled_executable, permissions)?;

    let status = Command::new(&bundled_executable)
        .env(BUNDLED_ENV, "1")
        .status()?;
    if !status.success() {
        return Err(format!("bundled example exited with status {status}").into());
    }
    Ok(true)
}

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

fn info_plist(executable_name: &str, bundle_identifier: &str) -> String {
    format!(
        r#"<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
    <key>CFBundleExecutable</key>
    <string>{executable_name}</string>
    <key>CFBundleIdentifier</key>
    <string>{bundle_identifier}</string>
    <key>CFBundleName</key>
    <string>{executable_name}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.2.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.14</string>
</dict>
</plist>
"#
    )
}

fn is_inside_app_bundle(path: &Path) -> bool {
    path.ancestors()
        .any(|ancestor| ancestor.extension().is_some_and(|ext| ext == "app"))
}
