use crate::Error;

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
    /// An error is returned if the date does not exists, e.g. April 31st of a given year.
    // TODO: which error variant?
    pub fn from_ymd(_year: i32, _month: u32, _day: u32) -> Result<Self, Error> {
        todo!()
    }

    /// Returns a [NaiveDate] from the current system time.
    #[cfg(feature = "std")]
    pub fn today() -> Self {
        todo!()
    }
}
