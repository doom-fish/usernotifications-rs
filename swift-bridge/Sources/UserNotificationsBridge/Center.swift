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

private final class UNDelegateRegistration: @unchecked Sendable {
    private let eventCallback: UNCenterEventCallback?
    private let willPresentCallback: UNCenterWillPresentCallback?
    private let context: UnsafeMutableRawPointer
    private let release: UNContextCallback

    init?(
        eventCallback: UNCenterEventCallback?,
        willPresentCallback: UNCenterWillPresentCallback?,
        context: UnsafeMutableRawPointer?,
        retain: UNContextCallback?,
        release: UNContextCallback?
    ) {
        guard let context, let retain, let release else {
            return nil
        }
        self.eventCallback = eventCallback
        self.willPresentCallback = willPresentCallback
        self.context = context
        self.release = release
        retain(context)
    }

    deinit {
        release(context)
    }

    func send(_ json: String) {
        guard let eventCallback else { return }
        json.withCString { eventCallback(context, $0) }
    }

    func willPresent(_ json: String) -> UInt64 {
        guard let willPresentCallback else { return 0 }
        return json.withCString { willPresentCallback(context, $0) }
    }
}

private final class UNCenterDelegateMultiplexer: NSObject, UNUserNotificationCenterDelegate {
    static let shared = UNCenterDelegateMultiplexer()

    private let lock = NSRecursiveLock()
    private var registrations: [(token: UInt64, registration: UNDelegateRegistration)] = []
    private var nextToken: UInt64 = 1

    func register(
        _ registration: UNDelegateRegistration,
        on center: UNUserNotificationCenter
    ) -> UInt64 {
        lock.lock()
        defer { lock.unlock() }
        let token = nextToken
        nextToken += 1
        registrations.append((token, registration))
        if center.delegate !== self {
            center.delegate = self
        }
        return token
    }

    func unregister(_ token: UInt64, on center: UNUserNotificationCenter) {
        lock.lock()
        let index = registrations.firstIndex { $0.token == token }
        let removed = index.map { registrations.remove(at: $0).registration }
        if registrations.isEmpty, center.delegate === self {
            center.delegate = nil
        }
        withExtendedLifetime(removed) {
            lock.unlock()
        }
    }

    private func snapshot() -> [UNDelegateRegistration] {
        lock.lock()
        defer { lock.unlock() }
        return registrations.map(\.registration)
    }

    func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        willPresent notification: UNNotification,
        withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void
    ) {
        let json = un_encode_json(un_notification_payload(notification))
        let bits = snapshot().reduce(UInt64(0)) { $0 | $1.willPresent(json) }
        completionHandler(UNNotificationPresentationOptions(rawValue: UInt(truncatingIfNeeded: bits)))
    }

    func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        didReceive response: UNNotificationResponse,
        withCompletionHandler completionHandler: @escaping () -> Void
    ) {
        let json = un_encode_json(UNCenterEventPayload(
            event: "didReceiveNotificationResponse",
            notification: nil,
            response: un_response_payload(response)
        ))
        snapshot().forEach { $0.send(json) }
        completionHandler()
    }

    func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        openSettingsFor notification: UNNotification?
    ) {
        let json = un_encode_json(UNCenterEventPayload(
            event: "openSettingsForNotification",
            notification: notification.map(un_notification_payload),
            response: nil
        ))
        snapshot().forEach { $0.send(json) }
    }
}

private final class UNUserNotificationCenterBox: NSObject {
    let center: UNUserNotificationCenter
    private var delegateToken: UInt64?

    init(center: UNUserNotificationCenter) {
        self.center = center
        super.init()
    }

    func setDelegate(_ registration: UNDelegateRegistration?) {
        let newToken = registration.map {
            UNCenterDelegateMultiplexer.shared.register($0, on: center)
        }
        let oldToken = delegateToken
        delegateToken = newToken
        if let oldToken {
            UNCenterDelegateMultiplexer.shared.unregister(oldToken, on: center)
        }
    }

    deinit {
        if let delegateToken {
            UNCenterDelegateMultiplexer.shared.unregister(delegateToken, on: center)
        }
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
    _ context: UnsafeMutableRawPointer?,
    _ contextRetain: UNContextCallback?,
    _ contextRelease: UNContextCallback?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let box = un_center_box(centerPtr) else {
        un_write_error(errorOut, "notification center must not be null")
        return UNR_INVALID_ARGUMENT
    }
    guard let registration = UNDelegateRegistration(
        eventCallback: eventCallback,
        willPresentCallback: willPresentCallback,
        context: context,
        retain: contextRetain,
        release: contextRelease
    ) else {
        un_write_error(errorOut, "delegate context and its retain/release callbacks must not be null")
        return UNR_INVALID_ARGUMENT
    }

    box.setDelegate(registration)
    return UNR_OK
}

@_cdecl("un_center_clear_delegate")
public func un_center_clear_delegate(_ centerPtr: UnsafeMutableRawPointer?) {
    un_center_box(centerPtr)?.setDelegate(nil)
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

    outGranted.pointee = false
    let result = UNCompletionResult()
    center.requestAuthorization(
        options: UNAuthorizationOptions(rawValue: UInt(truncatingIfNeeded: options))
    ) {
        result.finish(UNCompletionOutcome(granted: $0, error: $1))
    }
    guard let outcome = result.wait() else {
        un_write_error(
            errorOut,
            "timed out after \(UN_WAIT_SECONDS) s waiting for the notification permission prompt; authorization has not been decided yet"
        )
        return UNR_TIMED_OUT
    }

    outGranted.pointee = outcome.granted
    if let error = outcome.error {
        un_write_error(errorOut, error: error)
        return UNR_FRAMEWORK_ERROR
    }
    return UNR_OK
}

private func un_write_timeout(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ operation: String
) {
    un_write_error(errorOut, "timed out after \(UN_WAIT_SECONDS) s waiting for \(operation)")
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

    let result = UNCompletionResult()
    center.setBadgeCount(newBadgeCount) {
        result.finish(UNCompletionOutcome(error: $0))
    }
    guard let outcome = result.wait() else {
        un_write_timeout(errorOut, "setBadgeCount")
        return UNR_TIMED_OUT
    }

    if let error = outcome.error {
        un_write_error(errorOut, error: error)
        return UNR_FRAMEWORK_ERROR
    }
    return UNR_OK
}

@_cdecl("un_center_get_notification_settings_json")
public func un_center_get_notification_settings_json(
    _ centerPtr: UnsafeMutableRawPointer?,
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outJSON?.pointee = nil
    guard let center = un_center_box(centerPtr)?.center else {
        un_write_error(errorOut, "notification center must not be null")
        return UNR_INVALID_ARGUMENT
    }

    let result = UNCompletionResult()
    center.getNotificationSettings {
        result.finish(UNCompletionOutcome(payload: un_encode_json(un_settings_payload($0))))
    }
    guard let payload = result.wait()?.payload else {
        un_write_timeout(errorOut, "the notification settings")
        return UNR_TIMED_OUT
    }
    outJSON?.pointee = un_string(payload)
    return UNR_OK
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
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outJSON?.pointee = nil
    guard let center = un_center_box(centerPtr)?.center else {
        un_write_error(errorOut, "notification center must not be null")
        return UNR_INVALID_ARGUMENT
    }

    let result = UNCompletionResult()
    center.getNotificationCategories {
        let payloads = Array($0)
            .sorted { $0.identifier < $1.identifier }
            .map { un_category_payload($0) }
        result.finish(UNCompletionOutcome(payload: un_encode_json(payloads)))
    }
    guard let payload = result.wait()?.payload else {
        un_write_timeout(errorOut, "the notification categories")
        return UNR_TIMED_OUT
    }
    outJSON?.pointee = un_string(payload)
    return UNR_OK
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
        let result = UNCompletionResult()
        center.add(request) {
            result.finish(UNCompletionOutcome(error: $0))
        }
        guard let outcome = result.wait() else {
            un_write_timeout(errorOut, "the notification request to be added")
            return UNR_TIMED_OUT
        }
        if let error = outcome.error {
            un_write_error(errorOut, error: error)
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
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outJSON?.pointee = nil
    guard let center = un_center_box(centerPtr)?.center else {
        un_write_error(errorOut, "notification center must not be null")
        return UNR_INVALID_ARGUMENT
    }

    let result = UNCompletionResult()
    center.getPendingNotificationRequests {
        let payloads = $0
            .sorted { $0.identifier < $1.identifier }
            .map { un_request_payload($0) }
        result.finish(UNCompletionOutcome(payload: un_encode_json(payloads)))
    }
    guard let payload = result.wait()?.payload else {
        un_write_timeout(errorOut, "the pending notification requests")
        return UNR_TIMED_OUT
    }
    outJSON?.pointee = un_string(payload)
    return UNR_OK
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
    _ outJSON: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outJSON?.pointee = nil
    guard let center = un_center_box(centerPtr)?.center else {
        un_write_error(errorOut, "notification center must not be null")
        return UNR_INVALID_ARGUMENT
    }

    let result = UNCompletionResult()
    center.getDeliveredNotifications {
        let payloads = $0
            .sorted { $0.request.identifier < $1.request.identifier }
            .map(un_notification_payload)
        result.finish(UNCompletionOutcome(payload: un_encode_json(payloads)))
    }
    guard let payload = result.wait()?.payload else {
        un_write_timeout(errorOut, "the delivered notifications")
        return UNR_TIMED_OUT
    }
    outJSON?.pointee = un_string(payload)
    return UNR_OK
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
