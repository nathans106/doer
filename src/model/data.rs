use crate::model::day::Day;
use chrono::NaiveDate;
use std::path::PathBuf;

pub struct Data;

impl Data {
    pub fn new(_database: PathBuf, _context: String) -> Self {
        todo!()
    }

    pub fn last_date(&self) -> NaiveDate {
        todo!()
    }

    pub fn day(&self, _date: &NaiveDate) -> Day {
        todo!()
    }
}
