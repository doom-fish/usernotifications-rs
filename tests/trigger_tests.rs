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
