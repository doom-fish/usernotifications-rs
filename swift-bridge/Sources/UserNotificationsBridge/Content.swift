import Foundation
import UserNotifications

struct UNNotificationSoundPayload: Codable {
    var kind: String
    var name: String?
    var volume: Float?
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
    var attachments: [UNNotificationAttachmentPayload]
    var summary_argument: String
    var summary_argument_count: UInt64
    var interruption_level: Int32?
    var relevance_score: Double?
    var filter_criteria: String?
    var localized_title: UNLocalizedStringPayload?
    var localized_subtitle: UNLocalizedStringPayload?
    var localized_body: UNLocalizedStringPayload?
    var localized_summary_argument: UNLocalizedStringPayload?
}

private func un_make_sound(_ payload: UNNotificationSoundPayload?) throws -> UNNotificationSound? {
    guard let payload else {
        return nil
    }

    switch payload.kind {
    case "default":
        return .default
    case "defaultCritical":
        return .defaultCritical
    case "defaultCriticalWithVolume":
        guard let volume = payload.volume else {
            throw NSError(domain: "usernotifications-rs", code: Int(UNR_INVALID_ARGUMENT), userInfo: [
                NSLocalizedDescriptionKey: "default critical sounds require a volume",
            ])
        }
        return .defaultCriticalSound(withAudioVolume: volume)
    case "named":
        guard let name = payload.name, !name.isEmpty else {
            throw NSError(domain: "usernotifications-rs", code: Int(UNR_INVALID_ARGUMENT), userInfo: [
                NSLocalizedDescriptionKey: "named notification sounds require a file name",
            ])
        }
        return .init(named: .init(rawValue: name))
    case "criticalNamed":
        guard let name = payload.name, !name.isEmpty else {
            throw NSError(domain: "usernotifications-rs", code: Int(UNR_INVALID_ARGUMENT), userInfo: [
                NSLocalizedDescriptionKey: "critical named notification sounds require a file name",
            ])
        }
        if let volume = payload.volume {
            return .criticalSoundNamed(.init(rawValue: name), withAudioVolume: volume)
        }
        return .criticalSoundNamed(.init(rawValue: name))
    default:
        throw NSError(domain: "usernotifications-rs", code: Int(UNR_INVALID_ARGUMENT), userInfo: [
            NSLocalizedDescriptionKey: "unsupported notification sound kind: \(payload.kind)",
        ])
    }
}

private func un_sound_payload(_ sound: UNNotificationSound?) -> UNNotificationSoundPayload? {
    guard let sound else {
        return nil
    }
    if sound.isEqual(UNNotificationSound.default) {
        return UNNotificationSoundPayload(kind: "default", name: nil, volume: nil)
    }
    if sound.isEqual(UNNotificationSound.defaultCritical) {
        return UNNotificationSoundPayload(kind: "defaultCritical", name: nil, volume: nil)
    }
    return UNNotificationSoundPayload(kind: "unknown", name: nil, volume: nil)
}

func un_make_content(_ payload: UNNotificationContentPayload) throws -> UNMutableNotificationContent {
    if payload.summary_argument_count == 0 {
        throw NSError(domain: "usernotifications-rs", code: Int(UNR_INVALID_ARGUMENT), userInfo: [
            NSLocalizedDescriptionKey: "summary_argument_count must be at least 1",
        ])
    }
    if let relevanceScore = payload.relevance_score, !(0.0 ... 1.0).contains(relevanceScore) {
        throw NSError(domain: "usernotifications-rs", code: Int(UNR_INVALID_ARGUMENT), userInfo: [
            NSLocalizedDescriptionKey: "relevance_score must be between 0.0 and 1.0",
        ])
    }

    let content = UNMutableNotificationContent()
    content.title = un_make_localized_string(payload.localized_title, fallback: payload.title)
    content.subtitle = un_make_localized_string(payload.localized_subtitle, fallback: payload.subtitle)
    content.body = un_make_localized_string(payload.localized_body, fallback: payload.body)

    if let badge = payload.badge {
        content.badge = NSNumber(value: badge)
    }
    content.categoryIdentifier = payload.category_identifier
    content.threadIdentifier = payload.thread_identifier
    if let userInfo = payload.user_info {
        content.userInfo = userInfo.mapValues(\.foundationObject)
    }
    if let sound = try un_make_sound(payload.sound) {
        content.sound = sound
    }
    content.attachments = try payload.attachments.map(un_make_attachment)
    content.summaryArgument = un_make_localized_string(
        payload.localized_summary_argument,
        fallback: payload.summary_argument
    )
    content.summaryArgumentCount = Int(payload.summary_argument_count)
    if #available(macOS 12.0, *), let interruptionLevel = payload.interruption_level {
        content.interruptionLevel = UNNotificationInterruptionLevel(rawValue: UInt(interruptionLevel))
            ?? .active
    }
    if #available(macOS 12.0, *), let relevanceScore = payload.relevance_score {
        content.relevanceScore = relevanceScore
    }
    if #available(macOS 13.0, *), let filterCriteria = payload.filter_criteria {
        content.filterCriteria = filterCriteria
    }
    return content
}

func un_content_payload(
    _ content: UNNotificationContent,
    source: UNNotificationContentPayload? = nil
) -> UNNotificationContentPayload {
    let userInfo: [String: UNJSONValue]? = {
        guard !content.userInfo.isEmpty else {
            return nil
        }
        var encoded: [String: UNJSONValue] = [:]
        for (key, value) in content.userInfo {
            encoded[String(describing: key)] = UNJSONValue.fromFoundationObject(value)
        }
        return encoded
    }()
    let attachments = zip(content.attachments, source?.attachments ?? []).map {
        un_attachment_payload($0.0, source: $0.1)
    }
    let encodedAttachments: [UNNotificationAttachmentPayload]
    if attachments.isEmpty {
        encodedAttachments = content.attachments.map { un_attachment_payload($0) }
    } else {
        encodedAttachments = attachments
    }

    return UNNotificationContentPayload(
        title: content.title,
        subtitle: content.subtitle,
        body: content.body,
        badge: content.badge?.int64Value,
        category_identifier: content.categoryIdentifier,
        thread_identifier: content.threadIdentifier,
        user_info: userInfo,
        sound: un_sound_payload(content.sound),
        attachments: encodedAttachments,
        summary_argument: content.summaryArgument,
        summary_argument_count: UInt64(content.summaryArgumentCount),
        interruption_level: {
            if #available(macOS 12.0, *) {
                return Int32(content.interruptionLevel.rawValue)
            }
            return nil
        }(),
        relevance_score: {
            if #available(macOS 12.0, *) {
                return content.relevanceScore
            }
            return nil
        }(),
        filter_criteria: {
            if #available(macOS 13.0, *) {
                return content.filterCriteria
            }
            return nil
        }(),
        localized_title: source?.localized_title,
        localized_subtitle: source?.localized_subtitle,
        localized_body: source?.localized_body,
        localized_summary_argument: source?.localized_summary_argument
    )
}

@_cdecl("un_content_roundtrip_json")
public func un_content_roundtrip_json(
    _ contentJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let payload = try un_decode_json(contentJSON, as: UNNotificationContentPayload.self)
        let content = try un_make_content(payload)
        return un_string(un_encode_json(un_content_payload(content, source: payload)))
    } catch {
        un_write_error(errorOut, error: error)
        return nil
    }
}
