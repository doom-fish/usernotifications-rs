use usernotifications::prelude::*;

#[test]
fn current_center_requires_app_bundle() {
    let Err(error) = UserNotificationCenter::current() else {
        panic!("test binaries are not app bundles");
    };
    assert!(
        error
            .message()
            .contains("UserNotifications requires running from a macOS app bundle")
    );
}

#[test]
fn callbacks_builder_supports_will_present() {
    let _callbacks = UserNotificationCenterCallbacks::new().on_will_present(|_| {
        NotificationPresentationOptions::BANNER | NotificationPresentationOptions::SOUND
    });
}
