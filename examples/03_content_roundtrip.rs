use usernotifications::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = NotificationContent::new("Content title", "Content body")
        .with_subtitle("Subtitle")
        .with_summary_argument("Summary")
        .with_summary_argument_count(2)
        .with_interruption_level(NotificationInterruptionLevel::TimeSensitive)
        .with_relevance_score(0.75)
        .with_filter_criteria("example-filter")
        .with_sound(NotificationSound::DefaultCritical);
    let roundtrip = content.bridge_roundtrip()?;
    println!("title = {}", roundtrip.title);
    println!("summary_argument_count = {}", roundtrip.summary_argument_count);
    println!("interruption_level = {:?}", roundtrip.interruption_level);
    println!("✅ UNNotificationContent roundtrip OK");
    Ok(())
}
