use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::binary::decode_binary;
use crate::error::{from_swift, UserNotificationsError};
use crate::ffi;

/// Wraps the date components used by `UNCalendarNotificationTrigger`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DateComponents {
    /// The year.
    pub year: Option<i64>,
    /// The month.
    pub month: Option<i64>,
    /// The day.
    pub day: Option<i64>,
    /// The hour.
    pub hour: Option<i64>,
    /// The minute.
    pub minute: Option<i64>,
    /// The second.
    pub second: Option<i64>,
    /// The weekday.
    pub weekday: Option<i64>,
}

impl DateComponents {
    /// Creates empty date components.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets year.
    #[must_use]
    pub fn with_year(mut self, year: i64) -> Self {
        self.year = Some(year);
        self
    }

    /// Sets month.
    #[must_use]
    pub fn with_month(mut self, month: i64) -> Self {
        self.month = Some(month);
        self
    }

    /// Sets day.
    #[must_use]
    pub fn with_day(mut self, day: i64) -> Self {
        self.day = Some(day);
        self
    }

    /// Sets hour.
    #[must_use]
    pub fn with_hour(mut self, hour: i64) -> Self {
        self.hour = Some(hour);
        self
    }

    /// Sets minute.
    #[must_use]
    pub fn with_minute(mut self, minute: i64) -> Self {
        self.minute = Some(minute);
        self
    }

    /// Sets second.
    #[must_use]
    pub fn with_second(mut self, second: i64) -> Self {
        self.second = Some(second);
        self
    }

    /// Sets weekday.
    #[must_use]
    pub fn with_weekday(mut self, weekday: i64) -> Self {
        self.weekday = Some(weekday);
        self
    }
}

/// Wraps `UNTimeIntervalNotificationTrigger`.
#[derive(Debug, Clone, PartialEq)]
pub struct TimeIntervalTrigger {
    /// The time interval.
    pub time_interval: f64,
    /// Whether the trigger repeats.
    pub repeats: bool,
    /// The next trigger date.
    pub next_trigger_date: Option<SystemTime>,
}

impl TimeIntervalTrigger {
    /// Creates a time-interval trigger.
    #[must_use]
    pub fn new(time_interval: f64, repeats: bool) -> Self {
        Self {
            time_interval,
            repeats,
            next_trigger_date: None,
        }
    }
}

/// Wraps `UNCalendarNotificationTrigger`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarTrigger {
    /// The date components.
    pub date_components: DateComponents,
    /// Whether the trigger repeats.
    pub repeats: bool,
    /// The next trigger date.
    pub next_trigger_date: Option<SystemTime>,
}

impl CalendarTrigger {
    /// Creates a calendar trigger.
    #[must_use]
    pub fn new(date_components: DateComponents, repeats: bool) -> Self {
        Self {
            date_components,
            repeats,
            next_trigger_date: None,
        }
    }
}

/// Wraps `UNNotificationTrigger` and its concrete subclasses.
#[derive(Debug, Clone, PartialEq)]
pub enum NotificationTrigger {
    /// Represents a `UNTimeIntervalNotificationTrigger`.
    TimeInterval(TimeIntervalTrigger),
    /// Represents a `UNCalendarNotificationTrigger`.
    Calendar(CalendarTrigger),
    /// Represents a push notification trigger.
    Push,
    /// Represents an unmodeled `UNNotificationTrigger` subclass.
    Unknown {
        /// The runtime Objective-C class name.
        class_name: String,
        /// Whether the trigger repeats.
        repeats: bool,
    },
}

impl NotificationTrigger {
    /// Creates a time-interval notification trigger.
    #[must_use]
    pub fn time_interval(seconds: f64, repeats: bool) -> Self {
        Self::TimeInterval(TimeIntervalTrigger::new(seconds, repeats))
    }

    /// Creates a calendar notification trigger.
    #[must_use]
    pub fn calendar(date_components: DateComponents, repeats: bool) -> Self {
        Self::Calendar(CalendarTrigger::new(date_components, repeats))
    }

    /// Creates a push notification trigger marker.
    #[must_use]
    pub const fn push() -> Self {
        Self::Push
    }

    pub(crate) fn validate(&self) -> Result<(), UserNotificationsError> {
        if let Self::TimeInterval(trigger) = self {
            if !trigger.time_interval.is_finite() || trigger.time_interval <= 0.0 {
                return Err(UserNotificationsError::InvalidArgument(
                    "time interval triggers need a finite time_interval greater than 0 seconds"
                        .into(),
                ));
            }
            if trigger.repeats && trigger.time_interval < 60.0 {
                return Err(UserNotificationsError::InvalidArgument(
                    "repeating time interval triggers must be at least 60 seconds".into(),
                ));
            }
        }
        Ok(())
    }

    /// Round-trips this trigger through the Swift bridge.
    pub fn bridge_roundtrip(&self) -> Result<Self, UserNotificationsError> {
        let trigger = encode_trigger_binary(self)?;

        let mut error = core::ptr::null_mut();
        let payload =
            unsafe { ffi::trigger::un_trigger_roundtrip_binary(trigger.as_ptr(), &raw mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_trigger_binary(payload)
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NotificationTriggerPayload {
    kind: String,
    repeats: Option<bool>,
    time_interval: Option<f64>,
    date_components: Option<DateComponents>,
    next_trigger_date: Option<f64>,
    class_name: Option<String>,
}

impl From<&NotificationTrigger> for NotificationTriggerPayload {
    fn from(value: &NotificationTrigger) -> Self {
        match value {
            NotificationTrigger::TimeInterval(trigger) => Self {
                kind: "timeInterval".into(),
                repeats: Some(trigger.repeats),
                time_interval: Some(trigger.time_interval),
                date_components: None,
                next_trigger_date: timestamp_from_system_time_opt(
                    trigger.next_trigger_date.as_ref(),
                ),
                class_name: None,
            },
            NotificationTrigger::Calendar(trigger) => Self {
                kind: "calendar".into(),
                repeats: Some(trigger.repeats),
                time_interval: None,
                date_components: Some(trigger.date_components.clone()),
                next_trigger_date: timestamp_from_system_time_opt(
                    trigger.next_trigger_date.as_ref(),
                ),
                class_name: None,
            },
            NotificationTrigger::Push => Self {
                kind: "push".into(),
                repeats: Some(false),
                time_interval: None,
                date_components: None,
                next_trigger_date: None,
                class_name: None,
            },
            NotificationTrigger::Unknown {
                class_name,
                repeats,
            } => Self {
                kind: "unknown".into(),
                repeats: Some(*repeats),
                time_interval: None,
                date_components: None,
                next_trigger_date: None,
                class_name: Some(class_name.clone()),
            },
        }
    }
}

impl From<NotificationTriggerPayload> for NotificationTrigger {
    fn from(value: NotificationTriggerPayload) -> Self {
        match value.kind.as_str() {
            "timeInterval" => Self::TimeInterval(TimeIntervalTrigger {
                time_interval: value.time_interval.unwrap_or_default(),
                repeats: value.repeats.unwrap_or(false),
                next_trigger_date: system_time_from_timestamp_opt(value.next_trigger_date),
            }),
            "calendar" => Self::Calendar(CalendarTrigger {
                date_components: value.date_components.unwrap_or_default(),
                repeats: value.repeats.unwrap_or(false),
                next_trigger_date: system_time_from_timestamp_opt(value.next_trigger_date),
            }),
            "push" => Self::Push,
            _ => Self::Unknown {
                class_name: value
                    .class_name
                    .unwrap_or_else(|| "UNNotificationTrigger".into()),
                repeats: value.repeats.unwrap_or(false),
            },
        }
    }
}

pub(crate) fn encode_trigger_binary(
    trigger: &NotificationTrigger,
) -> Result<crate::binary::Input, UserNotificationsError> {
    trigger.validate()?;
    Ok(crate::binary::Input::new(
        &NotificationTriggerPayload::from(trigger),
    ))
}

pub(crate) fn decode_trigger_binary(
    ptr: *mut crate::ffi::abi::UNBufferRaw,
) -> Result<NotificationTrigger, UserNotificationsError> {
    decode_binary::<NotificationTriggerPayload>(ptr).map(Into::into)
}

#[allow(clippy::single_option_map)]
fn timestamp_from_system_time_opt(time: Option<&SystemTime>) -> Option<f64> {
    time.map(|time| {
        time.duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs_f64()
    })
}

#[allow(clippy::single_option_map)]
fn system_time_from_timestamp_opt(timestamp: Option<f64>) -> Option<SystemTime> {
    timestamp.and_then(|timestamp| {
        Duration::try_from_secs_f64(timestamp.max(0.0))
            .ok()
            .and_then(|offset| UNIX_EPOCH.checked_add(offset))
    })
}

impl crate::binary::Record for DateComponents {
    type Raw = crate::ffi::abi::UNDateComponentsRaw;
    fn raw(&self, arena: &mut crate::binary::Arena) -> Self::Raw {
        Self::Raw {
            year: self.year.as_ref().map_or(core::ptr::null(), |value| {
                let raw = *value;
                arena.keep(raw)
            }),
            month: self.month.as_ref().map_or(core::ptr::null(), |value| {
                let raw = *value;
                arena.keep(raw)
            }),
            day: self.day.as_ref().map_or(core::ptr::null(), |value| {
                let raw = *value;
                arena.keep(raw)
            }),
            hour: self.hour.as_ref().map_or(core::ptr::null(), |value| {
                let raw = *value;
                arena.keep(raw)
            }),
            minute: self.minute.as_ref().map_or(core::ptr::null(), |value| {
                let raw = *value;
                arena.keep(raw)
            }),
            second: self.second.as_ref().map_or(core::ptr::null(), |value| {
                let raw = *value;
                arena.keep(raw)
            }),
            weekday: self.weekday.as_ref().map_or(core::ptr::null(), |value| {
                let raw = *value;
                arena.keep(raw)
            }),
        }
    }
    unsafe fn read(raw: &Self::Raw) -> Result<Self, UserNotificationsError> {
        Ok(Self {
            year: unsafe { raw.year.as_ref() }.copied(),
            month: unsafe { raw.month.as_ref() }.copied(),
            day: unsafe { raw.day.as_ref() }.copied(),
            hour: unsafe { raw.hour.as_ref() }.copied(),
            minute: unsafe { raw.minute.as_ref() }.copied(),
            second: unsafe { raw.second.as_ref() }.copied(),
            weekday: unsafe { raw.weekday.as_ref() }.copied(),
        })
    }
}

impl crate::binary::Record for NotificationTriggerPayload {
    type Raw = crate::ffi::abi::UNNotificationTriggerPayloadRaw;
    fn raw(&self, arena: &mut crate::binary::Arena) -> Self::Raw {
        Self::Raw {
            kind: match self.kind.as_str() {
                "timeInterval" => 1,
                "calendar" => 2,
                "push" => 3,
                "location" => 4,
                "unknown" => 5,
                _ => 0,
            },
            repeats: self.repeats.as_ref().map_or(core::ptr::null(), |value| {
                let raw = u8::from(*value);
                arena.keep(raw)
            }),
            time_interval: self
                .time_interval
                .as_ref()
                .map_or(core::ptr::null(), |value| {
                    let raw = *value;
                    arena.keep(raw)
                }),
            date_components: self
                .date_components
                .as_ref()
                .map_or(core::ptr::null(), |value| {
                    let raw = crate::binary::Record::raw(value, arena);
                    arena.keep(raw)
                }),
            next_trigger_date: self
                .next_trigger_date
                .as_ref()
                .map_or(core::ptr::null(), |value| {
                    let raw = *value;
                    arena.keep(raw)
                }),
            class_name: self.class_name.as_ref().map_or(core::ptr::null(), |value| {
                let raw = crate::binary::Record::raw(value, arena);
                arena.keep(raw)
            }),
        }
    }
    unsafe fn read(raw: &Self::Raw) -> Result<Self, UserNotificationsError> {
        Ok(Self {
            kind: match raw.kind {
                0 => "none",
                1 => "timeInterval",
                2 => "calendar",
                3 => "push",
                4 => "location",
                5 => "unknown",
                _ => return Err(crate::binary::invalid_data()),
            }
            .to_owned(),
            repeats: unsafe { raw.repeats.as_ref() }.map(|value| *value != 0),
            time_interval: unsafe { raw.time_interval.as_ref() }.copied(),
            date_components: unsafe { raw.date_components.as_ref() }
                .map(|value| unsafe { <DateComponents as crate::binary::Record>::read(value) })
                .transpose()?,
            next_trigger_date: unsafe { raw.next_trigger_date.as_ref() }.copied(),
            class_name: unsafe { raw.class_name.as_ref() }
                .map(|value| unsafe { <String as crate::binary::Record>::read(value) })
                .transpose()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        encode_trigger_binary, system_time_from_timestamp_opt, CalendarTrigger, DateComponents,
        NotificationTrigger, TimeIntervalTrigger,
    };
    use crate::error::UserNotificationsError;

    #[test]
    fn time_interval_triggers_reject_intervals_the_framework_would_abort_on() {
        for (seconds, repeats) in [
            (0.0, false),
            (-1.0, false),
            (f64::NAN, false),
            (f64::INFINITY, false),
            (f64::NEG_INFINITY, false),
            (59.9, true),
        ] {
            let trigger = NotificationTrigger::time_interval(seconds, repeats);
            assert!(
                matches!(
                    encode_trigger_binary(&trigger),
                    Err(UserNotificationsError::InvalidArgument(_))
                ),
                "{seconds} s (repeats: {repeats}) must be rejected"
            );
        }
        assert!(encode_trigger_binary(&NotificationTrigger::time_interval(0.5, false)).is_ok());
        assert!(encode_trigger_binary(&NotificationTrigger::time_interval(60.0, true)).is_ok());
    }

    #[test]
    fn out_of_range_trigger_dates_decode_without_panicking() {
        assert_eq!(system_time_from_timestamp_opt(Some(f64::MAX)), None);
        assert_eq!(
            system_time_from_timestamp_opt(Some(f64::NAN)),
            Some(std::time::UNIX_EPOCH)
        );
        assert!(system_time_from_timestamp_opt(Some(1_000.0)).is_some());
        assert_eq!(system_time_from_timestamp_opt(None), None);
    }

    #[test]
    fn date_components_round_trip_preserves_values() {
        let components = DateComponents::new()
            .with_year(2026)
            .with_month(5)
            .with_day(20)
            .with_hour(12)
            .with_minute(34)
            .with_second(56)
            .with_weekday(4);

        let encoded = crate::binary::Input::new(&components);
        let roundtrip: DateComponents = unsafe { crate::binary::read(encoded.as_ptr()) }.unwrap();

        assert_eq!(roundtrip, components);
    }

    #[test]
    fn time_interval_trigger_new_sets_expected_fields() {
        let trigger = TimeIntervalTrigger::new(60.0, true);

        assert!((trigger.time_interval - 60.0).abs() < f64::EPSILON);
        assert!(trigger.repeats);
        assert!(trigger.next_trigger_date.is_none());
    }

    #[test]
    fn calendar_trigger_new_sets_expected_fields() {
        let components = DateComponents::new()
            .with_month(5)
            .with_day(20)
            .with_hour(9);
        let trigger = CalendarTrigger::new(components.clone(), false);

        assert_eq!(trigger.date_components, components);
        assert!(!trigger.repeats);
        assert!(trigger.next_trigger_date.is_none());
    }

    #[test]
    fn notification_trigger_time_interval_helper_wraps_constructor() {
        let trigger = NotificationTrigger::time_interval(30.0, true);

        assert_eq!(
            trigger,
            NotificationTrigger::TimeInterval(TimeIntervalTrigger::new(30.0, true)),
        );
    }

    #[test]
    fn notification_trigger_calendar_and_push_helpers_wrap_expected_variants() {
        let components = DateComponents::new().with_month(5).with_day(20);

        assert_eq!(
            NotificationTrigger::calendar(components.clone(), false),
            NotificationTrigger::Calendar(CalendarTrigger::new(components, false)),
        );
        assert_eq!(NotificationTrigger::push(), NotificationTrigger::Push);
    }
}
