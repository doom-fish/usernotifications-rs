import Foundation
import UserNotificationsABI

extension UNNotificationResponsePayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNNotificationResponsePayloadRaw {
        UNNotificationResponsePayloadRaw(
            action_identifier: action_identifier.raw(arena),
            notification: notification.raw(arena),
            user_text: user_text.map { arena.keep($0.raw(arena)) }
        )
    }
    static func read(_ raw: UNNotificationResponsePayloadRaw) throws -> Self {
        Self(
            action_identifier: try String.read(raw.action_identifier),
            notification: try UNNotificationPayload.read(raw.notification),
            user_text: try raw.user_text.map { try String.read($0.pointee) }
        )
    }
}
extension UNNotificationRequestPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNNotificationRequestPayloadRaw {
        UNNotificationRequestPayloadRaw(
            identifier: identifier.raw(arena),
            content: content.raw(arena),
            trigger: trigger.map { arena.keep($0.raw(arena)) }
        )
    }
    static func read(_ raw: UNNotificationRequestPayloadRaw) throws -> Self {
        Self(
            identifier: try String.read(raw.identifier),
            content: try UNNotificationContentPayload.read(raw.content),
            trigger: try raw.trigger.map { try UNNotificationTriggerPayload.read($0.pointee) }
        )
    }
}
extension UNNotificationActionIconPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNNotificationActionIconPayloadRaw {
        UNNotificationActionIconPayloadRaw(
            kind: ["unknown": UInt32(0), "templateImage": UInt32(1), "systemImage": UInt32(2)][kind] ?? 0,
            name: name.map { arena.keep($0.raw(arena)) }
        )
    }
    static func read(_ raw: UNNotificationActionIconPayloadRaw) throws -> Self {
        Self(
            kind: [UInt32(0): "unknown", UInt32(1): "templateImage", UInt32(2): "systemImage"][raw.kind] ?? "unknown",
            name: try raw.name.map { try String.read($0.pointee) }
        )
    }
}
extension UNNotificationActionPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNNotificationActionPayloadRaw {
        UNNotificationActionPayloadRaw(
            identifier: identifier.raw(arena),
            title: title.raw(arena),
            options: options,
            icon: icon.map { arena.keep($0.raw(arena)) },
            text_input_button_title: text_input_button_title.map { arena.keep($0.raw(arena)) },
            text_input_placeholder: text_input_placeholder.map { arena.keep($0.raw(arena)) },
            localized_title: localized_title.map { arena.keep($0.raw(arena)) },
            localized_text_input_button_title: localized_text_input_button_title.map { arena.keep($0.raw(arena)) },
            localized_text_input_placeholder: localized_text_input_placeholder.map { arena.keep($0.raw(arena)) }
        )
    }
    static func read(_ raw: UNNotificationActionPayloadRaw) throws -> Self {
        Self(
            identifier: try String.read(raw.identifier),
            title: try String.read(raw.title),
            options: raw.options,
            icon: try raw.icon.map { try UNNotificationActionIconPayload.read($0.pointee) },
            text_input_button_title: try raw.text_input_button_title.map { try String.read($0.pointee) },
            text_input_placeholder: try raw.text_input_placeholder.map { try String.read($0.pointee) },
            localized_title: try raw.localized_title.map { try UNLocalizedStringPayload.read($0.pointee) },
            localized_text_input_button_title: try raw.localized_text_input_button_title.map { try UNLocalizedStringPayload.read($0.pointee) },
            localized_text_input_placeholder: try raw.localized_text_input_placeholder.map { try UNLocalizedStringPayload.read($0.pointee) }
        )
    }
}
extension UNNotificationPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNNotificationPayloadRaw {
        UNNotificationPayloadRaw(
            date: date,
            request: request.raw(arena)
        )
    }
    static func read(_ raw: UNNotificationPayloadRaw) throws -> Self {
        Self(
            date: raw.date,
            request: try UNNotificationRequestPayload.read(raw.request)
        )
    }
}
extension UNCenterEventPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNCenterEventPayloadRaw {
        UNCenterEventPayloadRaw(
            event: ["unknown": UInt32(0), "didReceiveNotificationResponse": UInt32(1), "openSettingsForNotification": UInt32(2)][event] ?? 0,
            notification: notification.map { arena.keep($0.raw(arena)) },
            response: response.map { arena.keep($0.raw(arena)) }
        )
    }
    static func read(_ raw: UNCenterEventPayloadRaw) throws -> Self {
        Self(
            event: [UInt32(0): "unknown", UInt32(1): "didReceiveNotificationResponse", UInt32(2): "openSettingsForNotification"][raw.event] ?? "unknown",
            notification: try raw.notification.map { try UNNotificationPayload.read($0.pointee) },
            response: try raw.response.map { try UNNotificationResponsePayload.read($0.pointee) }
        )
    }
}
extension UNDateComponentsPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNDateComponentsRaw {
        UNDateComponentsRaw(
            year: year.map { arena.keep(Int64($0)) },
            month: month.map { arena.keep(Int64($0)) },
            day: day.map { arena.keep(Int64($0)) },
            hour: hour.map { arena.keep(Int64($0)) },
            minute: minute.map { arena.keep(Int64($0)) },
            second: second.map { arena.keep(Int64($0)) },
            weekday: weekday.map { arena.keep(Int64($0)) }
        )
    }
    static func read(_ raw: UNDateComponentsRaw) throws -> Self {
        Self(
            year: raw.year.map { Int($0.pointee) },
            month: raw.month.map { Int($0.pointee) },
            day: raw.day.map { Int($0.pointee) },
            hour: raw.hour.map { Int($0.pointee) },
            minute: raw.minute.map { Int($0.pointee) },
            second: raw.second.map { Int($0.pointee) },
            weekday: raw.weekday.map { Int($0.pointee) }
        )
    }
}
extension UNNotificationTriggerPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNNotificationTriggerPayloadRaw {
        UNNotificationTriggerPayloadRaw(
            kind: ["none": UInt32(0), "timeInterval": UInt32(1), "calendar": UInt32(2), "push": UInt32(3), "location": UInt32(4), "unknown": UInt32(5)][kind] ?? 0,
            repeats: repeats.map { arena.keep($0 ? 1 : 0) },
            time_interval: time_interval.map { arena.keep($0) },
            date_components: date_components.map { arena.keep($0.raw(arena)) },
            next_trigger_date: next_trigger_date.map { arena.keep($0) },
            class_name: class_name.map { arena.keep($0.raw(arena)) }
        )
    }
    static func read(_ raw: UNNotificationTriggerPayloadRaw) throws -> Self {
        Self(
            kind: [UInt32(0): "none", UInt32(1): "timeInterval", UInt32(2): "calendar", UInt32(3): "push", UInt32(4): "location", UInt32(5): "unknown"][raw.kind] ?? "unknown",
            repeats: raw.repeats.map { $0.pointee != 0 },
            time_interval: raw.time_interval.map { $0.pointee },
            date_components: try raw.date_components.map { try UNDateComponentsPayload.read($0.pointee) },
            next_trigger_date: raw.next_trigger_date.map { $0.pointee },
            class_name: try raw.class_name.map { try String.read($0.pointee) }
        )
    }
}
extension UNNotificationCategoryPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNNotificationCategoryPayloadRaw {
        UNNotificationCategoryPayloadRaw(
            identifier: identifier.raw(arena),
            actions: actions.raw(arena),
            intent_identifiers: intent_identifiers.raw(arena),
            options: options,
            hidden_previews_body_placeholder: hidden_previews_body_placeholder.map { arena.keep($0.raw(arena)) },
            category_summary_format: category_summary_format.map { arena.keep($0.raw(arena)) },
            localized_hidden_previews_body_placeholder: localized_hidden_previews_body_placeholder.map { arena.keep($0.raw(arena)) },
            localized_category_summary_format: localized_category_summary_format.map { arena.keep($0.raw(arena)) }
        )
    }
    static func read(_ raw: UNNotificationCategoryPayloadRaw) throws -> Self {
        Self(
            identifier: try String.read(raw.identifier),
            actions: try [UNNotificationActionPayload].read(raw.actions),
            intent_identifiers: try [String].read(raw.intent_identifiers),
            options: raw.options,
            hidden_previews_body_placeholder: try raw.hidden_previews_body_placeholder.map { try String.read($0.pointee) },
            category_summary_format: try raw.category_summary_format.map { try String.read($0.pointee) },
            localized_hidden_previews_body_placeholder: try raw.localized_hidden_previews_body_placeholder.map { try UNLocalizedStringPayload.read($0.pointee) },
            localized_category_summary_format: try raw.localized_category_summary_format.map { try UNLocalizedStringPayload.read($0.pointee) }
        )
    }
}
extension UNNotificationSettingsPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNNotificationSettingsPayloadRaw {
        UNNotificationSettingsPayloadRaw(
            authorization_status: authorization_status,
            sound_setting: sound_setting,
            badge_setting: badge_setting,
            alert_setting: alert_setting,
            notification_center_setting: notification_center_setting,
            lock_screen_setting: lock_screen_setting,
            alert_style: alert_style,
            show_previews_setting: show_previews_setting,
            critical_alert_setting: critical_alert_setting,
            provides_app_notification_settings: provides_app_notification_settings ? 1 : 0,
            time_sensitive_setting: time_sensitive_setting,
            scheduled_delivery_setting: scheduled_delivery_setting,
            direct_messages_setting: direct_messages_setting
        )
    }
    static func read(_ raw: UNNotificationSettingsPayloadRaw) throws -> Self {
        Self(
            authorization_status: raw.authorization_status,
            sound_setting: raw.sound_setting,
            badge_setting: raw.badge_setting,
            alert_setting: raw.alert_setting,
            notification_center_setting: raw.notification_center_setting,
            lock_screen_setting: raw.lock_screen_setting,
            alert_style: raw.alert_style,
            show_previews_setting: raw.show_previews_setting,
            critical_alert_setting: raw.critical_alert_setting,
            provides_app_notification_settings: raw.provides_app_notification_settings != 0,
            time_sensitive_setting: raw.time_sensitive_setting,
            scheduled_delivery_setting: raw.scheduled_delivery_setting,
            direct_messages_setting: raw.direct_messages_setting
        )
    }
}
extension UNLocalizedStringPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNLocalizedNotificationStringRaw {
        UNLocalizedNotificationStringRaw(
            key: key.raw(arena),
            arguments: arguments.raw(arena)
        )
    }
    static func read(_ raw: UNLocalizedNotificationStringRaw) throws -> Self {
        Self(
            key: try String.read(raw.key),
            arguments: try [UNPropertyValue].read(raw.arguments)
        )
    }
}
extension UNNotificationSoundPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNNotificationSoundPayloadRaw {
        UNNotificationSoundPayloadRaw(
            kind: ["unknown": UInt32(0), "default": UInt32(1), "defaultCritical": UInt32(2), "defaultCriticalWithVolume": UInt32(3), "named": UInt32(4), "criticalNamed": UInt32(5)][kind] ?? 0,
            name: name.map { arena.keep($0.raw(arena)) },
            volume: volume.map { arena.keep($0) }
        )
    }
    static func read(_ raw: UNNotificationSoundPayloadRaw) throws -> Self {
        Self(
            kind: [UInt32(0): "unknown", UInt32(1): "default", UInt32(2): "defaultCritical", UInt32(3): "defaultCriticalWithVolume", UInt32(4): "named", UInt32(5): "criticalNamed"][raw.kind] ?? "unknown",
            name: try raw.name.map { try String.read($0.pointee) },
            volume: raw.volume.map { $0.pointee }
        )
    }
}
extension UNNotificationContentPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNNotificationContentPayloadRaw {
        UNNotificationContentPayloadRaw(
            title: title.raw(arena),
            subtitle: subtitle.raw(arena),
            body: body.raw(arena),
            badge: badge.map { arena.keep($0) },
            category_identifier: category_identifier.raw(arena),
            thread_identifier: thread_identifier.raw(arena),
            user_info: user_info.map { arena.keep($0.raw(arena)) },
            sound: sound.map { arena.keep($0.raw(arena)) },
            attachments: attachments.raw(arena),
            summary_argument: summary_argument.raw(arena),
            summary_argument_count: summary_argument_count,
            interruption_level: interruption_level.map { arena.keep($0) },
            relevance_score: relevance_score.map { arena.keep($0) },
            filter_criteria: filter_criteria.map { arena.keep($0.raw(arena)) },
            localized_title: localized_title.map { arena.keep($0.raw(arena)) },
            localized_subtitle: localized_subtitle.map { arena.keep($0.raw(arena)) },
            localized_body: localized_body.map { arena.keep($0.raw(arena)) },
            localized_summary_argument: localized_summary_argument.map { arena.keep($0.raw(arena)) }
        )
    }
    static func read(_ raw: UNNotificationContentPayloadRaw) throws -> Self {
        Self(
            title: try String.read(raw.title),
            subtitle: try String.read(raw.subtitle),
            body: try String.read(raw.body),
            badge: raw.badge.map { $0.pointee },
            category_identifier: try String.read(raw.category_identifier),
            thread_identifier: try String.read(raw.thread_identifier),
            user_info: try raw.user_info.map { try [String: UNPropertyValue].read($0.pointee) },
            sound: try raw.sound.map { try UNNotificationSoundPayload.read($0.pointee) },
            attachments: try [UNNotificationAttachmentPayload].read(raw.attachments),
            summary_argument: try String.read(raw.summary_argument),
            summary_argument_count: raw.summary_argument_count,
            interruption_level: raw.interruption_level.map { $0.pointee },
            relevance_score: raw.relevance_score.map { $0.pointee },
            filter_criteria: try raw.filter_criteria.map { try String.read($0.pointee) },
            localized_title: try raw.localized_title.map { try UNLocalizedStringPayload.read($0.pointee) },
            localized_subtitle: try raw.localized_subtitle.map { try UNLocalizedStringPayload.read($0.pointee) },
            localized_body: try raw.localized_body.map { try UNLocalizedStringPayload.read($0.pointee) },
            localized_summary_argument: try raw.localized_summary_argument.map { try UNLocalizedStringPayload.read($0.pointee) }
        )
    }
}
extension UNNotificationMessagePersonPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNNotificationMessagePersonPayloadRaw {
        UNNotificationMessagePersonPayloadRaw(
            handle: handle.raw(arena),
            handle_type: handle_type,
            display_name: display_name.map { arena.keep($0.raw(arena)) },
            contact_identifier: contact_identifier.map { arena.keep($0.raw(arena)) },
            custom_identifier: custom_identifier.map { arena.keep($0.raw(arena)) },
            is_me: is_me ? 1 : 0
        )
    }
    static func read(_ raw: UNNotificationMessagePersonPayloadRaw) throws -> Self {
        Self(
            handle: try String.read(raw.handle),
            handle_type: raw.handle_type,
            display_name: try raw.display_name.map { try String.read($0.pointee) },
            contact_identifier: try raw.contact_identifier.map { try String.read($0.pointee) },
            custom_identifier: try raw.custom_identifier.map { try String.read($0.pointee) },
            is_me: raw.is_me != 0
        )
    }
}
extension UNNotificationAttributedMessageContextPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNNotificationAttributedMessageContextPayloadRaw {
        UNNotificationAttributedMessageContextPayloadRaw(
            sender: sender.map { arena.keep($0.raw(arena)) },
            recipients: recipients.raw(arena),
            attributed_content: attributed_content.raw(arena),
            content: content.map { arena.keep($0.raw(arena)) },
            outgoing_message_type: outgoing_message_type,
            conversation_identifier: conversation_identifier.map { arena.keep($0.raw(arena)) },
            service_name: service_name.map { arena.keep($0.raw(arena)) },
            group_name: group_name.map { arena.keep($0.raw(arena)) }
        )
    }
    static func read(_ raw: UNNotificationAttributedMessageContextPayloadRaw) throws -> Self {
        Self(
            sender: try raw.sender.map { try UNNotificationMessagePersonPayload.read($0.pointee) },
            recipients: try [UNNotificationMessagePersonPayload].read(raw.recipients),
            attributed_content: try String.read(raw.attributed_content),
            content: try raw.content.map { try String.read($0.pointee) },
            outgoing_message_type: raw.outgoing_message_type,
            conversation_identifier: try raw.conversation_identifier.map { try String.read($0.pointee) },
            service_name: try raw.service_name.map { try String.read($0.pointee) },
            group_name: try raw.group_name.map { try String.read($0.pointee) }
        )
    }
}
extension UNNotificationContentProviderPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNNotificationContentProviderPayloadRaw {
        UNNotificationContentProviderPayloadRaw(
            kind: ["unknown": UInt32(0), "attributedMessageContext": UInt32(1)][kind] ?? 0,
            attributed_message_context: attributed_message_context.map { arena.keep($0.raw(arena)) }
        )
    }
    static func read(_ raw: UNNotificationContentProviderPayloadRaw) throws -> Self {
        Self(
            kind: [UInt32(0): "unknown", UInt32(1): "attributedMessageContext"][raw.kind] ?? "unknown",
            attributed_message_context: try raw.attributed_message_context.map { try UNNotificationAttributedMessageContextPayload.read($0.pointee) }
        )
    }
}
extension UNAttachmentThumbnailClippingRectPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNAttachmentThumbnailClippingRectPayloadRaw {
        UNAttachmentThumbnailClippingRectPayloadRaw(
            x: x,
            y: y,
            width: width,
            height: height
        )
    }
    static func read(_ raw: UNAttachmentThumbnailClippingRectPayloadRaw) throws -> Self {
        Self(
            x: raw.x,
            y: raw.y,
            width: raw.width,
            height: raw.height
        )
    }
}
extension UNNotificationAttachmentOptionsPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNNotificationAttachmentOptionsPayloadRaw {
        UNNotificationAttachmentOptionsPayloadRaw(
            type_hint: type_hint.map { arena.keep($0.raw(arena)) },
            thumbnail_hidden: thumbnail_hidden ? 1 : 0,
            thumbnail_clipping_rect: thumbnail_clipping_rect.map { arena.keep($0.raw(arena)) },
            thumbnail_time: thumbnail_time.map { arena.keep($0.raw(arena)) }
        )
    }
    static func read(_ raw: UNNotificationAttachmentOptionsPayloadRaw) throws -> Self {
        Self(
            type_hint: try raw.type_hint.map { try String.read($0.pointee) },
            thumbnail_hidden: raw.thumbnail_hidden != 0,
            thumbnail_clipping_rect: try raw.thumbnail_clipping_rect.map { try UNAttachmentThumbnailClippingRectPayload.read($0.pointee) },
            thumbnail_time: try raw.thumbnail_time.map { try UNAttachmentThumbnailTimePayload.read($0.pointee) }
        )
    }
}
extension UNNotificationAttachmentPayload: UNABIValue {
    func raw(_ arena: UNArena) -> UNNotificationAttachmentPayloadRaw {
        UNNotificationAttachmentPayloadRaw(
            identifier: identifier.raw(arena),
            file_path: file_path.raw(arena),
            attachment_type: attachment_type.map { arena.keep($0.raw(arena)) },
            options: options.raw(arena)
        )
    }
    static func read(_ raw: UNNotificationAttachmentPayloadRaw) throws -> Self {
        Self(
            identifier: try String.read(raw.identifier),
            file_path: try String.read(raw.file_path),
            attachment_type: try raw.attachment_type.map { try String.read($0.pointee) },
            options: try UNNotificationAttachmentOptionsPayload.read(raw.options)
        )
    }
}
