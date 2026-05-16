# UserNotifications SDK coverage

`usernotifications-rs` v0.2.1 was audited against the macOS `UserNotifications` and `UserNotificationsUI` headers in the active Xcode SDK. Every macOS-facing area is accounted for below as a direct Rust binding, a Rust-friendly simulator/context wrapper, or an explicit skip when Apple does not expose that API on macOS.

## Legend

- ✅ direct safe Rust binding
- 🧪 Rust-friendly simulator/context wrapper for extension-only APIs
- 🚫 explicit skip because Apple marks the API unavailable on macOS
- ↔️ represented by a Rust-native helper instead of a literal Objective-C method name

## Covered logical areas

| Area | Apple SDK surface | Status | Notes |
| --- | --- | --- | --- |
| Center | `UNUserNotificationCenter`, center delegate callbacks, request authorization, category management, pending requests, delivered notifications, `supportsContentExtensions`, `setBadgeCount` | ✅ | Exposed through `UserNotificationCenter`, `UserNotificationCenterDelegate`, and `UserNotificationCenterCallbacks`. |
| Request | `UNNotificationRequest` | ✅ | Exposed as `NotificationRequest`. |
| Content | `UNNotificationContent` / `UNMutableNotificationContent` macOS properties: title, subtitle, body, badge, category/thread identifiers, `userInfo`, sounds, attachments, summary fields, interruption level, relevance score, filter criteria | ✅ | Exposed as `NotificationContent` and `NotificationSound`. |
| Content providers | `UNNotificationContentProviding`, `-[UNNotificationContent contentByUpdatingWithProvider:error:]`, `UNNotificationAttributedMessageContext` | ✅ | Exposed as `NotificationContentProviding`, `NotificationContent::updating_from`, `NotificationAttributedMessageContext`, `NotificationMessagePerson`, and related enums. Requires macOS 15+ at runtime. |
| Localized strings | `+[NSString localizedUserNotificationStringForKey:arguments:]` | ↔️ | Represented by `LocalizedNotificationString`, which round-trips through the Swift bridge and preserves key + arguments. |
| Trigger | `UNNotificationTrigger`, `UNTimeIntervalNotificationTrigger`, `UNCalendarNotificationTrigger`, push-trigger decoding | ✅ | Exposed as `NotificationTrigger`, `TimeIntervalTrigger`, `CalendarTrigger`, and `DateComponents`. |
| Category | `UNNotificationCategory` | ✅ | Includes options, hidden-preview placeholder, and category summary format. |
| Action | `UNNotificationAction`, `UNTextInputNotificationAction`, `UNNotificationActionIcon` | ✅ | Exposed as `NotificationAction`, `NotificationActionOptions`, and `NotificationActionIcon`. |
| Attachment | `UNNotificationAttachment` and attachment option keys | ✅ | Exposed as `NotificationAttachment`, `NotificationAttachmentOptions`, and the thumbnail/type-hint constants. |
| Response | `UNNotificationResponse`, `UNTextInputNotificationResponse`, default/dismiss identifiers, will-present presentation options | ✅ | Exposed as `NotificationResponse`, `default_action_identifier`, `dismiss_action_identifier`, and `NotificationPresentationOptions`. |
| Settings | `UNNotificationSettings`, authorization/status enums, preview style, alert style | ✅ | Exposed as `NotificationSettings`, `AuthorizationOptions`, `AuthorizationStatus`, `NotificationSetting`, `AlertStyle`, and `ShowPreviewsSetting`. |
| Error constants | `UNErrorDomain`, `UNErrorCode` | ✅ | Exposed as `USER_NOTIFICATIONS_ERROR_DOMAIN` and `UserNotificationsFrameworkErrorCode`. Bridge/runtime failures still surface as `UserNotificationsError`. |
| Service extension | `UNNotificationServiceExtension` | 🧪 | Exposed as `NotificationServiceExtensionSimulator` plus callback/handler traits. |
| Content extension | `UNNotificationContentExtension`, response options, play/pause metadata, extension context notification actions | 🧪 | Exposed as `NotificationContentExtensionSimulator`, `NotificationContentExtensionContext`, and related enums/structs. |

## Explicit macOS skips

| Apple SDK surface | Status | Reason |
| --- | --- | --- |
| `UNLocationNotificationTrigger` | 🚫 | Apple does not expose it as a supported macOS API. |
| `launchImageName` | 🚫 | `UNNotificationContent` marks it `API_UNAVAILABLE(macos)`. |
| `targetContentIdentifier` | 🚫 | Apple only declares it for iOS. |
| `carPlaySetting` | 🚫 | `UNNotificationSettings` marks it `API_UNAVAILABLE(macos)`. |
| `announcementSetting` | 🚫 | `UNNotificationSettings` marks it unavailable on macOS. |

## Verification assets

- Examples: `examples/01_smoke.rs` through `examples/11_content_extension_simulator.rs`
- Integration tests: `tests/*_tests.rs`
- Validation commands used for v0.2.1: `cargo test --quiet`, `cargo clippy --all-targets -- -D warnings`, and running every example under `examples/`
