import Foundation
import UserNotifications

struct UNNotificationResponsePayload: Codable {
    var action_identifier: String
    var notification: UNNotificationPayload
    var user_text: String?
}

func un_response_payload(_ response: UNNotificationResponse) -> UNNotificationResponsePayload {
    UNNotificationResponsePayload(
        action_identifier: response.actionIdentifier,
        notification: un_notification_payload(response.notification),
        user_text: (response as? UNTextInputNotificationResponse)?.userText
    )
}

@_cdecl("un_response_get_default_action_identifier")
public func un_response_get_default_action_identifier() -> UnsafeMutablePointer<CChar>? {
    un_string(UNNotificationDefaultActionIdentifier)
}

@_cdecl("un_response_get_dismiss_action_identifier")
public func un_response_get_dismiss_action_identifier() -> UnsafeMutablePointer<CChar>? {
    un_string(UNNotificationDismissActionIdentifier)
}
