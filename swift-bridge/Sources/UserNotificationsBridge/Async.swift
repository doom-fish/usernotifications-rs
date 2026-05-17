// Async APIs for UserNotifications framework

import Foundation
import UserNotifications

// MARK: - Callback typedef

public typealias UNAsyncCallback = @convention(c) (
    UnsafeRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer
) -> Void

// MARK: - Authorization Request

@available(macOS 12, *)
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

    Task {
        do {
            let granted = try await center.requestAuthorization(
                options: UNAuthorizationOptions(rawValue: UInt(options))
            )
            let boolValue = granted ? 1 as UInt8 : 0 as UInt8
            let holder = Unmanaged.passRetained(NSNumber(value: boolValue))
            callback(holder.toOpaque(), nil, context)
        } catch {
            error.localizedDescription.withCString { callback(nil, $0, context) }
        }
    }
}

// MARK: - Add Notification Request

@available(macOS 12, *)
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

    Task {
        do {
            let payload = try un_decode_json(requestJson, as: UNNotificationRequestPayload.self)
            let request = try un_make_request(payload)
            try await center.add(request)
            callback(nil, nil, context)
        } catch {
            error.localizedDescription.withCString { callback(nil, $0, context) }
        }
    }
}

// MARK: - Get Delivered Notifications

@available(macOS 12, *)
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

    Task {
        do {
            let notifications = try await center.deliveredNotifications()
            let payloads = notifications.map { UNNotificationPayload(
                date: $0.date.timeIntervalSince1970,
                request: un_request_payload($0.request)
            ) }
            let jsonString = un_encode_json(payloads)
            let holder = Unmanaged.passRetained(jsonString as NSString)
            callback(holder.toOpaque(), nil, context)
        } catch {
            error.localizedDescription.withCString { callback(nil, $0, context) }
        }
    }
}

// MARK: - Get Pending Notification Requests

@available(macOS 12, *)
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

    Task {
        do {
            let requests = try await center.pendingNotificationRequests()
            let payloads = requests.map { un_request_payload($0) }
            let jsonString = un_encode_json(payloads)
            let holder = Unmanaged.passRetained(jsonString as NSString)
            callback(holder.toOpaque(), nil, context)
        } catch {
            error.localizedDescription.withCString { callback(nil, $0, context) }
        }
    }
}

// MARK: - Get Notification Categories

@available(macOS 12, *)
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

    Task {
        do {
            let categories = try await center.notificationCategories()
            let payloads = Array(categories)
                .sorted { $0.identifier < $1.identifier }
                .map { un_category_payload($0) }
            let jsonString = un_encode_json(payloads)
            let holder = Unmanaged.passRetained(jsonString as NSString)
            callback(holder.toOpaque(), nil, context)
        } catch {
            error.localizedDescription.withCString { callback(nil, $0, context) }
        }
    }
}

// MARK: - Get Notification Settings

@available(macOS 12, *)
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

    Task {
        do {
            let settings = try await center.notificationSettings()
            let payload = un_settings_payload(settings)
            let jsonString = un_encode_json(payload)
            let holder = Unmanaged.passRetained(jsonString as NSString)
            callback(holder.toOpaque(), nil, context)
        } catch {
            error.localizedDescription.withCString { callback(nil, $0, context) }
        }
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
