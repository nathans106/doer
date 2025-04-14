use chrono::NaiveDateTime;

pub struct Day;

impl Day {
    pub fn priorities(&self) -> Vec<Priority> {
        todo!()
    }

    pub fn set_priorities(&mut self, _priorities: Vec<Priority>) {
        todo!()
    }

    pub fn focus(&self) -> Option<Focus> {
        todo!()
    }

    pub fn set_focus(&mut self, _focus: Focus) {
        todo!()
    }

    pub fn set_focus_to_str(&mut self, _focus: String) {
        todo!()
    }

    pub fn log(&self) -> &[Focus] {
        todo!()
    }

    pub fn end(&mut self) {
        todo!()
    }

    pub fn notes(&self) -> &[String] {
        todo!()
    }
}

pub struct Priority {
    pub done: bool,
}

impl Priority {
    pub fn name(&self) -> String {
        todo!()
    }

    pub fn is_done(&self) -> bool {
        todo!()
    }
}

pub struct Focus;

impl Focus {
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

pub struct Break;

impl Break {
    pub fn start(&self) -> NaiveDateTime {
        todo!()
    }

    pub fn end(&self) -> NaiveDateTime {
        todo!()
    }
}
