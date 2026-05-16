import Foundation
import UserNotifications

struct UNNotificationRequestPayload: Codable {
    var identifier: String
    var content: UNNotificationContentPayload
    var trigger: UNNotificationTriggerPayload?
}

func un_make_request(_ payload: UNNotificationRequestPayload) throws -> UNNotificationRequest {
    UNNotificationRequest(
        identifier: payload.identifier,
        content: try un_make_content(payload.content),
        trigger: try un_make_trigger(payload.trigger)
    )
}

func un_request_payload(
    _ request: UNNotificationRequest,
    source: UNNotificationRequestPayload? = nil
) -> UNNotificationRequestPayload {
    UNNotificationRequestPayload(
        identifier: request.identifier,
        content: un_content_payload(request.content, source: source?.content),
        trigger: request.trigger.map { _ in un_trigger_payload(request.trigger) }
    )
}

@_cdecl("un_request_roundtrip_json")
public func un_request_roundtrip_json(
    _ requestJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let payload = try un_decode_json(requestJSON, as: UNNotificationRequestPayload.self)
        let request = try un_make_request(payload)
        return un_string(un_encode_json(un_request_payload(request, source: payload)))
    } catch {
        un_write_error(errorOut, error: error)
        return nil
    }
}
