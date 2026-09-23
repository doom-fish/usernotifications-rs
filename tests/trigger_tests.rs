use usernotifications::prelude::*;

#[test]
fn time_interval_roundtrip_preserves_values() {
    let roundtrip = NotificationTrigger::time_interval(90.0, false)
        .bridge_roundtrip()
        .expect("time interval trigger should roundtrip");
    assert!(matches!(roundtrip, NotificationTrigger::TimeInterval(_)));
}

#[test]
fn calendar_roundtrip_preserves_components() {
    let roundtrip = NotificationTrigger::calendar(
        DateComponents::new().with_hour(8).with_minute(15),
        true,
    )
    .bridge_roundtrip()
    .expect("calendar trigger should roundtrip");
    assert!(matches!(roundtrip, NotificationTrigger::Calendar(_)));
}

#[test]
fn invalid_time_intervals_are_errors_instead_of_objc_exceptions() {
    for seconds in [0.0, -5.0, f64::NAN, f64::INFINITY] {
        let error = NotificationTrigger::time_interval(seconds, false)
            .bridge_roundtrip()
            .expect_err("an invalid interval must be rejected");
        assert!(matches!(error, UserNotificationsError::InvalidArgument(_)));
    }

    let request = NotificationRequest::new(
        "invalid-interval",
        NotificationContent::new("Title", "Body"),
        Some(NotificationTrigger::time_interval(0.0, false)),
    );
    assert!(matches!(
        request.bridge_roundtrip(),
        Err(UserNotificationsError::InvalidArgument(_))
    ));
}

#[test]
fn the_swift_bridge_rejects_invalid_intervals_that_bypass_rust_validation() {
    for payload in [
        c"{\"kind\":\"timeInterval\",\"time_interval\":0.0,\"repeats\":false}",
        c"{\"kind\":\"timeInterval\",\"time_interval\":-1.0,\"repeats\":false}",
        c"{\"kind\":\"timeInterval\",\"time_interval\":30.0,\"repeats\":true}",
    ] {
        let mut error = std::ptr::null_mut();
        let result = unsafe {
            usernotifications::ffi::trigger::un_trigger_roundtrip_json(
                payload.as_ptr(),
                &raw mut error,
            )
        };
        assert!(result.is_null());
        assert!(!error.is_null());
        unsafe { libc::free(error.cast()) };
    }
}
