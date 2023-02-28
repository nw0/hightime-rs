use crate::Error;

pub struct NaiveDate {}

impl NaiveDate {
    pub fn from_ymd(_year: i32, _month: i32, _day: i32) -> Result<Self, Error> {
        todo!()
    }

    #[cfg(feature = "std")]
    pub fn today() -> Self {
        todo!()
    }
}
