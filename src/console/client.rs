use crate::config::Config;
use crate::model::data::Data;
use crate::model::day_editor::DayEditor;
use chrono::NaiveDate;

pub struct Client {
    data: Data,
    day_editor: DayEditor,
}

impl Client {
    pub fn plan_priorities(&self, _date: &NaiveDate) {
        todo!();
    }

    pub fn copy_priorities(&self, _from: &NaiveDate, _date: &NaiveDate) {
        todo!();
    }

    pub fn show(&self, _date: &NaiveDate) {
        todo!();
    }

    pub fn tick(&self, _id: &u8) {
        todo!();
    }

    pub fn un_tick(&self, _id: &u8) {
        todo!();
    }

    pub fn context(&self) {
        println!("{}", Config::context())
    }

    pub fn contexts(&self) {
        for context in Config::contexts() {
            println!("{}", context);
        }
    }

    pub fn set_context(&self, context: &str) {
        Config::set_context(context);
    }

    pub fn set_focus_to_priority(&self, _id: &u8) {
        todo!();
    }

    pub fn set_focus(&self, _focus: &str) {
        todo!();
    }

    pub fn start_break(&self) {
        todo!();
    }

    pub fn end_break(&self) {
        todo!();
    }

    pub fn end_day(&self) {
        todo!();
    }

    pub fn note(&self, _note: &str) {
        todo!();
    }

    pub fn last_date(&self) -> NaiveDate {
        todo!();
    }
}

impl Default for Client {
    fn default() -> Self {
        todo!()
    }
}
