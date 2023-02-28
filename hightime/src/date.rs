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

/// A date without time zone information.
pub struct NaiveDate {}

impl NaiveDate {
    /// Returns a [NaiveDate] from the calendar date.
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

    /// Returns a [NaiveDate] from the ISO week date.
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

    /// Returns a [NaiveDate] from the current system time.
    #[cfg(feature = "std")]
    pub fn today() -> Self {
        todo!()
    }
}

#[cfg(feature = "std")]
impl From<SystemTime> for NaiveDate {
    fn from(_val: SystemTime) -> Self {
        todo!()
    }
}
