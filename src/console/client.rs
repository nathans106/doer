use chrono::NaiveDate;

pub struct Client;

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
        todo!();
    }

    pub fn contexts(&self) {
        todo!();
    }

    pub fn set_context(&self, _context: &str) {
        todo!();
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
