import Foundation
import UserNotifications

struct UNNotificationPayload: Codable {
    var date: Double
    var request: UNNotificationRequestPayload
}

func un_notification_payload(_ notification: UNNotification) -> UNNotificationPayload {
    UNNotificationPayload(
        date: notification.date.timeIntervalSince1970,
        request: un_request_payload(notification.request)
    )
}
