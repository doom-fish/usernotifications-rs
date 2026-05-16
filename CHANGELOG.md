# Changelog

## [0.1.0] - 2026-05-16

- Initial release of `usernotifications-rs`.
- Safe Rust wrappers for `UNUserNotificationCenter`, `UNNotificationRequest`, `UNMutableNotificationContent`, `UNNotificationCategory`, `UNNotificationAction`, and `UNNotificationSettings` on macOS.
- Delegate callbacks for `didReceiveNotificationResponse` and `openSettingsForNotification` using Rust closures or trait objects.
- JSON-backed Swift bridge for scheduling requests, inspecting delivered and pending notifications, and querying settings without a UI prompt.
- Explicitly omits `UNLocationNotificationTrigger` because Apple's macOS SDK marks it unavailable.
