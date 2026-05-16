import Dispatch
import Foundation
import UserNotifications

public typealias UNCenterEventCallback =
    @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void

private final class UNRustCenterDelegate: NSObject, UNUserNotificationCenterDelegate {
    let callback: UNCenterEventCallback
    let userInfo: UnsafeMutableRawPointer?
    private var isActive = true

    init(callback: @escaping UNCenterEventCallback, userInfo: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.userInfo = userInfo
        super.init()
    }

    func deactivate() {
        isActive = false
    }

    private func send(_ payload: [String: Any]) {
        guard isActive else { return }
        let json = un_json_string(payload)
        json.withCString { callback(userInfo, $0) }
    }

    func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        didReceive response: UNNotificationResponse,
        withCompletionHandler completionHandler: @escaping () -> Void
    ) {
        send([
            "event": "didReceiveNotificationResponse",
            "response": un_response_object(response),
        ])
        completionHandler()
    }

    func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        openSettingsFor notification: UNNotification?
    ) {
        send([
            "event": "openSettingsForNotification",
            "notification": notification.map(un_notification_object) ?? NSNull(),
        ])
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
    let box = UNUserNotificationCenterBox(center: center)
    outCenter.pointee = un_retain(box)
    return UNR_OK
}

@_cdecl("un_center_set_delegate")
public func un_center_set_delegate(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ callback: UNCenterEventCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = un_center_box(centerPtr), let callback else {
        un_write_error(errorOut, "notification center and callback must not be null")
        return UNR_INVALID_ARGUMENT
    }

    let delegateBox = UNRustCenterDelegate(callback: callback, userInfo: userInfo)
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
        un_write_error(errorOut, completionError.localizedDescription)
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
    var json = "null"
    center.getNotificationSettings {
        json = un_json_string(un_settings_object($0))
        semaphore.signal()
    }
    semaphore.wait()
    return un_string(json)
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
        un_write_error(errorOut, error.localizedDescription)
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
    var json = "[]"
    center.getNotificationCategories {
        let categories = Array($0).sorted { $0.identifier < $1.identifier }
        json = un_json_string(categories.map(un_category_object))
        semaphore.signal()
    }
    semaphore.wait()
    return un_string(json)
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
            un_write_error(errorOut, completionError.localizedDescription)
            return UNR_FRAMEWORK_ERROR
        }
        return UNR_OK
    } catch {
        un_write_error(errorOut, error.localizedDescription)
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
    var json = "[]"
    center.getPendingNotificationRequests {
        let requests = $0.sorted { $0.identifier < $1.identifier }
        json = un_json_string(requests.map(un_request_object))
        semaphore.signal()
    }
    semaphore.wait()
    return un_string(json)
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
    var json = "[]"
    center.getDeliveredNotifications {
        let notifications = $0.sorted { $0.request.identifier < $1.request.identifier }
        json = un_json_string(notifications.map(un_notification_object))
        semaphore.signal()
    }
    semaphore.wait()
    return un_string(json)
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
