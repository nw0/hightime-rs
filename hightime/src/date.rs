#[cfg(feature = "std")]
use std::time::SystemTime;

use crate::Error;

pub type IsoWeekday = u8;

impl From<Weekday> for IsoWeekday {
    fn from(val: Weekday) -> Self {
        match val {
            Weekday::Monday => 1,
            Weekday::Tuesday => 2,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 4,
            Weekday::Friday => 5,
            Weekday::Saturday => 6,
            Weekday::Sunday => 7,
        }
    }
}

impl TryFrom<IsoWeekday> for Weekday {
    type Error = Error;
    fn try_from(val: IsoWeekday) -> Result<Self, Self::Error> {
        match val {
            1 => Ok(Self::Monday),
            2 => Ok(Self::Tuesday),
            3 => Ok(Self::Wednesday),
            4 => Ok(Self::Thursday),
            5 => Ok(Self::Friday),
            6 => Ok(Self::Saturday),
            7 => Ok(Self::Sunday),
            _ => Err(Error::RangeExceeded),
        }
    }
}

/// A day of the week.
#[derive(Debug, Eq, PartialEq)]
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
    Sunday = 0,
}

/// A date in the proleptic Gregorian calendar.
///
/// Years in the range `-999_999_999..=999_999_999` are supported.
#[derive(Debug, Eq, PartialEq)]
pub struct Date {
    /// The ordinal day within the year.
    day: u16,
    /// The year.
    // This is to support years up to 999,999,999.
    year: i32,
}

impl Date {
    const MONTH_END: [u16; 13] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];
    const MONTH_END_LEAP: [u16; 13] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366];

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
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Result<Self, Error> {
        if year < -999_999_999 || 999_999_999 < year {
            return Err(Error::UnsupportedYear);
        }
        if month < 1 || 12 < month || day < 1 {
            return Err(Error::RangeExceeded);
        }
        let month_ends = Self::month_ends(year);
        let day = month_ends[month as usize - 1] + day as u16;
        if day > month_ends[month as usize] {
            return Err(Error::RangeExceeded);
        }
        Ok(Self { year, day })
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

    /// Returns the weekday.
    pub fn weekday(&self) -> Weekday {
        let y = self.year - 1;
        let d = self.day as i32;
        (match (d + 5 * (y % 4) + 4 * (y % 100) + 6 * (y % 400)) % 7 {
            0 => 7,
            i => i,
        } as IsoWeekday)
            .try_into()
            .expect("can't be out of range")
    }

    fn md(&self) -> (u8, u8) {
        let month_ends = Self::month_ends(self.year);
        let m = month_ends.partition_point(|&d| d < self.day);
        let d = self.day - month_ends[m - 1];
        (m as u8, d as u8)
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
        Self::is_leap(self.year)
    }

    fn is_leap(year: i32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }

    fn month_ends(year: i32) -> &'static [u16; 13] {
        if Self::is_leap(year) {
            &Self::MONTH_END_LEAP
        } else {
            &Self::MONTH_END
        }
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

    #[test]
    fn date_ymd_range() {
        assert_eq!(Date::from_ymd(2004, 0, 1), Err(Error::RangeExceeded));
        assert_eq!(Date::from_ymd(2004, 13, 1), Err(Error::RangeExceeded));
        assert_eq!(Date::from_ymd(2004, 1, 0), Err(Error::RangeExceeded));
        assert_eq!(Date::from_ymd(2004, 1, 32), Err(Error::RangeExceeded));
        assert_eq!(Date::from_ymd(2003, 2, 29), Err(Error::RangeExceeded));
        assert!(Date::from_ymd(2004, 2, 29).is_ok());
    }

    #[test]
    fn date_ymd() {
        assert_eq!(Date::from_ymd(2004, 2, 29).unwrap().day_ord(), 60);
        assert_eq!(Date::from_ymd(2004, 3, 1).unwrap().day_ord(), 61);
        assert_eq!(Date::from_ymd(2003, 3, 1).unwrap().day_ord(), 60);
        assert_eq!(Date::from_ymd(2002, 12, 31).unwrap().day_ord(), 365);
        assert_eq!(Date::from_ymd(2004, 12, 31).unwrap().day_ord(), 366);
    }

    #[test]
    fn date_weekday() {
        assert_eq!(
            Date::from_ymd(2023, 3, 1).unwrap().weekday(),
            Weekday::Wednesday
        );
        assert_eq!(
            Date::from_ymd(1200, 4, 28).unwrap().weekday(),
            Weekday::Friday
        );
    }
}
