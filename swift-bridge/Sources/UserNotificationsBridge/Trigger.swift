import Foundation
import UserNotifications

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

private func un_date_components(_ payload: UNDateComponentsPayload) -> DateComponents {
    var components = DateComponents()
    components.year = payload.year
    components.month = payload.month
    components.day = payload.day
    components.hour = payload.hour
    components.minute = payload.minute
    components.second = payload.second
    components.weekday = payload.weekday
    return components
}

private func un_date_components_payload(_ components: DateComponents) -> UNDateComponentsPayload {
    UNDateComponentsPayload(
        year: components.year,
        month: components.month,
        day: components.day,
        hour: components.hour,
        minute: components.minute,
        second: components.second,
        weekday: components.weekday
    )
}

func un_make_trigger(_ payload: UNNotificationTriggerPayload?) throws -> UNNotificationTrigger? {
    guard let payload else {
        return nil
    }

    switch payload.kind {
    case "timeInterval":
        guard let timeInterval = payload.time_interval else {
            throw NSError(domain: "usernotifications-rs", code: Int(UNR_INVALID_ARGUMENT), userInfo: [
                NSLocalizedDescriptionKey: "time interval triggers require time_interval",
            ])
        }
        if payload.repeats == true && timeInterval < 60 {
            throw NSError(domain: "usernotifications-rs", code: Int(UNR_INVALID_ARGUMENT), userInfo: [
                NSLocalizedDescriptionKey: "repeating time interval triggers must be at least 60 seconds",
            ])
        }
        return UNTimeIntervalNotificationTrigger(
            timeInterval: timeInterval,
            repeats: payload.repeats ?? false
        )
    case "calendar":
        guard let dateComponents = payload.date_components else {
            throw NSError(domain: "usernotifications-rs", code: Int(UNR_INVALID_ARGUMENT), userInfo: [
                NSLocalizedDescriptionKey: "calendar triggers require date_components",
            ])
        }
        return UNCalendarNotificationTrigger(
            dateMatching: un_date_components(dateComponents),
            repeats: payload.repeats ?? false
        )
    case "push":
        throw NSError(domain: "usernotifications-rs", code: Int(UNR_INVALID_ARGUMENT), userInfo: [
            NSLocalizedDescriptionKey: "UNPushNotificationTrigger cannot be created locally",
        ])
    case "location":
        throw NSError(domain: "usernotifications-rs", code: Int(UNR_INVALID_ARGUMENT), userInfo: [
            NSLocalizedDescriptionKey: "UNLocationNotificationTrigger is unavailable on macOS",
        ])
    default:
        throw NSError(domain: "usernotifications-rs", code: Int(UNR_INVALID_ARGUMENT), userInfo: [
            NSLocalizedDescriptionKey: "unsupported notification trigger kind: \(payload.kind)",
        ])
    }
}

func un_trigger_payload(_ trigger: UNNotificationTrigger?) -> UNNotificationTriggerPayload {
    guard let trigger else {
        return UNNotificationTriggerPayload(
            kind: "none",
            repeats: nil,
            time_interval: nil,
            date_components: nil,
            next_trigger_date: nil,
            class_name: nil
        )
    }
    if let trigger = trigger as? UNTimeIntervalNotificationTrigger {
        return UNNotificationTriggerPayload(
            kind: "timeInterval",
            repeats: trigger.repeats,
            time_interval: trigger.timeInterval,
            date_components: nil,
            next_trigger_date: trigger.nextTriggerDate()?.timeIntervalSince1970,
            class_name: nil
        )
    }
    if let trigger = trigger as? UNCalendarNotificationTrigger {
        return UNNotificationTriggerPayload(
            kind: "calendar",
            repeats: trigger.repeats,
            time_interval: nil,
            date_components: un_date_components_payload(trigger.dateComponents),
            next_trigger_date: trigger.nextTriggerDate()?.timeIntervalSince1970,
            class_name: nil
        )
    }
    if trigger is UNPushNotificationTrigger {
        return UNNotificationTriggerPayload(
            kind: "push",
            repeats: trigger.repeats,
            time_interval: nil,
            date_components: nil,
            next_trigger_date: nil,
            class_name: nil
        )
    }
    return UNNotificationTriggerPayload(
        kind: "unknown",
        repeats: trigger.repeats,
        time_interval: nil,
        date_components: nil,
        next_trigger_date: nil,
        class_name: NSStringFromClass(type(of: trigger))
    )
}

@_cdecl("un_trigger_roundtrip_json")
public func un_trigger_roundtrip_json(
    _ triggerJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let payload = try un_decode_json(triggerJSON, as: UNNotificationTriggerPayload.self)
        guard payload.kind != "none" else {
            return un_string(un_encode_json(payload))
        }
        let trigger = try un_make_trigger(payload)
        return un_string(un_encode_json(un_trigger_payload(trigger)))
    } catch {
        un_write_error(errorOut, error: error)
        return nil
    }
}
