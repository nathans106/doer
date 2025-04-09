use crate::model::data::Data;
use crate::model::day::Day;
use chrono::NaiveDate;

pub struct DayEditor;

impl DayEditor {
    pub fn at(date: &NaiveDate, data: &Data) -> Self {
        todo!()
    }

    pub fn day(&mut self) -> &mut Day {
        todo!()
    }
}

impl Drop for DayEditor {
    fn drop(&mut self) {
        todo!()
    }
}
