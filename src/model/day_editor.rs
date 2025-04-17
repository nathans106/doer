use crate::model::data::Data;
use crate::model::day::Day;
use chrono::NaiveDate;

pub struct DayEditor<'data> {
    date: NaiveDate,
    data: &'data Data,
    day: Day,
}

impl<'data> DayEditor<'data> {
    pub fn at(date: NaiveDate, data: &'data Data) -> anyhow::Result<Self> {
        Ok(Self {
            date,
            data,
            day: data.day(&date)?,
        })
    }

    pub fn day(&mut self) -> &mut Day {
        &mut self.day
    }
}

impl Drop for DayEditor<'_> {
    fn drop(&mut self) {
        self.data
            .set_day(self.date, self.day.clone())
            .expect("Failed to set day");
    }
}
