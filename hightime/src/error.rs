/// An error returned by `hightime`.
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// The year is out of range.
    ///
    /// Years in the range `-999_999_999..=999_999_999` are supported.
    UnsupportedYear,
    /// Tried to create a date which is invalid because it is out of range.
    RangeExceeded,
    /// The system time is invalid (it likely is before the Unix epoch)
    #[cfg(feature = "std")]
    InvalidSystemTime,
}
