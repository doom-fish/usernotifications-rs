# Changelog

All notable changes to `usernotifications-rs` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-09-24

### Security

- The center delegate's callback state is no longer freed while
  UserNotifications can still call it. It lives in a reference-counted
  `CallbackContext`; each registration holds a reference, in-flight callbacks
  keep theirs, and Rust deactivates the context before unregistering.
- The async `add_notification_request` no longer reads the caller's request
  JSON from a Swift task after the FFI call has returned (and the string was
  freed); the request is built before the task starts.

### Fixed

- Dropping a temporary `UserNotificationCenter::current()` (or any other
  value) no longer clears another value's delegate. One shared, refcounted
  delegate object is installed on the process-wide center while any value has
  a delegate, and each value only adds or removes its own registration.
- A time-interval trigger with an interval of zero or less, a non-finite
  interval, or a repeating interval under 60 seconds returns
  `InvalidArgument` instead of raising an Objective-C exception that aborted
  the process.
- Blocking center calls and the service-extension simulator wait at most
  30 seconds and return `UserNotificationsError::TimedOut`; completions write
  into a lock-protected result.
- A `summary_argument_count` above `i64::MAX` is clamped instead of trapping
  in Swift, and other framework raw values are converted without trapping.
- Out-of-range trigger and notification dates decode without panicking.
- The callbacks-builder test asserts the will-present result.
- `build.rs` no longer adds the toolchain's Swift 5.5 back-deployment
  directory (`usr/lib/swift-5.5/macosx`) to the link search path or the
  rpath. Its old `libswift_Concurrency.dylib` shadowed the SDK's
  `libswift_Concurrency.tbd` in every binary that depends on this crate, so
  linking failed next to a Swift bridge that uses newer concurrency APIs,
  such as apple-localauthentication's.

### Changed

- **Breaking:** the Swift bridge targets macOS 12 (was 10.14), so binaries
  need macOS 12 or newer. Its async API uses Swift concurrency, which ran on
  older systems without an availability check. The bridge's availability
  checks for macOS 11 and 12 are gone, including the per-call macOS 12 checks
  in the async API.
- **Breaking:** `UserNotificationCenter::clear_delegate` removes only the
  delegate installed through that value. Responses and settings requests go
  to every installed delegate, and the presentation options of all
  `will_present_notification` implementations are combined.
- **Breaking:** `ffi::center::un_center_set_delegate` takes context retain and
  release callbacks; the settings, categories, pending-request and
  delivered-notification ffi getters return a status and write their JSON to
  an out-pointer.
- `apple-cf` requirement is `>=0.11, <0.12` (sibling path dependency) and
  `doom-fish-utils` is `>=0.4.1, <0.5`.
- `rust-version` is 1.82.
- `swift-bridge/.build` is no longer tracked in git.
- The async authorization-request test is `#[ignore]`: from an app bundle it
  would show a permission prompt.
- README and COVERAGE explain the delegate model, timeouts and attachment
  handling, require Xcode 16, and say what the coverage numbers measure.

### Added

- `UserNotificationsError::TimedOut` (status `-3`).
- Docs on `add_notification_request` and `NotificationAttachment`: adding a
  request moves its attachment files into the system's attachment store.

## [0.3.7] - 2026-06-06

- Async completion trampolines are panic-guarded, and an ObjC object is no
  longer passed where the Rust side expected a C string.

## [0.3.6] - 2026-05-20

- Clippy hygiene sweep: cleared all `-D warnings` lints across the crate. No public API change.

## [0.3.5] - 2026-05-20

- Added in-`src/` unit tests across `settings`, `response`, `action`, `trigger`, `content`, and `error`, providing fast `cargo test --lib` fail-fast signal alongside the existing integration tests under `tests/`.
- Widened the `doom-fish-utils` dependency bound to `<0.4` so the sibling 0.3.x crate resolves during release verification.

## [0.3.4] - 2026-05-18

- Added rustdoc coverage across the public UserNotifications wrappers, lifting crate docs from 2.2% to full public-item coverage.

## [0.3.3] - 2026-05-18

- Widen doom-fish-utils version bound to `<0.3` so 0.2.x resolves.

## [0.3.2] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

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
