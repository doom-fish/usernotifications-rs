import AppKit
import Foundation
import UserNotifications
import UserNotificationsUI

public typealias UNContentExtensionNotificationCallback =
    @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void
public typealias UNContentExtensionResponseCallback =
    @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> UInt64
public typealias UNContentExtensionSimpleCallback =
    @convention(c) (UnsafeMutableRawPointer?) -> Void

private final class UNRustContentExtensionContextBox: NSObject {
    var notificationActions: [UNNotificationAction] = []
}

@_cdecl("un_content_extension_context_create")
public func un_content_extension_context_create(
    _ outContext: UnsafeMutablePointer<UnsafeMutableRawPointer?>
) -> Int32 {
    outContext.pointee = un_retain(UNRustContentExtensionContextBox())
    return UNR_OK
}

private func un_content_extension_context_box(
    _ ptr: UnsafeMutableRawPointer?
) -> UNRustContentExtensionContextBox? {
    guard let ptr else {
        return nil
    }
    let box: UNRustContentExtensionContextBox = un_borrow(ptr)
    return box
}

@_cdecl("un_content_extension_context_set_notification_actions")
public func un_content_extension_context_set_notification_actions(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ actionsJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 11.0, *) else {
        un_write_error(errorOut, "notification content extensions require macOS 11 or newer")
        return UNR_FRAMEWORK_ERROR
    }
    guard let box = un_content_extension_context_box(contextPtr) else {
        un_write_error(errorOut, "notification content extension context must not be null")
        return UNR_INVALID_ARGUMENT
    }

    do {
        let payloads = try un_decode_json(actionsJSON, as: [UNNotificationActionPayload].self)
        box.notificationActions = payloads.map(un_make_action)
        return UNR_OK
    } catch {
        un_write_error(errorOut, error: error)
        return UNR_INVALID_ARGUMENT
    }
}

@_cdecl("un_content_extension_context_get_notification_actions_json")
public func un_content_extension_context_get_notification_actions_json(
    _ contextPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 11.0, *) else {
        un_write_error(errorOut, "notification content extensions require macOS 11 or newer")
        return nil
    }
    guard let box = un_content_extension_context_box(contextPtr) else {
        un_write_error(errorOut, "notification content extension context must not be null")
        return nil
    }

    return un_string(un_encode_json(box.notificationActions.map { un_action_payload($0) }))
}

@_cdecl("un_content_extension_context_perform_default_action")
public func un_content_extension_context_perform_default_action(
    _ contextPtr: UnsafeMutableRawPointer?
) {
    guard #available(macOS 11.0, *), un_content_extension_context_box(contextPtr) != nil else { return }
}

@_cdecl("un_content_extension_context_dismiss")
public func un_content_extension_context_dismiss(_ contextPtr: UnsafeMutableRawPointer?) {
    guard #available(macOS 11.0, *), un_content_extension_context_box(contextPtr) != nil else { return }
}

@_cdecl("un_content_extension_context_media_playing_started")
public func un_content_extension_context_media_playing_started(
    _ contextPtr: UnsafeMutableRawPointer?
) {
    guard #available(macOS 11.0, *), un_content_extension_context_box(contextPtr) != nil else { return }
}

@_cdecl("un_content_extension_context_media_playing_paused")
public func un_content_extension_context_media_playing_paused(
    _ contextPtr: UnsafeMutableRawPointer?
) {
    guard #available(macOS 11.0, *), un_content_extension_context_box(contextPtr) != nil else { return }
}

@available(macOS 11.0, *)
private final class UNRustNotificationContentExtensionHost: NSObject, UNNotificationContentExtension {
    let notificationCallback: UNContentExtensionNotificationCallback?
    let responseCallback: UNContentExtensionResponseCallback?
    let mediaPlayCallback: UNContentExtensionSimpleCallback?
    let mediaPauseCallback: UNContentExtensionSimpleCallback?
    let userInfo: UnsafeMutableRawPointer?
    var storedButtonType: UNNotificationContentExtensionMediaPlayPauseButtonType = .none
    var storedButtonFrame: CGRect = .zero
    var storedTintColor: NSColor?

    init(
        notificationCallback: UNContentExtensionNotificationCallback?,
        responseCallback: UNContentExtensionResponseCallback?,
        mediaPlayCallback: UNContentExtensionSimpleCallback?,
        mediaPauseCallback: UNContentExtensionSimpleCallback?,
        userInfo: UnsafeMutableRawPointer?
    ) {
        self.notificationCallback = notificationCallback
        self.responseCallback = responseCallback
        self.mediaPlayCallback = mediaPlayCallback
        self.mediaPauseCallback = mediaPauseCallback
        self.userInfo = userInfo
        super.init()
    }

    func notify(_ payload: UNNotificationPayload) {
        guard let notificationCallback else { return }
        let json = un_encode_json(payload)
        json.withCString { notificationCallback(userInfo, $0) }
    }

    func respond(_ payload: UNNotificationResponsePayload) -> UInt64 {
        guard let responseCallback else {
            return UInt64(UNNotificationContentExtensionResponseOption.dismissAndForwardAction.rawValue)
        }
        let json = un_encode_json(payload)
        return json.withCString { responseCallback(userInfo, $0) }
    }

    func didReceive(_ notification: UNNotification) {
        notify(un_notification_payload(notification))
    }

    func didReceive(
        _ response: UNNotificationResponse,
        completionHandler completion: @escaping (UNNotificationContentExtensionResponseOption) -> Void
    ) {
        let optionRaw = respond(un_response_payload(response))
        completion(UNNotificationContentExtensionResponseOption(rawValue: UInt(optionRaw)) ?? .dismissAndForwardAction)
    }

    var mediaPlayPauseButtonType: UNNotificationContentExtensionMediaPlayPauseButtonType {
        storedButtonType
    }

    var mediaPlayPauseButtonFrame: CGRect {
        storedButtonFrame
    }

    var mediaPlayPauseButtonTintColor: NSColor {
        storedTintColor ?? .controlAccentColor
    }

    func mediaPlay() {
        mediaPlayCallback?(userInfo)
    }

    func mediaPause() {
        mediaPauseCallback?(userInfo)
    }
}

@available(macOS 11.0, *)
private func un_content_extension_host(
    _ ptr: UnsafeMutableRawPointer?
) -> UNRustNotificationContentExtensionHost? {
    guard let ptr else {
        return nil
    }
    let host: UNRustNotificationContentExtensionHost = un_borrow(ptr)
    return host
}

@_cdecl("un_content_extension_simulator_create")
public func un_content_extension_simulator_create(
    _ notificationCallback: UNContentExtensionNotificationCallback?,
    _ responseCallback: UNContentExtensionResponseCallback?,
    _ mediaPlayCallback: UNContentExtensionSimpleCallback?,
    _ mediaPauseCallback: UNContentExtensionSimpleCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ outSimulator: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 11.0, *) else {
        un_write_error(errorOut, "notification content extensions require macOS 11 or newer")
        outSimulator.pointee = nil
        return UNR_FRAMEWORK_ERROR
    }
    outSimulator.pointee = un_retain(UNRustNotificationContentExtensionHost(
        notificationCallback: notificationCallback,
        responseCallback: responseCallback,
        mediaPlayCallback: mediaPlayCallback,
        mediaPauseCallback: mediaPauseCallback,
        userInfo: userInfo
    ))
    return UNR_OK
}

@_cdecl("un_content_extension_simulator_set_media_play_pause_button_type")
public func un_content_extension_simulator_set_media_play_pause_button_type(
    _ simulatorPtr: UnsafeMutableRawPointer?,
    _ buttonType: UInt64
) {
    guard #available(macOS 11.0, *), let host = un_content_extension_host(simulatorPtr) else {
        return
    }
    host.storedButtonType = UNNotificationContentExtensionMediaPlayPauseButtonType(rawValue: UInt(buttonType))
        ?? .none
}

@_cdecl("un_content_extension_simulator_get_media_play_pause_button_type")
public func un_content_extension_simulator_get_media_play_pause_button_type(
    _ simulatorPtr: UnsafeMutableRawPointer?
) -> UInt64 {
    guard #available(macOS 11.0, *) else {
        return 0
    }
    return UInt64(un_content_extension_host(simulatorPtr)?.storedButtonType.rawValue ?? 0)
}

@_cdecl("un_content_extension_simulator_set_media_play_pause_button_frame")
public func un_content_extension_simulator_set_media_play_pause_button_frame(
    _ simulatorPtr: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double
) {
    guard #available(macOS 11.0, *) else {
        return
    }
    un_content_extension_host(simulatorPtr)?.storedButtonFrame = CGRect(
        x: x,
        y: y,
        width: width,
        height: height
    )
}

@_cdecl("un_content_extension_simulator_get_media_play_pause_button_frame")
public func un_content_extension_simulator_get_media_play_pause_button_frame(
    _ simulatorPtr: UnsafeMutableRawPointer?,
    _ x: UnsafeMutablePointer<Double>,
    _ y: UnsafeMutablePointer<Double>,
    _ width: UnsafeMutablePointer<Double>,
    _ height: UnsafeMutablePointer<Double>
) {
    guard #available(macOS 11.0, *) else {
        x.pointee = 0
        y.pointee = 0
        width.pointee = 0
        height.pointee = 0
        return
    }
    let frame = un_content_extension_host(simulatorPtr)?.storedButtonFrame ?? .zero
    x.pointee = frame.origin.x
    y.pointee = frame.origin.y
    width.pointee = frame.size.width
    height.pointee = frame.size.height
}

@_cdecl("un_content_extension_simulator_set_media_play_pause_button_tint_color")
public func un_content_extension_simulator_set_media_play_pause_button_tint_color(
    _ simulatorPtr: UnsafeMutableRawPointer?,
    _ red: Double,
    _ green: Double,
    _ blue: Double,
    _ alpha: Double
) {
    guard #available(macOS 11.0, *) else {
        return
    }
    un_content_extension_host(simulatorPtr)?.storedTintColor = NSColor(
        calibratedRed: red,
        green: green,
        blue: blue,
        alpha: alpha
    )
}

@_cdecl("un_content_extension_simulator_get_media_play_pause_button_tint_color")
public func un_content_extension_simulator_get_media_play_pause_button_tint_color(
    _ simulatorPtr: UnsafeMutableRawPointer?,
    _ red: UnsafeMutablePointer<Double>,
    _ green: UnsafeMutablePointer<Double>,
    _ blue: UnsafeMutablePointer<Double>,
    _ alpha: UnsafeMutablePointer<Double>
) -> Bool {
    guard #available(macOS 11.0, *),
          let color = un_content_extension_host(simulatorPtr)?.storedTintColor
    else {
        return false
    }
    let converted = color.usingColorSpace(.deviceRGB) ?? color
    red.pointee = Double(converted.redComponent)
    green.pointee = Double(converted.greenComponent)
    blue.pointee = Double(converted.blueComponent)
    alpha.pointee = Double(converted.alphaComponent)
    return true
}

@_cdecl("un_content_extension_simulator_receive_notification_json")
public func un_content_extension_simulator_receive_notification_json(
    _ simulatorPtr: UnsafeMutableRawPointer?,
    _ notificationJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 11.0, *), let host = un_content_extension_host(simulatorPtr) else {
        un_write_error(errorOut, "notification content extension simulator must not be null")
        return UNR_INVALID_ARGUMENT
    }

    do {
        let payload = try un_decode_json(notificationJSON, as: UNNotificationPayload.self)
        host.notify(payload)
        return UNR_OK
    } catch {
        un_write_error(errorOut, error: error)
        return UNR_INVALID_ARGUMENT
    }
}

@_cdecl("un_content_extension_simulator_receive_response_json")
public func un_content_extension_simulator_receive_response_json(
    _ simulatorPtr: UnsafeMutableRawPointer?,
    _ responseJSON: UnsafePointer<CChar>?,
    _ outOption: UnsafeMutablePointer<UInt64>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 11.0, *), let host = un_content_extension_host(simulatorPtr) else {
        un_write_error(errorOut, "notification content extension simulator must not be null")
        return UNR_INVALID_ARGUMENT
    }

    do {
        let payload = try un_decode_json(responseJSON, as: UNNotificationResponsePayload.self)
        outOption.pointee = host.respond(payload)
        return UNR_OK
    } catch {
        un_write_error(errorOut, error: error)
        return UNR_INVALID_ARGUMENT
    }
}

@_cdecl("un_content_extension_simulator_media_play")
public func un_content_extension_simulator_media_play(_ simulatorPtr: UnsafeMutableRawPointer?) {
    guard #available(macOS 11.0, *) else {
        return
    }
    un_content_extension_host(simulatorPtr)?.mediaPlay()
}

@_cdecl("un_content_extension_simulator_media_pause")
public func un_content_extension_simulator_media_pause(_ simulatorPtr: UnsafeMutableRawPointer?) {
    guard #available(macOS 11.0, *) else {
        return
    }
    un_content_extension_host(simulatorPtr)?.mediaPause()
}
