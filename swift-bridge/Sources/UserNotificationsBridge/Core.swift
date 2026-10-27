import UserNotificationsABI
import Foundation
import UserNotifications

public let UNR_OK: Int32 = 0
public let UNR_INVALID_ARGUMENT: Int32 = -1
public let UNR_FRAMEWORK_ERROR: Int32 = -2
public let UNR_TIMED_OUT: Int32 = -3

let UN_WAIT_SECONDS = 30

public typealias UNContextCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

struct UNCompletionOutcome {
    var granted = false
    var payload: UNBinaryPayload?
    var error: Error?
}

final class UNCompletionResult: @unchecked Sendable {
    private let lock = NSLock()
    private let semaphore = DispatchSemaphore(value: 0)
    private var outcome: UNCompletionOutcome?

    func finish(_ value: UNCompletionOutcome) {
        lock.lock()
        let first = outcome == nil
        if first {
            outcome = value
        }
        lock.unlock()
        if first {
            semaphore.signal()
        }
    }

    func wait() -> UNCompletionOutcome? {
        _ = semaphore.wait(timeout: .now() + .seconds(UN_WAIT_SECONDS))
        lock.lock()
        defer { lock.unlock() }
        return outcome
    }
}

@inline(__always)
public func un_retain<T: AnyObject>(_ object: T) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
public func un_borrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@_cdecl("un_object_release")
public func un_object_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

@_cdecl("un_error_domain")
public func un_error_domain() -> UnsafeMutablePointer<CChar>? {
    un_string(UNErrorDomain)
}

@inline(__always)
func un_string(_ value: String) -> UnsafeMutablePointer<CChar>? {
    value.withCString { strdup($0) }
}

func un_error_message(_ error: Error) -> String {
    let nsError = error as NSError
    return "\(nsError.domain):\(nsError.code):\(nsError.localizedDescription)"
}

@inline(__always)
func un_write_error(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ message: String
) {
    errorOut?.pointee = un_string(message)
}

@inline(__always)
func un_write_error(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    error: Error
) {
    un_write_error(errorOut, un_error_message(error))
}

struct UNLocalizedStringPayload {
    var key: String
    var arguments: [UNPropertyValue]
}

func un_make_localized_string(_ localized: UNLocalizedStringPayload?, fallback: String) -> String {
    guard let localized else {
        return fallback
    }
    return NSString.localizedUserNotificationString(
        forKey: localized.key,
        arguments: localized.arguments.map(\.foundationObject)
    ) as String
}
