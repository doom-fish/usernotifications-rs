import Foundation
import UserNotifications

struct UNNotificationSettingsPayload: Codable {
    var authorization_status: Int32
    var sound_setting: Int32
    var badge_setting: Int32
    var alert_setting: Int32
    var notification_center_setting: Int32
    var lock_screen_setting: Int32
    var alert_style: Int32
    var show_previews_setting: Int32
    var critical_alert_setting: Int32
    var provides_app_notification_settings: Bool
    var time_sensitive_setting: Int32
    var scheduled_delivery_setting: Int32
    var direct_messages_setting: Int32
}

func un_settings_payload(_ settings: UNNotificationSettings) -> UNNotificationSettingsPayload {
    UNNotificationSettingsPayload(
        authorization_status: Int32(settings.authorizationStatus.rawValue),
        sound_setting: Int32(settings.soundSetting.rawValue),
        badge_setting: Int32(settings.badgeSetting.rawValue),
        alert_setting: Int32(settings.alertSetting.rawValue),
        notification_center_setting: Int32(settings.notificationCenterSetting.rawValue),
        lock_screen_setting: Int32(settings.lockScreenSetting.rawValue),
        alert_style: Int32(settings.alertStyle.rawValue),
        show_previews_setting: Int32(settings.showPreviewsSetting.rawValue),
        critical_alert_setting: Int32(settings.criticalAlertSetting.rawValue),
        provides_app_notification_settings: settings.providesAppNotificationSettings,
        time_sensitive_setting: {
            if #available(macOS 12.0, *) {
                return Int32(settings.timeSensitiveSetting.rawValue)
            }
            return 0
        }(),
        scheduled_delivery_setting: {
            if #available(macOS 12.0, *) {
                return Int32(settings.scheduledDeliverySetting.rawValue)
            }
            return 0
        }(),
        direct_messages_setting: {
            if #available(macOS 12.0, *) {
                return Int32(settings.directMessagesSetting.rawValue)
            }
            return 0
        }()
    )
}
