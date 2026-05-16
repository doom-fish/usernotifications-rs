use usernotifications::prelude::*;

#[test]
fn framework_error_constants_match_sdk_values() {
    assert_eq!(USER_NOTIFICATIONS_ERROR_DOMAIN, "UNErrorDomain");
    assert_eq!(
        UserNotificationsFrameworkErrorCode::from_raw(1501),
        Some(UserNotificationsFrameworkErrorCode::ContentProvidingInvalid),
    );
    assert_eq!(UserNotificationsFrameworkErrorCode::from_raw(9999), None);
}
