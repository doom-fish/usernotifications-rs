import Dispatch
import Foundation
import UserNotifications

public typealias UNServiceExtensionReceiveCallback =
    @convention(c) (
        UnsafeMutableRawPointer?,
        UnsafePointer<CChar>?,
        UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
    ) -> UnsafeMutablePointer<CChar>?
public typealias UNServiceExtensionExpireCallback =
    @convention(c) (UnsafeMutableRawPointer?) -> Void

private final class UNRustNotificationServiceExtension: UNNotificationServiceExtension {
    let receiveCallback: UNServiceExtensionReceiveCallback?
    let expireCallback: UNServiceExtensionExpireCallback?
    let userInfo: UnsafeMutableRawPointer?
    var pendingErrorMessage: String?
    var lastDeliveredContent: UNNotificationContent?

    init(
        receiveCallback: UNServiceExtensionReceiveCallback?,
        expireCallback: UNServiceExtensionExpireCallback?,
        userInfo: UnsafeMutableRawPointer?
    ) {
        self.receiveCallback = receiveCallback
        self.expireCallback = expireCallback
        self.userInfo = userInfo
        super.init()
    }

    override func didReceive(
        _ request: UNNotificationRequest,
        withContentHandler contentHandler: @escaping (UNNotificationContent) -> Void
    ) {
        guard let receiveCallback else {
            lastDeliveredContent = request.content
            contentHandler(request.content)
            return
        }

        let requestJSON = un_encode_json(un_request_payload(request))
        var callbackError: UnsafeMutablePointer<CChar>?
        let contentJSON = requestJSON.withCString { callback in
            receiveCallback(userInfo, callback, &callbackError)
        }
        defer {
            if let callbackError {
                free(callbackError)
            }
        }

        if let callbackError {
            pendingErrorMessage = String(cString: callbackError)
            lastDeliveredContent = request.content
            contentHandler(request.content)
            return
        }

        guard let contentJSON else {
            lastDeliveredContent = request.content
            contentHandler(request.content)
            return
        }
        defer { free(contentJSON) }

        do {
            let payload = try un_decode_json(contentJSON, as: UNNotificationContentPayload.self)
            let content = try un_make_content(payload)
            lastDeliveredContent = content
            contentHandler(content)
        } catch {
            pendingErrorMessage = un_error_message(error)
            lastDeliveredContent = request.content
            contentHandler(request.content)
        }
    }

    override func serviceExtensionTimeWillExpire() {
        expireCallback?(userInfo)
    }
}

@_cdecl("un_service_extension_simulator_create")
public func un_service_extension_simulator_create(
    _ receiveCallback: UNServiceExtensionReceiveCallback?,
    _ expireCallback: UNServiceExtensionExpireCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ outSimulator: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outSimulator.pointee = un_retain(
        UNRustNotificationServiceExtension(
            receiveCallback: receiveCallback,
            expireCallback: expireCallback,
            userInfo: userInfo
        )
    )
    return UNR_OK
}

@_cdecl("un_service_extension_simulator_receive_request_json")
public func un_service_extension_simulator_receive_request_json(
    _ simulatorPtr: UnsafeMutableRawPointer?,
    _ requestJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let simulatorPtr else {
        un_write_error(errorOut, "service extension simulator must not be null")
        return nil
    }

    do {
        let payload = try un_decode_json(requestJSON, as: UNNotificationRequestPayload.self)
        let request = try un_make_request(payload)
        let simulator: UNRustNotificationServiceExtension = un_borrow(simulatorPtr)
        simulator.pendingErrorMessage = nil
        simulator.lastDeliveredContent = nil

        let semaphore = DispatchSemaphore(value: 0)
        simulator.didReceive(request) { deliveredContent in
            simulator.lastDeliveredContent = deliveredContent
            semaphore.signal()
        }
        semaphore.wait()

        if let pendingErrorMessage = simulator.pendingErrorMessage {
            simulator.pendingErrorMessage = nil
            un_write_error(errorOut, pendingErrorMessage)
            return nil
        }
        guard let deliveredContent = simulator.lastDeliveredContent else {
            un_write_error(errorOut, "service extension did not produce content")
            return nil
        }
        return un_string(un_encode_json(un_content_payload(deliveredContent)))
    } catch {
        un_write_error(errorOut, error: error)
        return nil
    }
}

@_cdecl("un_service_extension_simulator_time_will_expire")
public func un_service_extension_simulator_time_will_expire(
    _ simulatorPtr: UnsafeMutableRawPointer?
) {
    guard let simulatorPtr else { return }
    let simulator: UNRustNotificationServiceExtension = un_borrow(simulatorPtr)
    simulator.serviceExtensionTimeWillExpire()
}
