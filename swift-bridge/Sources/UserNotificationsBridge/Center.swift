import Dispatch
import Foundation
import UserNotifications

struct UNCenterEventPayload: Codable {
    var event: String
    var notification: UNNotificationPayload?
    var response: UNNotificationResponsePayload?
}

public typealias UNCenterEventCallback =
    @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void
public typealias UNCenterWillPresentCallback =
    @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> UInt64

private final class UNRustCenterDelegate: NSObject, UNUserNotificationCenterDelegate {
    let eventCallback: UNCenterEventCallback?
    let willPresentCallback: UNCenterWillPresentCallback?
    let userInfo: UnsafeMutableRawPointer?
    private var isActive = true

    init(
        eventCallback: UNCenterEventCallback?,
        willPresentCallback: UNCenterWillPresentCallback?,
        userInfo: UnsafeMutableRawPointer?
    ) {
        self.eventCallback = eventCallback
        self.willPresentCallback = willPresentCallback
        self.userInfo = userInfo
        super.init()
    }

    func deactivate() {
        isActive = false
    }

    private func send(_ payload: UNCenterEventPayload) {
        guard isActive, let eventCallback else { return }
        let json = un_encode_json(payload)
        json.withCString { eventCallback(userInfo, $0) }
    }

    func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        willPresent notification: UNNotification,
        withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void
    ) {
        guard isActive else {
            completionHandler([])
            return
        }
        guard let willPresentCallback else {
            completionHandler([])
            return
        }
        let json = un_encode_json(un_notification_payload(notification))
        let bits = json.withCString { willPresentCallback(userInfo, $0) }
        completionHandler(UNNotificationPresentationOptions(rawValue: UInt(bits)))
    }

    func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        didReceive response: UNNotificationResponse,
        withCompletionHandler completionHandler: @escaping () -> Void
    ) {
        send(UNCenterEventPayload(
            event: "didReceiveNotificationResponse",
            notification: nil,
            response: un_response_payload(response)
        ))
        completionHandler()
    }

    func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        openSettingsFor notification: UNNotification?
    ) {
        send(UNCenterEventPayload(
            event: "openSettingsForNotification",
            notification: notification.map(un_notification_payload),
            response: nil
        ))
    }
}

private final class UNUserNotificationCenterBox: NSObject {
    let center: UNUserNotificationCenter
    private var delegateBox: UNRustCenterDelegate?

    init(center: UNUserNotificationCenter, delegateBox: UNRustCenterDelegate? = nil) {
        self.center = center
        self.delegateBox = delegateBox
        super.init()
        self.center.delegate = delegateBox
    }

    func setDelegateBox(_ delegateBox: UNRustCenterDelegate?) {
        self.delegateBox?.deactivate()
        self.delegateBox = delegateBox
        center.delegate = delegateBox
    }

    deinit {
        delegateBox?.deactivate()
        center.delegate = nil
    }
}

private func un_center_box(_ ptr: UnsafeMutableRawPointer?) -> UNUserNotificationCenterBox? {
    guard let ptr else {
        return nil
    }
    let box: UNUserNotificationCenterBox = un_borrow(ptr)
    return box
}

// Public function for async APIs to access the center
func un_center_unwrap(_ ptr: UnsafeMutableRawPointer?) -> UNUserNotificationCenter? {
    un_center_box(ptr)?.center
}

private func un_current_notification_center(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UNUserNotificationCenter? {
    let bundleURL = Bundle.main.bundleURL
    guard bundleURL.pathExtension == "app", Bundle.main.bundleIdentifier != nil else {
        un_write_error(
            errorOut,
            "UserNotifications requires running from a macOS app bundle"
        )
        return nil
    }
    return .current()
}

@_cdecl("un_center_current")
public func un_center_current(
    _ outCenter: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outCenter.pointee = nil
    guard let center = un_current_notification_center(errorOut) else {
        return UNR_FRAMEWORK_ERROR
    }
    outCenter.pointee = un_retain(UNUserNotificationCenterBox(center: center))
    return UNR_OK
}

@_cdecl("un_center_set_delegate")
public func un_center_set_delegate(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ eventCallback: UNCenterEventCallback?,
    _ willPresentCallback: UNCenterWillPresentCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = un_center_box(centerPtr) else {
        un_write_error(errorOut, "notification center must not be null")
        return UNR_INVALID_ARGUMENT
    }

    let delegateBox = UNRustCenterDelegate(
        eventCallback: eventCallback,
        willPresentCallback: willPresentCallback,
        userInfo: userInfo
    )
    box.setDelegateBox(delegateBox)
    return UNR_OK
}

@_cdecl("un_center_clear_delegate")
public func un_center_clear_delegate(_ centerPtr: UnsafeMutableRawPointer?) {
    un_center_box(centerPtr)?.setDelegateBox(nil)
}

@_cdecl("un_center_request_authorization")
public func un_center_request_authorization(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ options: UInt64,
    _ outGranted: UnsafeMutablePointer<Bool>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let center = un_center_box(centerPtr)?.center else {
        un_write_error(errorOut, "notification center must not be null")
        outGranted.pointee = false
        return UNR_INVALID_ARGUMENT
    }

    let semaphore = DispatchSemaphore(value: 0)
    var granted = false
    var completionError: Error?

    center.requestAuthorization(options: UNAuthorizationOptions(rawValue: UInt(options))) {
        granted = $0
        completionError = $1
        semaphore.signal()
    }
    semaphore.wait()

    outGranted.pointee = granted
    if let completionError {
        un_write_error(errorOut, error: completionError)
        return UNR_FRAMEWORK_ERROR
    }
    return UNR_OK
}

@_cdecl("un_center_supports_content_extensions")
public func un_center_supports_content_extensions(_ centerPtr: UnsafeMutableRawPointer?) -> Bool {
    un_center_box(centerPtr)?.center.supportsContentExtensions ?? false
}

@_cdecl("un_center_set_badge_count")
public func un_center_set_badge_count(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ newBadgeCount: Int,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let center = un_center_box(centerPtr)?.center else {
        un_write_error(errorOut, "notification center must not be null")
        return UNR_INVALID_ARGUMENT
    }
    guard #available(macOS 13.0, *) else {
        un_write_error(errorOut, "setBadgeCount requires macOS 13 or newer")
        return UNR_FRAMEWORK_ERROR
    }

    let semaphore = DispatchSemaphore(value: 0)
    var completionError: Error?
    center.setBadgeCount(newBadgeCount) {
        completionError = $0
        semaphore.signal()
    }
    semaphore.wait()

    if let completionError {
        un_write_error(errorOut, error: completionError)
        return UNR_FRAMEWORK_ERROR
    }
    return UNR_OK
}

@_cdecl("un_center_get_notification_settings_json")
public func un_center_get_notification_settings_json(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let center = un_center_box(centerPtr)?.center else {
        un_write_error(errorOut, "notification center must not be null")
        return nil
    }

    let semaphore = DispatchSemaphore(value: 0)
    var payload: UNNotificationSettingsPayload?
    center.getNotificationSettings {
        payload = un_settings_payload($0)
        semaphore.signal()
    }
    semaphore.wait()

    return payload.map(un_encode_json).flatMap(un_string)
}

@_cdecl("un_center_set_notification_categories")
public func un_center_set_notification_categories(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ categoriesJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let center = un_center_box(centerPtr)?.center else {
        un_write_error(errorOut, "notification center must not be null")
        return UNR_INVALID_ARGUMENT
    }

    do {
        let payloads = try un_decode_json(categoriesJSON, as: [UNNotificationCategoryPayload].self)
        center.setNotificationCategories(Set(payloads.map(un_make_category)))
        return UNR_OK
    } catch {
        un_write_error(errorOut, error: error)
        return UNR_INVALID_ARGUMENT
    }
}

@_cdecl("un_center_get_notification_categories_json")
public func un_center_get_notification_categories_json(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let center = un_center_box(centerPtr)?.center else {
        un_write_error(errorOut, "notification center must not be null")
        return nil
    }

    let semaphore = DispatchSemaphore(value: 0)
    var payloads: [UNNotificationCategoryPayload] = []
    center.getNotificationCategories {
        payloads = Array($0)
            .sorted { $0.identifier < $1.identifier }
            .map { un_category_payload($0) }
        semaphore.signal()
    }
    semaphore.wait()
    return un_string(un_encode_json(payloads))
}

@_cdecl("un_center_add_request")
public func un_center_add_request(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ requestJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let center = un_center_box(centerPtr)?.center else {
        un_write_error(errorOut, "notification center must not be null")
        return UNR_INVALID_ARGUMENT
    }

    do {
        let payload = try un_decode_json(requestJSON, as: UNNotificationRequestPayload.self)
        let request = try un_make_request(payload)
        let semaphore = DispatchSemaphore(value: 0)
        var completionError: Error?
        center.add(request) {
            completionError = $0
            semaphore.signal()
        }
        semaphore.wait()
        if let completionError {
            un_write_error(errorOut, error: completionError)
            return UNR_FRAMEWORK_ERROR
        }
        return UNR_OK
    } catch {
        un_write_error(errorOut, error: error)
        return UNR_INVALID_ARGUMENT
    }
}

@_cdecl("un_center_get_pending_requests_json")
public func un_center_get_pending_requests_json(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let center = un_center_box(centerPtr)?.center else {
        un_write_error(errorOut, "notification center must not be null")
        return nil
    }

    let semaphore = DispatchSemaphore(value: 0)
    var payloads: [UNNotificationRequestPayload] = []
    center.getPendingNotificationRequests {
        payloads = $0
            .sorted { $0.identifier < $1.identifier }
            .map { un_request_payload($0) }
        semaphore.signal()
    }
    semaphore.wait()
    return un_string(un_encode_json(payloads))
}

@_cdecl("un_center_remove_pending_requests")
public func un_center_remove_pending_requests(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ identifiersJSON: UnsafePointer<CChar>?
) {
    guard let center = un_center_box(centerPtr)?.center,
          let identifiers = try? un_decode_json(identifiersJSON, as: [String].self)
    else {
        return
    }

    center.removePendingNotificationRequests(withIdentifiers: identifiers)
}

@_cdecl("un_center_remove_all_pending_requests")
public func un_center_remove_all_pending_requests(_ centerPtr: UnsafeMutableRawPointer?) {
    un_center_box(centerPtr)?.center.removeAllPendingNotificationRequests()
}

@_cdecl("un_center_get_delivered_notifications_json")
public func un_center_get_delivered_notifications_json(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let center = un_center_box(centerPtr)?.center else {
        un_write_error(errorOut, "notification center must not be null")
        return nil
    }

    let semaphore = DispatchSemaphore(value: 0)
    var payloads: [UNNotificationPayload] = []
    center.getDeliveredNotifications {
        payloads = $0
            .sorted { $0.request.identifier < $1.request.identifier }
            .map(un_notification_payload)
        semaphore.signal()
    }
    semaphore.wait()
    return un_string(un_encode_json(payloads))
}

@_cdecl("un_center_remove_delivered_notifications")
public func un_center_remove_delivered_notifications(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ identifiersJSON: UnsafePointer<CChar>?
) {
    guard let center = un_center_box(centerPtr)?.center,
          let identifiers = try? un_decode_json(identifiersJSON, as: [String].self)
    else {
        return
    }

    center.removeDeliveredNotifications(withIdentifiers: identifiers)
}

@_cdecl("un_center_remove_all_delivered_notifications")
public func un_center_remove_all_delivered_notifications(_ centerPtr: UnsafeMutableRawPointer?) {
    un_center_box(centerPtr)?.center.removeAllDeliveredNotifications()
}
