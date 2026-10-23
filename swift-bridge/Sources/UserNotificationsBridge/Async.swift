// Async APIs for UserNotifications framework

import Foundation
import UserNotifications

// MARK: - Callback typedef

public typealias UNAsyncCallback = @convention(c) (
    UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer
) -> Void

// MARK: - Authorization Request

@_cdecl("un_center_request_authorization_async")
public func un_center_request_authorization_async(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ options: UInt64,
    _ callback: UNAsyncCallback,
    _ context: UnsafeMutableRawPointer
) {
    guard let center = un_center_unwrap(centerPtr) else {
        "notification center must not be null".withCString { callback(nil, $0, context) }
        return
    }

    center.requestAuthorization(
        options: UNAuthorizationOptions(rawValue: UInt(truncatingIfNeeded: options))
    ) { granted, error in
        if let error {
            error.localizedDescription.withCString { callback(nil, $0, context) }
        } else {
            // The Rust callback reads a single byte from `result` and does not
            // free it, so pass a pointer to a stack byte that is valid for the
            // synchronous duration of the callback (no allocation, no leak).
            var boolValue: UInt8 = granted ? 1 : 0
            withUnsafePointer(to: &boolValue) {
                callback(UnsafeRawPointer($0), nil, context)
            }
        }
    }
}

func un_add_notification_request(
    _ requestJson: UnsafePointer<CChar>,
    using add: (UNNotificationRequest, @escaping (Error?) -> Void) -> Void,
    completion: @escaping (Error?) -> Void
) {
    do {
        let request = try un_make_request(
            un_decode_json(requestJson, as: UNNotificationRequestPayload.self)
        )
        add(request, completion)
    } catch {
        completion(error)
    }
}

// MARK: - Add Notification Request

@_cdecl("un_center_add_notification_request_async")
public func un_center_add_notification_request_async(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ requestJson: UnsafePointer<CChar>,
    _ callback: UNAsyncCallback,
    _ context: UnsafeMutableRawPointer
) {
    guard let center = un_center_unwrap(centerPtr) else {
        "notification center must not be null".withCString { callback(nil, $0, context) }
        return
    }

    un_add_notification_request(requestJson, using: { request, completion in
        center.add(request, withCompletionHandler: completion)
    }) { error in
        if let error {
            error.localizedDescription.withCString { callback(nil, $0, context) }
        } else {
            callback(nil, nil, context)
        }
    }
}

// MARK: - Get Delivered Notifications

@_cdecl("un_center_get_delivered_notifications_async")
public func un_center_get_delivered_notifications_async(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ callback: UNAsyncCallback,
    _ context: UnsafeMutableRawPointer
) {
    guard let center = un_center_unwrap(centerPtr) else {
        "notification center must not be null".withCString { callback(nil, $0, context) }
        return
    }

    center.getDeliveredNotifications { notifications in
        let payloads = notifications.map { UNNotificationPayload(
            date: $0.date.timeIntervalSince1970,
            request: un_request_payload($0.request)
        ) }
        let jsonString = un_encode_json(payloads)
        // Rust takes ownership of this C string and frees it, matching the
        // synchronous bridge's un_string/strdup convention.
        let cString = un_string(jsonString)
        callback(cString.map(UnsafeRawPointer.init), nil, context)
    }
}

// MARK: - Get Pending Notification Requests

@_cdecl("un_center_get_pending_notification_requests_async")
public func un_center_get_pending_notification_requests_async(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ callback: UNAsyncCallback,
    _ context: UnsafeMutableRawPointer
) {
    guard let center = un_center_unwrap(centerPtr) else {
        "notification center must not be null".withCString { callback(nil, $0, context) }
        return
    }

    center.getPendingNotificationRequests { requests in
        let payloads = requests.map { un_request_payload($0) }
        let jsonString = un_encode_json(payloads)
        // Rust takes ownership of this C string and frees it, matching the
        // synchronous bridge's un_string/strdup convention.
        let cString = un_string(jsonString)
        callback(cString.map(UnsafeRawPointer.init), nil, context)
    }
}

// MARK: - Get Notification Categories

@_cdecl("un_center_get_notification_categories_async")
public func un_center_get_notification_categories_async(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ callback: UNAsyncCallback,
    _ context: UnsafeMutableRawPointer
) {
    guard let center = un_center_unwrap(centerPtr) else {
        "notification center must not be null".withCString { callback(nil, $0, context) }
        return
    }

    center.getNotificationCategories { categories in
        let payloads = Array(categories)
            .sorted { $0.identifier < $1.identifier }
            .map { un_category_payload($0) }
        let jsonString = un_encode_json(payloads)
        // Rust takes ownership of this C string and frees it, matching the
        // synchronous bridge's un_string/strdup convention.
        let cString = un_string(jsonString)
        callback(cString.map(UnsafeRawPointer.init), nil, context)
    }
}

// MARK: - Get Notification Settings

@_cdecl("un_center_get_notification_settings_async")
public func un_center_get_notification_settings_async(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ callback: UNAsyncCallback,
    _ context: UnsafeMutableRawPointer
) {
    guard let center = un_center_unwrap(centerPtr) else {
        "notification center must not be null".withCString { callback(nil, $0, context) }
        return
    }

    center.getNotificationSettings { settings in
        let payload = un_settings_payload(settings)
        let jsonString = un_encode_json(payload)
        // Rust takes ownership of this C string and frees it, matching the
        // synchronous bridge's un_string/strdup convention.
        let cString = un_string(jsonString)
        callback(cString.map(UnsafeRawPointer.init), nil, context)
    }
}

// MARK: - Helper Functions

private func un_request_payload(_ request: UNNotificationRequest) -> UNNotificationRequestPayload {
    UNNotificationRequestPayload(
        identifier: request.identifier,
        content: un_content_payload(request.content),
        trigger: request.trigger.map(un_trigger_payload)
    )
}
