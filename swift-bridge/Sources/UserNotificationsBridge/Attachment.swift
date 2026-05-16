import CoreGraphics
import Foundation
import UserNotifications

struct UNAttachmentThumbnailClippingRectPayload: Codable {
    var x: Double
    var y: Double
    var width: Double
    var height: Double
}

enum UNAttachmentThumbnailTimePayload: Codable {
    case Seconds(Double)
    case Frame(UInt64)
}

struct UNNotificationAttachmentOptionsPayload: Codable {
    var type_hint: String?
    var thumbnail_hidden: Bool
    var thumbnail_clipping_rect: UNAttachmentThumbnailClippingRectPayload?
    var thumbnail_time: UNAttachmentThumbnailTimePayload?
}

struct UNNotificationAttachmentPayload: Codable {
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

@_cdecl("un_attachment_roundtrip_json")
public func un_attachment_roundtrip_json(
    _ attachmentJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let payload = try un_decode_json(attachmentJSON, as: UNNotificationAttachmentPayload.self)
        let attachment = try un_make_attachment(payload)
        return un_string(un_encode_json(un_attachment_payload(attachment, source: payload)))
    } catch {
        un_write_error(errorOut, error: error)
        return nil
    }
}
