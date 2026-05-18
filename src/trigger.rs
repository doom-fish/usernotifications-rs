use core::ffi::c_char;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::error::{from_swift, UserNotificationsError};
use crate::ffi;
use crate::private::{decode_json, to_cstring};

/// Wraps the date components used by `UNCalendarNotificationTrigger`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
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

    /// Round-trips this trigger through the Swift bridge.
    pub fn bridge_roundtrip(&self) -> Result<Self, UserNotificationsError> {
        let trigger = encode_trigger_json(self)?;
        let trigger = to_cstring(&trigger)?;
        let mut error = core::ptr::null_mut();
        let payload =
            unsafe { ffi::trigger::un_trigger_roundtrip_json(trigger.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(from_swift(ffi::status::FRAMEWORK_ERROR, error))
        } else {
            decode_trigger_json(payload)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub(crate) fn encode_trigger_json(
    trigger: &NotificationTrigger,
) -> Result<String, UserNotificationsError> {
    serde_json::to_string(&NotificationTriggerPayload::from(trigger)).map_err(|error| {
        UserNotificationsError::FrameworkError(format!(
            "failed to encode notification trigger: {error}",
        ))
    })
}

pub(crate) fn decode_trigger_json(
    ptr: *mut c_char,
) -> Result<NotificationTrigger, UserNotificationsError> {
    decode_json::<NotificationTriggerPayload>(ptr).map(Into::into)
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
    timestamp.map(|timestamp| UNIX_EPOCH + Duration::from_secs_f64(timestamp.max(0.0)))
}
