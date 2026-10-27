#ifndef USER_NOTIFICATIONS_ABI_H
#define USER_NOTIFICATIONS_ABI_H
#include <stddef.h>
#include <stdint.h>
typedef struct { const uint8_t *data; size_t len; } UNStringRaw;
typedef struct { const void *data; size_t len; } UNSliceRaw;
typedef struct UNBufferRaw UNBufferRaw;
struct UNBufferRaw { const void *data; size_t size; void *owner; void (*release)(UNBufferRaw *); };
typedef struct { uint32_t tag; int64_t signed_value; uint64_t unsigned_value; double real_value; UNStringRaw string; UNSliceRaw values; UNSliceRaw entries; } UNValueRaw;
typedef struct { UNStringRaw key; UNValueRaw value; } UNDictionaryEntryRaw;
typedef struct { UNSliceRaw entries; } UNUserInfoRaw;
typedef struct { uint32_t tag; double seconds; uint64_t frame; } UNAttachmentThumbnailTimePayloadRaw;
typedef struct UNNotificationResponsePayloadRaw UNNotificationResponsePayloadRaw;
typedef struct UNNotificationRequestPayloadRaw UNNotificationRequestPayloadRaw;
typedef struct UNNotificationActionIconPayloadRaw UNNotificationActionIconPayloadRaw;
typedef struct UNNotificationActionPayloadRaw UNNotificationActionPayloadRaw;
typedef struct UNNotificationPayloadRaw UNNotificationPayloadRaw;
typedef struct UNCenterEventPayloadRaw UNCenterEventPayloadRaw;
typedef struct UNDateComponentsRaw UNDateComponentsRaw;
typedef struct UNNotificationTriggerPayloadRaw UNNotificationTriggerPayloadRaw;
typedef struct UNNotificationCategoryPayloadRaw UNNotificationCategoryPayloadRaw;
typedef struct UNNotificationSettingsPayloadRaw UNNotificationSettingsPayloadRaw;
typedef struct UNLocalizedNotificationStringRaw UNLocalizedNotificationStringRaw;
typedef struct UNNotificationSoundPayloadRaw UNNotificationSoundPayloadRaw;
typedef struct UNNotificationContentPayloadRaw UNNotificationContentPayloadRaw;
typedef struct UNNotificationMessagePersonPayloadRaw UNNotificationMessagePersonPayloadRaw;
typedef struct UNNotificationAttributedMessageContextPayloadRaw UNNotificationAttributedMessageContextPayloadRaw;
typedef struct UNNotificationContentProviderPayloadRaw UNNotificationContentProviderPayloadRaw;
typedef struct UNAttachmentThumbnailClippingRectPayloadRaw UNAttachmentThumbnailClippingRectPayloadRaw;
typedef struct UNNotificationAttachmentOptionsPayloadRaw UNNotificationAttachmentOptionsPayloadRaw;
typedef struct UNNotificationAttachmentPayloadRaw UNNotificationAttachmentPayloadRaw;
struct UNNotificationContentPayloadRaw {
    UNStringRaw title;
    UNStringRaw subtitle;
    UNStringRaw body;
    const int64_t * badge;
    UNStringRaw category_identifier;
    UNStringRaw thread_identifier;
    const UNUserInfoRaw * user_info;
    const UNNotificationSoundPayloadRaw * sound;
    UNSliceRaw attachments;
    UNStringRaw summary_argument;
    uint64_t summary_argument_count;
    const int32_t * interruption_level;
    const double * relevance_score;
    const UNStringRaw * filter_criteria;
    const UNLocalizedNotificationStringRaw * localized_title;
    const UNLocalizedNotificationStringRaw * localized_subtitle;
    const UNLocalizedNotificationStringRaw * localized_body;
    const UNLocalizedNotificationStringRaw * localized_summary_argument;
};
struct UNNotificationRequestPayloadRaw {
    UNStringRaw identifier;
    UNNotificationContentPayloadRaw content;
    const UNNotificationTriggerPayloadRaw * trigger;
};
struct UNNotificationPayloadRaw {
    double date;
    UNNotificationRequestPayloadRaw request;
};
struct UNNotificationResponsePayloadRaw {
    UNStringRaw action_identifier;
    UNNotificationPayloadRaw notification;
    const UNStringRaw * user_text;
};
struct UNNotificationActionIconPayloadRaw {
    uint32_t kind;
    const UNStringRaw * name;
};
struct UNNotificationActionPayloadRaw {
    UNStringRaw identifier;
    UNStringRaw title;
    uint64_t options;
    const UNNotificationActionIconPayloadRaw * icon;
    const UNStringRaw * text_input_button_title;
    const UNStringRaw * text_input_placeholder;
    const UNLocalizedNotificationStringRaw * localized_title;
    const UNLocalizedNotificationStringRaw * localized_text_input_button_title;
    const UNLocalizedNotificationStringRaw * localized_text_input_placeholder;
};
struct UNCenterEventPayloadRaw {
    uint32_t event;
    const UNNotificationPayloadRaw * notification;
    const UNNotificationResponsePayloadRaw * response;
};
struct UNDateComponentsRaw {
    const int64_t * year;
    const int64_t * month;
    const int64_t * day;
    const int64_t * hour;
    const int64_t * minute;
    const int64_t * second;
    const int64_t * weekday;
};
struct UNNotificationTriggerPayloadRaw {
    uint32_t kind;
    const uint8_t * repeats;
    const double * time_interval;
    const UNDateComponentsRaw * date_components;
    const double * next_trigger_date;
    const UNStringRaw * class_name;
};
struct UNNotificationCategoryPayloadRaw {
    UNStringRaw identifier;
    UNSliceRaw actions;
    UNSliceRaw intent_identifiers;
    uint64_t options;
    const UNStringRaw * hidden_previews_body_placeholder;
    const UNStringRaw * category_summary_format;
    const UNLocalizedNotificationStringRaw * localized_hidden_previews_body_placeholder;
    const UNLocalizedNotificationStringRaw * localized_category_summary_format;
};
struct UNNotificationSettingsPayloadRaw {
    int32_t authorization_status;
    int32_t sound_setting;
    int32_t badge_setting;
    int32_t alert_setting;
    int32_t notification_center_setting;
    int32_t lock_screen_setting;
    int32_t alert_style;
    int32_t show_previews_setting;
    int32_t critical_alert_setting;
    uint8_t provides_app_notification_settings;
    int32_t time_sensitive_setting;
    int32_t scheduled_delivery_setting;
    int32_t direct_messages_setting;
};
struct UNLocalizedNotificationStringRaw {
    UNStringRaw key;
    UNSliceRaw arguments;
};
struct UNNotificationSoundPayloadRaw {
    uint32_t kind;
    const UNStringRaw * name;
    const float * volume;
};
struct UNNotificationMessagePersonPayloadRaw {
    UNStringRaw handle;
    int32_t handle_type;
    const UNStringRaw * display_name;
    const UNStringRaw * contact_identifier;
    const UNStringRaw * custom_identifier;
    uint8_t is_me;
};
struct UNNotificationAttributedMessageContextPayloadRaw {
    const UNNotificationMessagePersonPayloadRaw * sender;
    UNSliceRaw recipients;
    UNStringRaw attributed_content;
    const UNStringRaw * content;
    int32_t outgoing_message_type;
    const UNStringRaw * conversation_identifier;
    const UNStringRaw * service_name;
    const UNStringRaw * group_name;
};
struct UNNotificationContentProviderPayloadRaw {
    uint32_t kind;
    const UNNotificationAttributedMessageContextPayloadRaw * attributed_message_context;
};
struct UNAttachmentThumbnailClippingRectPayloadRaw {
    double x;
    double y;
    double width;
    double height;
};
struct UNNotificationAttachmentOptionsPayloadRaw {
    const UNStringRaw * type_hint;
    uint8_t thumbnail_hidden;
    const UNAttachmentThumbnailClippingRectPayloadRaw * thumbnail_clipping_rect;
    const UNAttachmentThumbnailTimePayloadRaw * thumbnail_time;
};
struct UNNotificationAttachmentPayloadRaw {
    UNStringRaw identifier;
    UNStringRaw file_path;
    const UNStringRaw * attachment_type;
    UNNotificationAttachmentOptionsPayloadRaw options;
};
#endif
