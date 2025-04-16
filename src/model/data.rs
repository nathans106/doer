use crate::model::day::Day;
use chrono::NaiveDate;
use std::path::Path;

pub struct Data;

impl Data {
    pub fn new(_database: &Path, _context: String) -> Self {
        todo!()
    }

    pub fn last_date(&self) -> NaiveDate {
        todo!()
    }

    pub fn day(&self, _date: &NaiveDate) -> Day {
        todo!()
    }

    pub fn set_day(&self, _date: &NaiveDate, _day: &Day) {
        todo!()
    }
}
