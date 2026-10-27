import UserNotificationsABI
import CoreGraphics
import Foundation
import UserNotifications

struct UNAttachmentThumbnailClippingRectPayload {
    var x: Double
    var y: Double
    var width: Double
    var height: Double
}

enum UNAttachmentThumbnailTimePayload {
    case Seconds(Double)
    case Frame(UInt64)
}

extension UNAttachmentThumbnailTimePayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNAttachmentThumbnailTimePayloadRaw {
        switch self {
        case .Seconds(let seconds):
            return UNAttachmentThumbnailTimePayloadRaw(tag: 1, seconds: seconds, frame: 0)
        case .Frame(let frame):
            return UNAttachmentThumbnailTimePayloadRaw(tag: 2, seconds: 0, frame: frame)
        }
    }

    static func read(_ raw: UNAttachmentThumbnailTimePayloadRaw) throws -> Self {
        switch raw.tag {
        case 1: return .Seconds(raw.seconds)
        case 2: return .Frame(raw.frame)
        default: throw un_binary_error()
        }
    }
}

struct UNNotificationAttachmentOptionsPayload {
    var type_hint: String?
    var thumbnail_hidden: Bool
    var thumbnail_clipping_rect: UNAttachmentThumbnailClippingRectPayload?
    var thumbnail_time: UNAttachmentThumbnailTimePayload?
}

struct UNNotificationAttachmentPayload {
    var identifier: String
    var file_path: String
    var attachment_type: String?
    var options: UNNotificationAttachmentOptionsPayload
}

private func un_make_attachment_options(
    _ payload: UNNotificationAttachmentOptionsPayload
) -> [AnyHashable: Any] {
    var options: [AnyHashable: Any] = [:]
    if let typeHint = payload.type_hint, !typeHint.isEmpty {
        options[UNNotificationAttachmentOptionsTypeHintKey] = typeHint
    }
    if payload.thumbnail_hidden {
        options[UNNotificationAttachmentOptionsThumbnailHiddenKey] = NSNumber(value: true)
    }
    if let rect = payload.thumbnail_clipping_rect {
        let clippingRect = CGRect(
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height
        )
        options[UNNotificationAttachmentOptionsThumbnailClippingRectKey] =
            CGRectCreateDictionaryRepresentation(clippingRect) as NSDictionary
    }
    if let thumbnailTime = payload.thumbnail_time {
        switch thumbnailTime {
        case .Seconds(let seconds):
            options[UNNotificationAttachmentOptionsThumbnailTimeKey] = NSNumber(value: seconds)
        case .Frame(let frame):
            options[UNNotificationAttachmentOptionsThumbnailTimeKey] = NSNumber(value: frame)
        }
    }
    return options
}

func un_make_attachment(_ payload: UNNotificationAttachmentPayload) throws -> UNNotificationAttachment {
    guard !payload.file_path.isEmpty else {
        throw NSError(domain: "usernotifications-rs", code: Int(UNR_INVALID_ARGUMENT), userInfo: [
            NSLocalizedDescriptionKey: "notification attachment file_path must not be empty",
        ])
    }

    return try UNNotificationAttachment(
        identifier: payload.identifier,
        url: URL(fileURLWithPath: payload.file_path),
        options: un_make_attachment_options(payload.options)
    )
}

private func un_attachment_options_payload(
    _ payload: UNNotificationAttachmentOptionsPayload?
) -> UNNotificationAttachmentOptionsPayload {
    payload ?? UNNotificationAttachmentOptionsPayload(
        type_hint: nil,
        thumbnail_hidden: false,
        thumbnail_clipping_rect: nil,
        thumbnail_time: nil
    )
}

func un_attachment_payload(
    _ attachment: UNNotificationAttachment,
    source: UNNotificationAttachmentPayload? = nil
) -> UNNotificationAttachmentPayload {
    UNNotificationAttachmentPayload(
        identifier: attachment.identifier,
        file_path: attachment.url.path,
        attachment_type: attachment.type,
        options: un_attachment_options_payload(source?.options)
    )
}

@_cdecl("un_attachment_roundtrip_binary")
public func un_attachment_roundtrip_binary(
    _ attachmentBinary: UnsafePointer<UNBufferRaw>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<UNBufferRaw>? {
    do {
        let payload = try un_decode_binary(attachmentBinary, as: UNNotificationAttachmentPayload.self)
        let attachment = try un_make_attachment(payload)
        return un_encode_binary(un_attachment_payload(attachment, source: payload))
    } catch {
        un_write_error(errorOut, error: error)
        return nil
    }
}
