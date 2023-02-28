use crate::{Date, Error};

/// Clock time within a date, without a time zone.
pub struct NaiveTime {}

impl NaiveTime {
    /// Returns a [NaiveTime] from the hour, minute, and second.
    ///
    /// # Errors
    ///
    /// Returns an error when the values are out of range.
    // TODO: which variant?
    pub fn from_hms(_hour: u8, _minute: u8, _second: u8) -> Result<Self, Error> {
        todo!()
    }

    /// Returns a [NaiveTime] from the hour, minute, second, and nanosecond.
    ///
    /// # Errors
    ///
    /// Returns an error when the values are out of range.
    // TODO: which variant?
    pub fn from_hms_ns(
        _hour: u8,
        _minute: u8,
        _second: u8,
        _nanosecond: u32,
    ) -> Result<Self, Error> {
        todo!()
    }
}

/// An offset, which may be implied by a time zone.
pub trait Offset {}

/// A fixed offset, specified in hours, minutes, and seconds.
pub struct FixedOffset {}

impl Offset for FixedOffset {}

/// A date and time with an offset for the time zone.
pub struct DateTime<O: Offset> {
    _marker: core::marker::PhantomData<O>,
}

impl<O: Offset> DateTime<O> {
    /// Returns a [DateTime] from a [Date], a [NaiveTime], and an offset.
    ///
    /// # Errors
    ///
    /// This should not result in an error as yet, but may cause an error
    /// once time zone information has been implemented, if the range is invalid.
    pub fn new(_date: Date, _time: NaiveTime) -> Result<Self, Error> {
        todo!()
    }

    /// Returns a [DateTime] from the system clock and time zone.
    #[cfg(feature = "std")]
    pub fn now() -> Result<Self, Error> {
        todo!()
    }
}
