use usernotifications::prelude::*;

#[test]
fn authorization_status_from_raw_matches_known_values() {
    assert_eq!(AuthorizationStatus::from_raw(0), AuthorizationStatus::NotDetermined);
    assert_eq!(AuthorizationStatus::from_raw(1), AuthorizationStatus::Denied);
    assert_eq!(AuthorizationStatus::from_raw(2), AuthorizationStatus::Authorized);
    assert_eq!(AuthorizationStatus::from_raw(3), AuthorizationStatus::Provisional);
}

#[test]
fn notification_setting_from_raw_matches_known_values() {
    assert_eq!(NotificationSetting::from_raw(0), NotificationSetting::NotSupported);
    assert_eq!(NotificationSetting::from_raw(1), NotificationSetting::Disabled);
    assert_eq!(NotificationSetting::from_raw(2), NotificationSetting::Enabled);
}
