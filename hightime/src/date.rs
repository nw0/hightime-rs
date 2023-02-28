use crate::Error;

pub struct NaiveDate {}

impl NaiveDate {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, Error> {
        todo!()
    }

    pub fn today() -> Self {
        todo!()
    }
}
