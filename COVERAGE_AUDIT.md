# usernotifications-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 41
VERIFIED: 39
GAPS: 2
EXEMPT: 0
COVERAGE_PCT: 95.1%

Top-level ObjC declarations in `UserNotifications.framework/Headers/*.h` were audited per the provided rules. `API_UNAVAILABLE(macos)` and other non-macOS-only symbols were filtered out, so items like `UNLocationNotificationTrigger` and iOS-only members such as `launchImageName`, `targetContentIdentifier`, `carPlaySetting`, and `announcementSetting` are excluded from the counts below. No surviving top-level declarations were macOS-deprecated, so `EXEMPT` is `0`.

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
| `UNNotificationCategory` | interface | `UNNotificationCategory.h` | `NotificationCategory` |
| `UNNotificationCategoryOptions` | typedef options | `UNNotificationCategory.h` | `NotificationCategoryOptions` |
| `UNNotificationContent` | interface | `UNNotificationContent.h` | `NotificationContent` |
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
| `UNNotificationAttributedMessageContext` | interface | `UNNotificationAttributedMessageContext.h` | No Rust wrapper; the macOS 15 message-context API depends on Intents types such as `INSendMessageIntent`. |
| `UNNotificationContentProviding` | protocol | `UNNotificationContent.h` | No public Rust trait/helper for provider-driven content specialization via `UNNotificationContent.updating(from:)`. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| _None_ | - | - | - | - |
