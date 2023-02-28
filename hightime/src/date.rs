#[cfg(feature = "std")]
use std::time::SystemTime;

use crate::Error;

/// A day of the week.
pub enum Weekday {
    #[allow(missing_docs)]
    Monday = 1,
    #[allow(missing_docs)]
    Tuesday = 2,
    #[allow(missing_docs)]
    Wednesday = 3,
    #[allow(missing_docs)]
    Thursday = 4,
    #[allow(missing_docs)]
    Friday = 5,
    #[allow(missing_docs)]
    Saturday = 6,
    #[allow(missing_docs)]
    Sunday = 7,
}

/// A date in the proleptic Gregorian calendar.
///
/// Years in the range `-999_999_999..=999_999_999` are supported.
pub struct Date {
    /// The ordinal day within the year.
    day: u16,
    /// The year.
    // This is to support years up to 999,999,999.
    year: i32,
}

impl Date {
    /// Returns a [Date] from the calendar date.
    ///
    /// `year` is the Gregorian ordinal year.
    ///
    /// `month` is a number from 1 to 12.
    ///
    /// `day` is a number from 1 to 31.
    ///
    /// # Errors
    ///
    /// An error is returned if the date does not exist, e.g. April 31st of a given year.
    // TODO: which error variant?
    pub fn from_ymd(_year: i32, _month: u32, _day: u32) -> Result<Self, Error> {
        todo!()
    }

    /// Returns a [Date] from the ISO week date.
    ///
    /// `year` is the Gregorian ordinal year.
    ///
    /// `week` is the ISO week number.
    ///
    /// `day` is the [Weekday].
    ///
    /// # Errors
    ///
    /// An error is returned if the date does not exist or is out of range.
    // TODO: which variant?
    pub fn from_iso_ywd(_year: i32, _week: u32, _day: Weekday) -> Result<Self, Error> {
        todo!()
    }

    /// Returns a [Date] from the current system time.
    #[cfg(feature = "std")]
    pub fn today() -> Self {
        todo!()
    }

    fn md(&self) -> (u8, u8) {
        const DAYS_BEFORE: [u16; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        const DAYS_BEFORE_LEAP: [u16; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];

        if self.is_leap_year() {
            let m = DAYS_BEFORE_LEAP.partition_point(|&d| d < self.day);
            let d = self.day - DAYS_BEFORE_LEAP[m - 1];
            (m as u8, d as u8)
        } else {
            let m = DAYS_BEFORE.partition_point(|&d| d < self.day);
            let d = self.day - DAYS_BEFORE[m - 1];
            (m as u8, d as u8)
        }
    }

    /// Returns the ordinal day within the year.
    pub fn day_ord(&self) -> u16 {
        self.day
    }

    /// The day within its month.
    pub fn day(&self) -> u8 {
        self.md().1
    }

    /// The month.
    pub fn month(&self) -> u8 {
        self.md().0
    }

    /// The numerical year.
    pub fn year(&self) -> i32 {
        self.year
    }

    /// Whether this represents a leap year.
    pub fn is_leap_year(&self) -> bool {
        self.year % 4 == 0 && (self.year % 100 != 0 || self.year % 400 == 0)
    }
}

#[cfg(feature = "std")]
impl From<SystemTime> for Date {
    fn from(_val: SystemTime) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_leap_years() {
        assert!(Date { year: -800, day: 0 }.is_leap_year());
        assert!(Date { year: 0, day: 0 }.is_leap_year());
        assert!(Date { year: 1200, day: 0 }.is_leap_year());
        assert!(Date { year: 2000, day: 0 }.is_leap_year());
        assert!(Date { year: 2004, day: 0 }.is_leap_year());
        assert!(!Date { year: -100, day: 0 }.is_leap_year());
        assert!(!Date { year: 1700, day: 0 }.is_leap_year());
        assert!(!Date { year: 1900, day: 0 }.is_leap_year());
        assert!(!Date { year: 2003, day: 0 }.is_leap_year());
        assert!(!Date { year: 2023, day: 0 }.is_leap_year());
    }

    #[test]
    fn date_day() {
        assert_eq!(Date { year: 0, day: 1 }.day(), 1);
        assert_eq!(Date { year: 0, day: 31 }.day(), 31);
        assert_eq!(Date { year: 0, day: 32 }.day(), 1);
        assert_eq!(Date { year: 3, day: 60 }.day(), 1);
        assert_eq!(Date { year: 3, day: 365 }.day(), 31);
        assert_eq!(Date { year: 4, day: 32 }.day(), 1);
        assert_eq!(Date { year: 4, day: 60 }.day(), 29);
        assert_eq!(Date { year: 4, day: 61 }.day(), 1);
        assert_eq!(Date { year: 4, day: 366 }.day(), 31);
    }

    #[test]
    fn date_month() {
        assert_eq!(Date { year: 0, day: 1 }.month(), 1);
        assert_eq!(Date { year: 0, day: 31 }.month(), 1);
        assert_eq!(Date { year: 0, day: 32 }.month(), 2);
        assert_eq!(Date { year: 3, day: 60 }.month(), 3);
        assert_eq!(Date { year: 3, day: 365 }.month(), 12);
        assert_eq!(Date { year: 4, day: 32 }.month(), 2);
        assert_eq!(Date { year: 4, day: 60 }.month(), 2);
        assert_eq!(Date { year: 4, day: 61 }.month(), 3);
        assert_eq!(Date { year: 4, day: 366 }.month(), 12);
    }
}
