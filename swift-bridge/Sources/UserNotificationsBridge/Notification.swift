import Foundation
import UserNotifications

struct UNNotificationSoundPayload: Codable {
    var kind: String
    var name: String?
}

struct UNNotificationContentPayload: Codable {
    var title: String
    var subtitle: String
    var body: String
    var badge: Int64?
    var category_identifier: String
    var thread_identifier: String
    var user_info: [String: UNJSONValue]?
    var sound: UNNotificationSoundPayload?
}

struct UNDateComponentsPayload: Codable {
    var year: Int?
    var month: Int?
    var day: Int?
    var hour: Int?
    var minute: Int?
    var second: Int?
    var weekday: Int?
}

struct UNNotificationTriggerPayload: Codable {
    var kind: String
    var repeats: Bool?
    var time_interval: Double?
    var date_components: UNDateComponentsPayload?
    var next_trigger_date: Double?
    var class_name: String?
}

struct UNNotificationRequestPayload: Codable {
    var identifier: String
    var content: UNNotificationContentPayload
    var trigger: UNNotificationTriggerPayload?
}

struct UNNotificationActionPayload: Codable {
    var identifier: String
    var title: String
    var options: UInt64
    var text_input_button_title: String?
    var text_input_placeholder: String?
}

struct UNNotificationCategoryPayload: Codable {
    var identifier: String
    var actions: [UNNotificationActionPayload]
    var intent_identifiers: [String]
    var options: UInt64
}

private func un_invalid_argument(_ message: String) -> NSError {
    NSError(domain: "usernotifications-rs", code: Int(UNR_INVALID_ARGUMENT), userInfo: [
        NSLocalizedDescriptionKey: message,
    ])
}

private func un_make_sound(_ payload: UNNotificationSoundPayload?) throws -> UNNotificationSound? {
    guard let payload else {
        return nil
    }

    switch payload.kind {
    case "default":
        return .default
    case "named":
        guard let name = payload.name, !name.isEmpty else {
            throw un_invalid_argument("named notification sounds require a file name")
        }
        return .init(named: .init(rawValue: name))
    default:
        throw un_invalid_argument("unsupported notification sound kind: \(payload.kind)")
    }
}

private func un_make_content(_ payload: UNNotificationContentPayload) throws -> UNMutableNotificationContent {
    let content = UNMutableNotificationContent()
    content.title = payload.title
    content.subtitle = payload.subtitle
    content.body = payload.body

    if let badge = payload.badge {
        content.badge = NSNumber(value: badge)
    }
    if !payload.category_identifier.isEmpty {
        content.categoryIdentifier = payload.category_identifier
    }
    if !payload.thread_identifier.isEmpty {
        content.threadIdentifier = payload.thread_identifier
    }
    if let userInfo = payload.user_info {
        content.userInfo = userInfo.mapValues(\.foundationObject)
    }
    if let sound = try un_make_sound(payload.sound) {
        content.sound = sound
    }
    return content
}

private func un_make_trigger(_ payload: UNNotificationTriggerPayload?) throws -> UNNotificationTrigger? {
    guard let payload else {
        return nil
    }

    switch payload.kind {
    case "timeInterval":
        guard let timeInterval = payload.time_interval else {
            throw un_invalid_argument("time interval triggers require time_interval")
        }
        if payload.repeats == true && timeInterval < 60 {
            throw un_invalid_argument("repeating time interval triggers must be at least 60 seconds")
        }
        return UNTimeIntervalNotificationTrigger(timeInterval: timeInterval, repeats: payload.repeats ?? false)
    case "calendar":
        guard let dateComponents = payload.date_components else {
            throw un_invalid_argument("calendar triggers require date_components")
        }
        var components = DateComponents()
        components.year = dateComponents.year
        components.month = dateComponents.month
        components.day = dateComponents.day
        components.hour = dateComponents.hour
        components.minute = dateComponents.minute
        components.second = dateComponents.second
        components.weekday = dateComponents.weekday
        return UNCalendarNotificationTrigger(dateMatching: components, repeats: payload.repeats ?? false)
    case "push":
        throw un_invalid_argument("UNPushNotificationTrigger cannot be created locally")
    case "location":
        throw un_invalid_argument("UNLocationNotificationTrigger is unavailable on macOS")
    default:
        throw un_invalid_argument("unsupported notification trigger kind: \(payload.kind)")
    }
}

func un_make_request(_ payload: UNNotificationRequestPayload) throws -> UNNotificationRequest {
    UNNotificationRequest(
        identifier: payload.identifier,
        content: try un_make_content(payload.content),
        trigger: try un_make_trigger(payload.trigger)
    )
}

private func un_make_action(_ payload: UNNotificationActionPayload) -> UNNotificationAction {
    let options = UNNotificationActionOptions(rawValue: UInt(payload.options))
    if let textInputButtonTitle = payload.text_input_button_title,
       let textInputPlaceholder = payload.text_input_placeholder {
        return UNTextInputNotificationAction(
            identifier: payload.identifier,
            title: payload.title,
            options: options,
            textInputButtonTitle: textInputButtonTitle,
            textInputPlaceholder: textInputPlaceholder
        )
    }

    return UNNotificationAction(
        identifier: payload.identifier,
        title: payload.title,
        options: options
    )
}

func un_make_category(_ payload: UNNotificationCategoryPayload) -> UNNotificationCategory {
    UNNotificationCategory(
        identifier: payload.identifier,
        actions: payload.actions.map(un_make_action),
        intentIdentifiers: payload.intent_identifiers,
        options: UNNotificationCategoryOptions(rawValue: UInt(payload.options))
    )
}

private func un_date_components_object(_ components: DateComponents) -> [String: Any] {
    var object: [String: Any] = [:]
    if let year = components.year { object["year"] = year }
    if let month = components.month { object["month"] = month }
    if let day = components.day { object["day"] = day }
    if let hour = components.hour { object["hour"] = hour }
    if let minute = components.minute { object["minute"] = minute }
    if let second = components.second { object["second"] = second }
    if let weekday = components.weekday { object["weekday"] = weekday }
    return object
}

private func un_sound_object(_ sound: UNNotificationSound?) -> Any {
    guard let sound else {
        return NSNull()
    }
    if sound.isEqual(UNNotificationSound.default) {
        return ["kind": "default"]
    }
    return ["kind": "unknown"]
}

private func un_content_object(_ content: UNNotificationContent) -> [String: Any] {
    var object: [String: Any] = [
        "title": content.title,
        "subtitle": content.subtitle,
        "body": content.body,
        "category_identifier": content.categoryIdentifier,
        "thread_identifier": content.threadIdentifier,
    ]
    if let badge = content.badge {
        object["badge"] = badge.int64Value
    }
    if !content.userInfo.isEmpty {
        object["user_info"] = un_json_safe(content.userInfo)
    }
    if content.sound != nil {
        object["sound"] = un_sound_object(content.sound)
    }
    return object
}

private func un_trigger_object(_ trigger: UNNotificationTrigger?) -> Any {
    guard let trigger else {
        return NSNull()
    }
    if let trigger = trigger as? UNTimeIntervalNotificationTrigger {
        var object: [String: Any] = [
            "kind": "timeInterval",
            "repeats": trigger.repeats,
            "time_interval": trigger.timeInterval,
        ]
        if let nextTriggerDate = trigger.nextTriggerDate()?.timeIntervalSince1970 {
            object["next_trigger_date"] = nextTriggerDate
        }
        return object
    }
    if let trigger = trigger as? UNCalendarNotificationTrigger {
        var object: [String: Any] = [
            "kind": "calendar",
            "repeats": trigger.repeats,
            "date_components": un_date_components_object(trigger.dateComponents),
        ]
        if let nextTriggerDate = trigger.nextTriggerDate()?.timeIntervalSince1970 {
            object["next_trigger_date"] = nextTriggerDate
        }
        return object
    }
    if trigger is UNPushNotificationTrigger {
        return [
            "kind": "push",
            "repeats": trigger.repeats,
        ]
    }
    return [
        "kind": "unknown",
        "class_name": NSStringFromClass(type(of: trigger)),
        "repeats": trigger.repeats,
    ]
}

private func un_action_object(_ action: UNNotificationAction) -> [String: Any] {
    var object: [String: Any] = [
        "identifier": action.identifier,
        "title": action.title,
        "options": action.options.rawValue,
    ]
    if let textAction = action as? UNTextInputNotificationAction {
        object["text_input_button_title"] = textAction.textInputButtonTitle
        object["text_input_placeholder"] = textAction.textInputPlaceholder
    }
    return object
}

func un_category_object(_ category: UNNotificationCategory) -> [String: Any] {
    [
        "identifier": category.identifier,
        "actions": category.actions.map(un_action_object),
        "intent_identifiers": category.intentIdentifiers,
        "options": category.options.rawValue,
    ]
}

func un_request_object(_ request: UNNotificationRequest) -> [String: Any] {
    [
        "identifier": request.identifier,
        "content": un_content_object(request.content),
        "trigger": un_trigger_object(request.trigger),
    ]
}

func un_notification_object(_ notification: UNNotification) -> [String: Any] {
    [
        "date": notification.date.timeIntervalSince1970,
        "request": un_request_object(notification.request),
    ]
}

func un_response_object(_ response: UNNotificationResponse) -> [String: Any] {
    var object: [String: Any] = [
        "action_identifier": response.actionIdentifier,
        "notification": un_notification_object(response.notification),
    ]
    if let textResponse = response as? UNTextInputNotificationResponse {
        object["user_text"] = textResponse.userText
    }
    return object
}

func un_settings_object(_ settings: UNNotificationSettings) -> [String: Any] {
    var object: [String: Any] = [
        "authorization_status": settings.authorizationStatus.rawValue,
        "sound_setting": settings.soundSetting.rawValue,
        "badge_setting": settings.badgeSetting.rawValue,
        "alert_setting": settings.alertSetting.rawValue,
        "notification_center_setting": settings.notificationCenterSetting.rawValue,
        "lock_screen_setting": settings.lockScreenSetting.rawValue,
        "alert_style": settings.alertStyle.rawValue,
        "show_previews_setting": settings.showPreviewsSetting.rawValue,
        "critical_alert_setting": settings.criticalAlertSetting.rawValue,
        "provides_app_notification_settings": settings.providesAppNotificationSettings,
    ]

    if #available(macOS 12.0, *) {
        object["time_sensitive_setting"] = settings.timeSensitiveSetting.rawValue
        object["scheduled_delivery_setting"] = settings.scheduledDeliverySetting.rawValue
        object["direct_messages_setting"] = settings.directMessagesSetting.rawValue
    } else {
        object["time_sensitive_setting"] = 0
        object["scheduled_delivery_setting"] = 0
        object["direct_messages_setting"] = 0
    }

    return object
}
