use usernotifications::prelude::*;

#[test]
fn content_roundtrip_preserves_summary_and_interruption() {
    let content = NotificationContent::new("Title", "Body")
        .with_summary_argument("Summary")
        .with_summary_argument_count(2)
        .with_interruption_level(NotificationInterruptionLevel::TimeSensitive)
        .with_relevance_score(0.5)
        .with_filter_criteria("filter");
    let roundtrip = content
        .bridge_roundtrip()
        .expect("content roundtrip should succeed");
    assert_eq!(roundtrip.summary_argument, "Summary");
    assert_eq!(roundtrip.summary_argument_count, 2);
    assert_eq!(roundtrip.interruption_level, Some(NotificationInterruptionLevel::TimeSensitive));
}
