use usernotifications::prelude::*;

unsafe extern "C" {
    fn free(ptr: *mut core::ffi::c_void);
}

#[test]
fn time_interval_roundtrip_preserves_values() {
    let roundtrip = NotificationTrigger::time_interval(90.0, false)
        .bridge_roundtrip()
        .expect("time interval trigger should roundtrip");
    assert!(matches!(roundtrip, NotificationTrigger::TimeInterval(_)));
}

#[test]
fn calendar_roundtrip_preserves_components() {
    let roundtrip =
        NotificationTrigger::calendar(DateComponents::new().with_hour(8).with_minute(15), true)
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
    for (time_interval, repeats) in [(0.0, 0_u8), (-1.0, 0), (30.0, 1)] {
        let trigger = usernotifications::ffi::abi::UNNotificationTriggerPayloadRaw {
            kind: 1,
            repeats: &raw const repeats,
            time_interval: &raw const time_interval,
            date_components: core::ptr::null(),
            next_trigger_date: core::ptr::null(),
            class_name: core::ptr::null(),
        };
        let payload = usernotifications::ffi::abi::UNBufferRaw {
            data: (&raw const trigger).cast(),
            size: core::mem::size_of_val(&trigger),
            owner: core::ptr::null_mut(),
            release: None,
        };
        let mut error = std::ptr::null_mut();
        let result = unsafe {
            usernotifications::ffi::trigger::un_trigger_roundtrip_binary(
                &raw const payload,
                &raw mut error,
            )
        };
        assert!(result.is_null());
        assert!(!error.is_null());
        unsafe { free(error.cast()) };
    }
}
