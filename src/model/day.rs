use anyhow::anyhow;
use chrono::{NaiveDateTime, NaiveTime, Utc};

pub struct Day {
    priorities: Vec<Priority>,
    log: Vec<Focus>,
    end_time: NaiveTime,
    notes: Vec<String>,
}

impl Day {
    pub fn priorities(&self) -> &[Priority] {
        self.priorities.as_slice()
    }

    pub fn set_priorities(&mut self, priorities: Vec<Priority>) {
        self.priorities = priorities;
    }

    pub fn focus(&self) -> Option<&Focus> {
        self.log.last()
    }

    pub fn set_focus_to_str(&mut self, name: String) {
        match self.log.last() {
            None => self.log.push(Focus::new(name, Utc::now().time())),
            Some(last_focus) => {
                if last_focus.name() != name {
                    self.log.push(Focus::new(name, Utc::now().time()))
                }
            }
        }
    }

    pub fn start_break(&mut self) -> anyhow::Result<()> {
        match self.log.last_mut() {
            None => Err(anyhow!("No focus; cannot start break")),
            Some(focus) => {
                focus.start_break();
                Ok(())
            }
        }
    }

    pub fn end_break(&mut self) -> anyhow::Result<()> {
        match self.log.last_mut() {
            None => Err(anyhow!("No focus; cannot end break")),
            Some(focus) => {
                focus.end_break();
                Ok(())
            }
        }
    }

    pub fn log(&self) -> &[Focus] {
        self.log.as_slice()
    }

    pub fn end(&mut self) {
        self.end_time = Utc::now().time();
    }

    pub fn notes(&self) -> &[String] {
        self.notes.as_slice()
    }

    pub fn push_note(&mut self, note: String) {
        self.notes.push(note);
    }
}

#[derive(Clone)]
pub struct Priority {
    pub done: bool,
}

impl Priority {
    pub fn new(_name: String) -> Self {
        todo!()
    }

    pub fn name(&self) -> String {
        todo!()
    }

    pub fn is_done(&self) -> bool {
        todo!()
    }
}

pub struct Focus;

impl Focus {
    pub fn new(_name: String, _time: NaiveTime) -> Self {
        todo!()
    }

    pub fn name(&self) -> String {
        todo!()
    }

    pub fn start(&self) -> NaiveDateTime {
        todo!()
    }

    pub fn breaks(&self) -> &[Break] {
        todo!()
    }

    pub fn start_break(&mut self) {
        todo!()
    }

    pub fn end_break(&mut self) {
        todo!()
    }
}

pub struct Break {
    pub start_time: NaiveTime,
    pub end_time: Option<NaiveTime>,
}

impl Break {
    pub fn new(start_time: NaiveTime) -> Self {
        Self {
            start_time,
            end_time: None,
        }
    }

    pub fn end(&mut self) {
        self.end_time = Some(Utc::now().naive_local().time());
    }
}
