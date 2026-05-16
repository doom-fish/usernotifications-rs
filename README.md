# usernotifications

Safe, idiomatic Rust bindings for Apple's [UserNotifications](https://developer.apple.com/documentation/usernotifications) framework on macOS.

## Features

- **Authorization + settings** — query notification authorization state and detailed `UserNotifications` settings without prompting the user.
- **Request management** — create notification content and schedule local requests with immediate, time-interval, or calendar triggers.
- **Categories + actions** — configure notification categories, regular actions, and text-input actions.
- **Delivered + pending snapshots** — inspect pending requests and delivered notifications as plain Rust data structures.
- **Delegate callbacks** — receive `didReceiveNotificationResponse` and `openSettingsForNotification` callbacks through Rust closures or traits.

## Requirements

- macOS 10.14 or newer
- Xcode 15+ with the macOS SDK
- For authorization and local-notification delivery in GUI apps, the app must run with the appropriate notification entitlements and user consent

## Installation

```toml
[dependencies]
usernotifications-rs = "0.1.0"
```

```rust,no_run
use usernotifications::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let center = UserNotificationCenter::current()?;
    let settings = center.notification_settings()?;
    println!("authorization = {:?}", settings.authorization_status);
    Ok(())
}
```

## Smoke example

```bash
cargo run --example 01_smoke
```

The smoke example reads the current notification settings and configured categories. It does **not** request authorization or schedule a notification, so it should not trigger a permission prompt. When launched from `cargo run`, it re-bundles itself under `target/debug/examples/01_smoke.app` before touching `UserNotifications`, because Apple requires a macOS app bundle.

## Notes

- `UNLocationNotificationTrigger` is unavailable on macOS, so this crate intentionally exposes only immediate, time-interval, and calendar scheduling for local notifications.
- Delegate callbacks require running inside an application context that is allowed to install a `UNUserNotificationCenter` delegate.
- Notification `user_info` values are surfaced as `serde_json::Value` for lossless JSON-friendly transport across the Swift bridge.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
