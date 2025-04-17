use crate::model::day::Day;
use chrono::NaiveDate;
use std::collections::HashMap;

pub type Year = HashMap<NaiveDate, Day>;

pub struct YearFile {
    version: String,
    days: Year,
}

pub fn year_to_json(_year: &Year) -> String {
    todo!()
}
