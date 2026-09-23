# usernotifications

Safe, idiomatic Rust bindings for Apple's [UserNotifications](https://developer.apple.com/documentation/usernotifications) and `UserNotificationsUI` frameworks on macOS.

## Highlights

- 11 logical coverage areas with one Rust module and one Swift bridge file per area.
- Expanded `UNUserNotificationCenter` coverage for settings, categories, delivered/pending notifications, delegate callbacks, `supportsContentExtensions`, and `setBadgeCount`.
- Rich model coverage for requests, content, triggers, categories, actions, attachments, responses, and settings.
- Added macOS 15 content-provider specialization via `NotificationContentProviding`, `NotificationContent::updating_from`, and `NotificationAttributedMessageContext`.
- Rust-friendly simulator/context wrappers for `UNNotificationServiceExtension` and `UNNotificationContentExtension`.
- 13 examples plus integration tests spanning every logical area.
- A checked-in [COVERAGE.md](COVERAGE.md) matrix documenting implemented, simulated, and macOS-unavailable SDK surface.

## Requirements

- macOS 10.14 or newer
- Xcode 16 or newer (the bridge uses macOS 15 SDK symbols behind runtime availability checks)
- Some APIs are availability-gated by Apple (`UNNotificationContentExtension` on macOS 11+, action icons / interruption metadata on macOS 12+, badge count / filter criteria on macOS 13+, content-provider specialization on macOS 15+). The `async` feature's operations need macOS 12 and fail with an error on older systems.
- For authorization and local-notification delivery in GUI apps, the app must run with the appropriate notification entitlements and user consent

## Installation

```toml
[dependencies]
usernotifications-rs = "0.4"
```

```rust,no_run
use usernotifications::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let center = UserNotificationCenter::current()?;
    let settings = center.notification_settings()?;
    println!("authorization = {:?}", settings.authorization_status);
    println!("supports_content_extensions = {}", center.supports_content_extensions());
    Ok(())
}
```

## Example catalog

| Example | Area |
| --- | --- |
| `01_smoke` | `UNUserNotificationCenter` smoke / bundle bootstrap |
| `02_async_authorization` | async authorization request (feature `async`) |
| `03_async_queries` | async delivered/pending/settings queries (feature `async`) |
| `02_request_roundtrip` | `UNNotificationRequest` |
| `03_content_roundtrip` | `UNNotificationContent` / content providers |
| `04_trigger_roundtrip` | `UNNotificationTrigger` |
| `05_category_roundtrip` | `UNNotificationCategory` |
| `06_action_roundtrip` | `UNNotificationAction` / `UNNotificationActionIcon` |
| `07_attachment_roundtrip` | `UNNotificationAttachment` |
| `08_response_constants` | `UNNotificationResponse` constants |
| `09_settings_from_center` | `UNNotificationSettings` via center |
| `10_service_extension_simulator` | `UNNotificationServiceExtension` simulator |
| `11_content_extension_simulator` | `UNNotificationContentExtension` simulator/context |

Run any example with:

```bash
cargo run --example 01_smoke
```

## Notes

- `UNUserNotificationCenter::current()` must run from a macOS app bundle. The bundle-aware examples re-launch themselves from `target/debug/examples/*.app` when started via `cargo run`.
- Every `UserNotificationCenter` value can install its own delegate. The crate sets one shared delegate object on the process-wide center while any value has a delegate installed. It forwards responses and settings requests to every installed delegate and combines the presentation options that all `will_present_notification` implementations return. Clearing or dropping a value removes only its own delegate, and dropping a value that never installed one changes nothing.
- Blocking calls wait at most 30 seconds and then return `UserNotificationsError::TimedOut`. After an authorization timeout the permission prompt may still be showing; the request hasn't been denied.
- Adding a request moves its attachment files into the system's attachment data store, so they no longer exist at their original paths. Pass copies of files you need to keep.
- Time-interval triggers need a finite interval greater than zero (at least 60 seconds when repeating); other values return `UserNotificationsError::InvalidArgument` instead of reaching `UNTimeIntervalNotificationTrigger`, which would abort the process.
- `LocalizedNotificationString` is the Rust representation of Apple's `localizedUserNotificationStringForKey:arguments:` localization flow.
- Extension APIs are exposed as simulator/context wrappers rather than generated Xcode extension targets, which keeps the crate usable from ordinary Rust test and example binaries.
- Notification `user_info` values are surfaced as `serde_json::Value` for lossless JSON-friendly transport across the Swift bridge.
- `NotificationContentProviding` is intentionally sealed because Apple only accepts SDK-owned provider objects; `NotificationAttributedMessageContext` is the safe Rust helper for the macOS 15 message-style provider flow.
- Platform-unavailable APIs such as `UNLocationNotificationTrigger` are documented explicitly in [COVERAGE.md](COVERAGE.md).

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
