# Changelog

## [0.2.0] - 2026-05-16

- Split the crate into per-area Rust modules and per-area Swift bridge files following the screencapturekit-style layout.
- Expanded center coverage with will-present callbacks, `supportsContentExtensions`, `setBadgeCount`, richer category/request/delivered-notification handling, and broader settings decoding.
- Added coverage for attachments, action icons, response constants/presentation options, richer content metadata, and typed `UNErrorDomain` / `UNErrorCode` constants.
- Added simulator/context wrappers for `UNNotificationServiceExtension` and `UNNotificationContentExtension`.
- Added 11 examples, integration tests across the logical areas, and a checked-in `COVERAGE.md` SDK matrix.

## [0.1.0] - 2026-05-16

- Initial release of `usernotifications-rs`.
- Safe Rust wrappers for `UNUserNotificationCenter`, `UNNotificationRequest`, `UNMutableNotificationContent`, `UNNotificationCategory`, `UNNotificationAction`, and `UNNotificationSettings` on macOS.
- Delegate callbacks for `didReceiveNotificationResponse` and `openSettingsForNotification` using Rust closures or trait objects.
- JSON-backed Swift bridge for scheduling requests, inspecting delivered and pending notifications, and querying settings without a UI prompt.
- Explicitly omits `UNLocationNotificationTrigger` because Apple's macOS SDK marks it unavailable.
