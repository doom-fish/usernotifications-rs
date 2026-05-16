import Foundation
import UserNotifications

struct UNNotificationActionIconPayload: Codable {
    var kind: String
    var name: String?
}

struct UNNotificationActionPayload: Codable {
    var identifier: String
    var title: String
    var options: UInt64
    var icon: UNNotificationActionIconPayload?
    var text_input_button_title: String?
    var text_input_placeholder: String?
    var localized_title: UNLocalizedStringPayload?
    var localized_text_input_button_title: UNLocalizedStringPayload?
    var localized_text_input_placeholder: UNLocalizedStringPayload?
}

@available(macOS 12.0, *)
func un_make_action_icon(_ payload: UNNotificationActionIconPayload?) -> UNNotificationActionIcon? {
    guard let payload else {
        return nil
    }

    switch payload.kind {
    case "templateImage":
        guard let name = payload.name, !name.isEmpty else {
            return nil
        }
        return UNNotificationActionIcon(templateImageName: name)
    case "systemImage":
        guard let name = payload.name, !name.isEmpty else {
            return nil
        }
        return UNNotificationActionIcon(systemImageName: name)
    default:
        return nil
    }
}

func un_make_action(_ payload: UNNotificationActionPayload) -> UNNotificationAction {
    let title = un_make_localized_string(payload.localized_title, fallback: payload.title)
    let options = UNNotificationActionOptions(rawValue: UInt(payload.options))

    if let textInputButtonTitle = payload.text_input_button_title,
       let textInputPlaceholder = payload.text_input_placeholder
    {
        let buttonTitle = un_make_localized_string(
            payload.localized_text_input_button_title,
            fallback: textInputButtonTitle
        )
        let placeholder = un_make_localized_string(
            payload.localized_text_input_placeholder,
            fallback: textInputPlaceholder
        )
        if #available(macOS 12.0, *), let icon = un_make_action_icon(payload.icon) {
            return UNTextInputNotificationAction(
                identifier: payload.identifier,
                title: title,
                options: options,
                icon: icon,
                textInputButtonTitle: buttonTitle,
                textInputPlaceholder: placeholder
            )
        }
        return UNTextInputNotificationAction(
            identifier: payload.identifier,
            title: title,
            options: options,
            textInputButtonTitle: buttonTitle,
            textInputPlaceholder: placeholder
        )
    }

    if #available(macOS 12.0, *), let icon = un_make_action_icon(payload.icon) {
        return UNNotificationAction(
            identifier: payload.identifier,
            title: title,
            options: options,
            icon: icon
        )
    }

    return UNNotificationAction(
        identifier: payload.identifier,
        title: title,
        options: options
    )
}

func un_action_payload(
    _ action: UNNotificationAction,
    source: UNNotificationActionPayload? = nil
) -> UNNotificationActionPayload {
    var payload = UNNotificationActionPayload(
        identifier: action.identifier,
        title: action.title,
        options: UInt64(action.options.rawValue),
        icon: source?.icon,
        text_input_button_title: nil,
        text_input_placeholder: nil,
        localized_title: source?.localized_title,
        localized_text_input_button_title: source?.localized_text_input_button_title,
        localized_text_input_placeholder: source?.localized_text_input_placeholder
    )
    if let textAction = action as? UNTextInputNotificationAction {
        payload.text_input_button_title = textAction.textInputButtonTitle
        payload.text_input_placeholder = textAction.textInputPlaceholder
    }
    return payload
}

@_cdecl("un_action_roundtrip_json")
public func un_action_roundtrip_json(
    _ actionJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let payload = try un_decode_json(actionJSON, as: UNNotificationActionPayload.self)
        let action = un_make_action(payload)
        return un_string(un_encode_json(un_action_payload(action, source: payload)))
    } catch {
        un_write_error(errorOut, error: error)
        return nil
    }
}
