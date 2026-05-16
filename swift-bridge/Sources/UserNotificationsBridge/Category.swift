import Foundation
import UserNotifications

struct UNNotificationCategoryPayload: Codable {
    var identifier: String
    var actions: [UNNotificationActionPayload]
    var intent_identifiers: [String]
    var options: UInt64
    var hidden_previews_body_placeholder: String?
    var category_summary_format: String?
    var localized_hidden_previews_body_placeholder: UNLocalizedStringPayload?
    var localized_category_summary_format: UNLocalizedStringPayload?
}

func un_make_category(_ payload: UNNotificationCategoryPayload) -> UNNotificationCategory {
    let actions = payload.actions.map(un_make_action)
    let options = UNNotificationCategoryOptions(rawValue: UInt(payload.options))
    let hiddenPreviewsBodyPlaceholder = payload.hidden_previews_body_placeholder.map {
        un_make_localized_string(payload.localized_hidden_previews_body_placeholder, fallback: $0)
    }
    let categorySummaryFormat = payload.category_summary_format.map {
        un_make_localized_string(payload.localized_category_summary_format, fallback: $0)
    }

    if let categorySummaryFormat {
        return UNNotificationCategory(
            identifier: payload.identifier,
            actions: actions,
            intentIdentifiers: payload.intent_identifiers,
            hiddenPreviewsBodyPlaceholder: hiddenPreviewsBodyPlaceholder,
            categorySummaryFormat: categorySummaryFormat,
            options: options
        )
    }

    if let hiddenPreviewsBodyPlaceholder {
        return UNNotificationCategory(
            identifier: payload.identifier,
            actions: actions,
            intentIdentifiers: payload.intent_identifiers,
            hiddenPreviewsBodyPlaceholder: hiddenPreviewsBodyPlaceholder,
            options: options
        )
    }

    return UNNotificationCategory(
        identifier: payload.identifier,
        actions: actions,
        intentIdentifiers: payload.intent_identifiers,
        options: options
    )
}

func un_category_payload(
    _ category: UNNotificationCategory,
    source: UNNotificationCategoryPayload? = nil
) -> UNNotificationCategoryPayload {
    UNNotificationCategoryPayload(
        identifier: category.identifier,
        actions: zip(category.actions, source?.actions ?? []).map {
            un_action_payload($0.0, source: $0.1)
        }.ifEmpty {
            category.actions.map { un_action_payload($0) }
        },
        intent_identifiers: category.intentIdentifiers,
        options: UInt64(category.options.rawValue),
        hidden_previews_body_placeholder: category.hiddenPreviewsBodyPlaceholder,
        category_summary_format: category.categorySummaryFormat,
        localized_hidden_previews_body_placeholder: source?.localized_hidden_previews_body_placeholder,
        localized_category_summary_format: source?.localized_category_summary_format
    )
}

@_cdecl("un_category_roundtrip_json")
public func un_category_roundtrip_json(
    _ categoryJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    do {
        let payload = try un_decode_json(categoryJSON, as: UNNotificationCategoryPayload.self)
        let category = un_make_category(payload)
        return un_string(un_encode_json(un_category_payload(category, source: payload)))
    } catch {
        un_write_error(errorOut, error: error)
        return nil
    }
}

private extension Array {
    func ifEmpty(_ fallback: () -> [Element]) -> [Element] {
        isEmpty ? fallback() : self
    }
}
