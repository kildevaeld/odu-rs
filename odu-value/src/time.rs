use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Time {
    Time(NaiveTime),
    Date(NaiveDate),
    DateTime(NaiveDateTime),
}

impl Time {
    pub fn as_time(&self) -> Option<NaiveTime> {
        match self {
            Time::DateTime(t) => Some(t.time()),
            Time::Time(t) => Some(*t),
            _ => None,
        }
    }

    pub fn as_date(&self) -> Option<NaiveDate> {
        match self {
            Time::DateTime(t) => Some(t.date()),
            Time::Date(t) => Some(*t),
            _ => None,
        }
    }

    pub fn as_datetime(&self) -> Option<NaiveDateTime> {
        match self {
            Time::DateTime(t) => Some(*t),
            _ => None,
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Time::Time(t) => write!(f, "{}", t),
            Time::Date(t) => write!(f, "{}", t),
            Time::DateTime(t) => write!(f, "{}", t),
        }
    }
}

impl From<NaiveDate> for Time {
    fn from(value: NaiveDate) -> Self {
        Time::Date(value)
    }
}

impl From<NaiveDateTime> for Time {
    fn from(value: NaiveDateTime) -> Self {
        Time::DateTime(value)
    }
}

impl From<NaiveTime> for Time {
    fn from(value: NaiveTime) -> Self {
        Time::Time(value)
    }
}

impl<T: TimeZone> From<DateTime<T>> for Time {
    fn from(value: DateTime<T>) -> Self {
        Time::DateTime(value.naive_utc())
    }
}
