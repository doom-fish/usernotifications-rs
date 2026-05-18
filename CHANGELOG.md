# Changelog

## [0.3.1] - 2026-05-18

- Widen apple-cf version bound to `<0.9` so the 0.8.0 nested-CGRect dep resolves. No source changes.

## [0.3.0] - 2025-05-17

- Added `async_api` module with async/await wrappers for completion-based APIs when the `async` feature is enabled.
  - `AsyncUserNotificationCenter::request_authorization()` — async authorization requests
  - `AsyncUserNotificationCenter::add_notification_request()` — async request scheduling
  - `AsyncUserNotificationCenter::get_delivered_notifications()` — async delivered notification retrieval
  - `AsyncUserNotificationCenter::get_pending_notification_requests()` — async pending request retrieval
  - `AsyncUserNotificationCenter::get_notification_categories()` — async category retrieval
  - `AsyncUserNotificationCenter::get_notification_settings()` — async settings retrieval
- All async operations use callback-based Swift FFI with `doom-fish-utils::completion` for true executor-agnostic async/await support.

## [0.2.1] - 2026-05-17

- Added macOS 15 content-provider coverage with the sealed `NotificationContentProviding` trait, `NotificationContent::updating_from`, and the Intents-backed `NotificationAttributedMessageContext` helper.
- Added content-provider smoke coverage to the content integration test and example, and updated the checked-in coverage docs to reflect full audit coverage.

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
