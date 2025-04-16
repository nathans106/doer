use crate::config::Config;
use crate::model::data::Data;
use crate::model::day::Priority;
use crate::model::day_editor::DayEditor;
use crate::storage::Storage;
use chrono::{NaiveDate, Utc};
use inquire::Text;

type Id = u8;

pub struct Client {
    config: Config,
    data: Data,
}

impl Client {
    pub fn new() -> anyhow::Result<Self> {
        let storage = Storage::new()?;
        let config = Config::new(storage.clone());
        let data = Data::new(storage.database(), config.context()?);
        Ok(Self { config, data })
    }

    pub fn plan_priorities(&self, date: &NaiveDate) -> anyhow::Result<()> {
        {
            let mut edit_guard = self.edit_day(*date);
            let day = edit_guard.day();
            let mut items: Vec<Priority> = vec![];

            let current_priorities = day.priorities();
            if !current_priorities.is_empty() {
                self.show_priorities(date);
                items.extend(current_priorities.iter().cloned());
            }

            items.extend(collect_items(date)?);
            day.set_priorities(order_items(items));
        }

        self.show_priorities(date);
        Ok(())
    }

    pub fn copy_priorities(&self, from: &NaiveDate, to: &NaiveDate) {
        {
            let mut edit_guard = self.edit_day(*to);
            let day_to = edit_guard.day();
            let day_from = self.data.day(from);
            day_to.set_priorities(day_from.priorities().to_vec());
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

        let mut priorities: Vec<Priority> = day.priorities().to_vec();
        priorities[*id as usize].done = is_done;
        day.set_priorities(priorities);
    }

    pub fn context(&self) -> anyhow::Result<()> {
        println!("{}", self.config.context()?);
        Ok(())
    }

    pub fn contexts(&self) -> anyhow::Result<()> {
        for context in self.config.contexts()? {
            println!("{}", context);
        }
        Ok(())
    }

    pub fn set_context(&self, context: String) -> anyhow::Result<()> {
        self.config.set_context(context)?;
        Ok(())
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

    pub fn start_break(&self) -> anyhow::Result<()> {
        let mut edit_guard = self.edit_today();
        let day = edit_guard.day();
        day.start_break()?;
        Ok(())
    }

    pub fn end_break(&self) -> anyhow::Result<()> {
        let mut edit_guard = self.edit_today();
        let day = edit_guard.day();
        day.end_break()?;
        Ok(())
    }

    pub fn end_day(&self) {
        let mut edit_guard = self.edit_today();
        let day = edit_guard.day();
        day.end();
    }

    pub fn note(&self, note: String) {
        let mut edit_guard = self.edit_today();
        let day = edit_guard.day();
        day.push_note(note);
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
                let break_start = break_.start_time.format("%H:%M");
                let mut line = format!("\t{break_start}");
                match break_.end_time {
                    None => {}
                    Some(end_time) => {
                        line += &format!(" - {end_time}");
                    }
                }
                println!("{}", line);
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

    fn edit_day(&self, date: NaiveDate) -> DayEditor {
        DayEditor::at(date, &self.data)
    }

    fn edit_today(&self) -> DayEditor {
        let today = Utc::now().naive_local().date();
        DayEditor::at(today, &self.data)
    }
}

fn collect_items(date: &NaiveDate) -> anyhow::Result<Vec<Priority>> {
    let mut items = vec![];
    let mut line = Text::new(format!("List items for {date}\n").as_str()).prompt()?;

    while !line.is_empty() {
        items.push(Priority::new(line));
        line = Text::new("Anymore?\n").prompt()?;
    }

    Ok(items)
}

fn order_items(items: Vec<Priority>) -> Vec<Priority> {
    let mut result = vec![];
    let mut remaining = items.clone();

    for _i in 0..6 {
        let max_id = remaining.len() as Id;
        if max_id == 0 {
            return result;
        }

        println!("Remaining:");
        for (j, item) in remaining.iter().enumerate() {
            let id = j + 1;
            let item_name = item.name();
            println!("{id}. {item_name}");
        }

        match get_item_id(&max_id) {
            None => {
                return result;
            }
            Some(choice) => {
                let choice_id = choice - 1;
                let priority = remaining.remove(choice_id as usize);
                result.push(priority);
            }
        }
    }

    result
}

fn get_item_id(max_id: &Id) -> Option<Id> {
    loop {
        match Text::new("Select the top priority, or press enter to end.\n").prompt() {
            Ok(choice) => match choice.parse::<Id>() {
                Ok(id) => {
                    if &id > max_id {
                        println!("{id} is too large, maximum is {max_id}");
                    }

                    return Some(id);
                }
                Err(_) => {
                    println!("{choice} is not a digit, please try again");
                }
            },
            Err(_) => {
                return None;
            }
        }
    }
}
