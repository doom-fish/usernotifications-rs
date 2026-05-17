# usernotifications-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 41
VERIFIED: 41
GAPS: 0
EXEMPT: 0
COVERAGE_PCT: 100.0%

This audit exhaustively enumerates all public macOS-available symbols in `UserNotifications.framework/Headers/*.h` against MacOSX26.2.sdk. The framework contains 42 top-level ObjC declarations; `UNLocationNotificationTrigger` (API_UNAVAILABLE(macos)) is excluded as iOS-only. The remaining 41 symbols are all wrapped by the crate either via Swift bridge or safe Rust API. No gaps or deprecated symbols remain unwrapped.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `NSString (UNUserNotificationCenterSupport)` | category | `NSString+UserNotifications.h` | `LocalizedNotificationString` |
| `UNAlertStyle` | typedef enum | `UNNotificationSettings.h` | `AlertStyle` |
| `UNAuthorizationOptions` | typedef options | `UNUserNotificationCenter.h` | `AuthorizationOptions` |
| `UNAuthorizationStatus` | typedef enum | `UNNotificationSettings.h` | `AuthorizationStatus` |
| `UNCalendarNotificationTrigger` | interface | `UNNotificationTrigger.h` | `CalendarTrigger`, `NotificationTrigger::Calendar` |
| `UNErrorCode` | typedef enum | `UNError.h` | `UserNotificationsFrameworkErrorCode` |
| `UNErrorDomain` | extern const | `UNError.h` | `USER_NOTIFICATIONS_ERROR_DOMAIN` |
| `UNMutableNotificationContent` | interface | `UNNotificationContent.h` | `NotificationContent` |
| `UNNotification` | interface | `UNNotification.h` | `Notification` |
| `UNNotificationAction` | interface | `UNNotificationAction.h` | `NotificationAction` |
| `UNNotificationActionIcon` | interface | `UNNotificationActionIcon.h` | `NotificationActionIcon` |
| `UNNotificationActionOptions` | typedef options | `UNNotificationAction.h` | `NotificationActionOptions` |
| `UNNotificationAttachment` | interface | `UNNotificationAttachment.h` | `NotificationAttachment` |
| `UNNotificationAttachmentOptionsThumbnailClippingRectKey` | extern const | `UNNotificationAttachment.h` | `ATTACHMENT_OPTIONS_THUMBNAIL_CLIPPING_RECT_KEY` |
| `UNNotificationAttachmentOptionsThumbnailHiddenKey` | extern const | `UNNotificationAttachment.h` | `ATTACHMENT_OPTIONS_THUMBNAIL_HIDDEN_KEY` |
| `UNNotificationAttachmentOptionsThumbnailTimeKey` | extern const | `UNNotificationAttachment.h` | `ATTACHMENT_OPTIONS_THUMBNAIL_TIME_KEY` |
| `UNNotificationAttachmentOptionsTypeHintKey` | extern const | `UNNotificationAttachment.h` | `ATTACHMENT_OPTIONS_TYPE_HINT_KEY` |
| `UNNotificationAttributedMessageContext` | interface | `UNNotificationAttributedMessageContext.h` | `NotificationAttributedMessageContext` |
| `UNNotificationCategory` | interface | `UNNotificationCategory.h` | `NotificationCategory` |
| `UNNotificationCategoryOptions` | typedef options | `UNNotificationCategory.h` | `NotificationCategoryOptions` |
| `UNNotificationContent` | interface | `UNNotificationContent.h` | `NotificationContent` |
| `UNNotificationContentProviding` | protocol | `UNNotificationContent.h` | `NotificationContentProviding`, `NotificationContent::updating_from` |
| `UNNotificationDefaultActionIdentifier` | extern const | `UNNotificationResponse.h` | `default_action_identifier()` |
| `UNNotificationDismissActionIdentifier` | extern const | `UNNotificationResponse.h` | `dismiss_action_identifier()` |
| `UNNotificationInterruptionLevel` | typedef enum | `UNNotificationContent.h` | `NotificationInterruptionLevel` |
| `UNNotificationPresentationOptions` | typedef options | `UNUserNotificationCenter.h` | `NotificationPresentationOptions` |
| `UNNotificationRequest` | interface | `UNNotificationRequest.h` | `NotificationRequest` |
| `UNNotificationResponse` | interface | `UNNotificationResponse.h` | `NotificationResponse` |
| `UNNotificationServiceExtension` | interface | `UNNotificationServiceExtension.h` | `NotificationServiceExtensionSimulator` |
| `UNNotificationSetting` | typedef enum | `UNNotificationSettings.h` | `NotificationSetting` |
| `UNNotificationSettings` | interface | `UNNotificationSettings.h` | `NotificationSettings` |
| `UNNotificationSound` | interface | `UNNotificationSound.h` | `NotificationSound` |
| `UNNotificationSoundName` | typedef string enum | `UNNotificationSound.h` | `NotificationSound::Named` / `NotificationSound::CriticalNamed` |
| `UNNotificationTrigger` | interface | `UNNotificationTrigger.h` | `NotificationTrigger` |
| `UNPushNotificationTrigger` | interface | `UNNotificationTrigger.h` | `NotificationTrigger::Push` |
| `UNShowPreviewsSetting` | typedef enum | `UNNotificationSettings.h` | `ShowPreviewsSetting` |
| `UNTextInputNotificationAction` | interface | `UNNotificationAction.h` | `NotificationAction::new_text_input` |
| `UNTextInputNotificationResponse` | interface | `UNNotificationResponse.h` | `NotificationResponse.user_text` |
| `UNTimeIntervalNotificationTrigger` | interface | `UNNotificationTrigger.h` | `TimeIntervalTrigger`, `NotificationTrigger::TimeInterval` |
| `UNUserNotificationCenter` | interface | `UNUserNotificationCenter.h` | `UserNotificationCenter` |
| `UNUserNotificationCenterDelegate` | protocol | `UNUserNotificationCenter.h` | `UserNotificationCenterDelegate`, `UserNotificationCenterCallbacks` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| _None_ | - | - | - |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `UNLocationNotificationTrigger` | interface | `UNNotificationTrigger.h` | iOS/watchOS-only, not available on macOS | `API_UNAVAILABLE(macos, tvos, macCatalyst, visionos)` |
