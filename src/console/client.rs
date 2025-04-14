use crate::config::Config;
use crate::model::data::Data;
use crate::model::day::Priority;
use crate::model::day_editor::DayEditor;
use chrono::NaiveDate;

pub struct Client {
    data: Data,
}

impl Client {
    pub fn plan_priorities(&self, date: &NaiveDate) {
        {
            let mut edit_guard = self.edit_day(date);
            let day = edit_guard.day();
            let mut items = vec![];

            let current_priorities = day.priorities();
            if !current_priorities.is_empty() {
                self.show_priorities(date);
                items.extend(current_priorities);
            }

            items.extend(collect_items(date).into_iter());
            day.set_priorities(order_items(items));
        }

        self.show_priorities(date);
    }

    pub fn copy_priorities(&self, from: &NaiveDate, to: &NaiveDate) {
        {
            let mut edit_guard = self.edit_day(to);
            let day_to = edit_guard.day();
            let day_from = self.data.day(from);
            day_to.set_priorities(day_from.priorities());
        }
        self.show_priorities(to);
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
        self.data.last_date()
    }

    fn show_priorities(&self, date: &NaiveDate) {
        todo!()
    }

    fn edit_day(&self, date: &NaiveDate) -> DayEditor {
        DayEditor::at(date, &self.data)
    }
}

impl Default for Client {
    fn default() -> Self {
        todo!()
    }
}

fn collect_items(date: &NaiveDate) -> Vec<Priority> {
    todo!()
}

fn order_items(items: Vec<Priority>) -> Vec<Priority> {
    todo!()
}
