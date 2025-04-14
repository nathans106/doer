use crate::config::Config;
use crate::model::data::Data;
use crate::model::day::Priority;
use crate::model::day_editor::DayEditor;
use chrono::{NaiveDate, Utc};

type Id = u8;

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

            items.extend(collect_items(date));
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

    pub fn show(&self, date: &NaiveDate) {
        self.show_priorities(date);
        self.show_log(date);
        self.show_notes(date);
    }

    pub fn tick(&mut self, id: &Id) {
        self.set_tick(id, true)
    }

    pub fn un_tick(&mut self, id: &Id) {
        self.set_tick(id, false)
    }

    fn set_tick(&mut self, id: &Id, is_done: bool) {
        let mut edit_guard = self.edit_today();
        let day = edit_guard.day();
        let max_id = day.priorities().len() - 1;
        if id > &(max_id as u8) {
            println!("id {id} invalid, maximum is {max_id}");
            return;
        }

        let mut priorities = day.priorities();
        priorities[*id as usize].done = is_done;
        day.set_priorities(priorities);
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

    pub fn set_focus_to_priority(&self, id: &Id) {
        let mut edit_guard = self.edit_today();
        let day = edit_guard.day();
        let max_id = day.priorities().len();
        if *id as usize > max_id {
            println!("id {id} invalid, maximum is {max_id}");
            return;
        }

        day.set_focus_to_str(day.priorities()[(id - 1) as usize].name())
    }

    pub fn set_focus(&self, focus: &str) {
        let mut edit_guard = self.edit_today();
        let day = edit_guard.day();
        day.set_focus_to_str(focus.to_string())
    }

    pub fn start_break(&self) {
        let mut edit_guard = self.edit_today();
        let day = edit_guard.day();
        match day.focus() {
            None => {
                println!("No focus; cannot start break");
            }
            Some(mut focus) => {
                focus.start_break();
                day.set_focus(focus);
            }
        }
    }

    pub fn end_break(&self) {
        let mut edit_guard = self.edit_today();
        let day = edit_guard.day();
        match day.focus() {
            None => {
                println!("No focus; cannot end break");
            }
            Some(mut focus) => {
                focus.end_break();
                day.set_focus(focus);
            }
        }
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
        let day = self.data.day(date);

        if day.priorities().is_empty() {
            println!("{date} has no priorities");
            return;
        }

        let mut focus_name = match &day.focus() {
            None => String::new(),
            Some(focus) => focus.name(),
        };

        println!("Priorities for {date} are:");
        for (i, priority) in day.priorities().iter().enumerate() {
            let id = i + 1;
            let priority_name = priority.name();

            let mut line = format!("{id}. {priority_name}");

            if priority_name == focus_name {
                line += "*";
                focus_name = String::new();
            }

            if priority.is_done() {
                line += " - done";
            }

            println!("{line}");
        }

        if !focus_name.is_empty() {
            println!("\nFocus: {focus_name}");
        }

        println!("\n");
    }

    fn show_log(&self, date: &NaiveDate) {
        let day = self.data.day(date);

        if day.log().is_empty() {
            println!("{date} has no log");
            return;
        }

        println!("Log for {date} is:");

        for focus in day.log() {
            let name = focus.name();
            let start = focus.start().format("%H:%M");
            println!("{start} - {name}");

            for break_ in focus.breaks() {
                let break_start = break_.start().format("%H:%M");
                let break_end = break_.end().format("%H:%M");
                println!("\t{break_start} - {break_end}");
            }
        }

        println!("\n");
    }

    fn show_notes(&self, date: &NaiveDate) {
        let day = self.data.day(date);

        if day.notes().is_empty() {
            println!("{date} has no notes");
            return;
        }

        println!("Notes for {date}:");

        for (i, note) in day.notes().iter().enumerate() {
            let id = i + 1;
            println!("{id}. {note}");
        }

        println!("\n");
    }

    fn edit_day(&self, date: &NaiveDate) -> DayEditor {
        DayEditor::at(date, &self.data)
    }

    fn edit_today(&self) -> DayEditor {
        let today = Utc::now().naive_local().date();
        DayEditor::at(&today, &self.data)
    }
}

impl Default for Client {
    fn default() -> Self {
        todo!()
    }
}

fn collect_items(_date: &NaiveDate) -> Vec<Priority> {
    todo!()
}

fn order_items(_items: Vec<Priority>) -> Vec<Priority> {
    todo!()
}
