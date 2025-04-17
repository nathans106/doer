use crate::model::day::{Break, Day, Focus, Priority};
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Year = HashMap<NaiveDate, Day>;

#[derive(Deserialize, Serialize)]
struct PriorityJson {
    pub name: String,
    pub done: bool,
}

#[derive(Deserialize, Serialize)]
struct BreakJson {
    pub start: NaiveTime,
    pub end: Option<NaiveTime>,
}

#[derive(Deserialize, Serialize)]
struct LogJson {
    pub name: String,
    pub start: NaiveTime,
    pub breaks: Vec<BreakJson>,
}

#[derive(Deserialize, Serialize)]
struct DayJson {
    pub priorities: Vec<PriorityJson>,
    pub log: Vec<LogJson>,
    pub notes: Vec<String>,
    pub end_time: Option<NaiveTime>,
}

impl From<&Day> for DayJson {
    fn from(value: &Day) -> Self {
        Self {
            priorities: value
                .priorities()
                .iter()
                .map(|priority| PriorityJson {
                    name: priority.name.clone(),
                    done: priority.is_done,
                })
                .collect(),
            log: value
                .log()
                .iter()
                .map(|log| LogJson {
                    name: log.name().clone(),
                    start: *log.start(),
                    breaks: log
                        .breaks()
                        .iter()
                        .map(|break_| BreakJson {
                            start: break_.start_time,
                            end: break_.end_time,
                        })
                        .collect(),
                })
                .collect(),
            notes: value.notes().to_vec(),
            end_time: *value.end_time(),
        }
    }
}

impl Into<Day> for DayJson {
    fn into(self) -> Day {
        Day::new(
            self.priorities
                .into_iter()
                .map(|priority_json| Priority {
                    name: priority_json.name,
                    is_done: priority_json.done,
                })
                .collect(),
            self.log
                .into_iter()
                .map(|log_json| {
                    Focus::full(
                        log_json.name,
                        log_json.start,
                        log_json
                            .breaks
                            .into_iter()
                            .map(|break_json| Break {
                                start_time: break_json.start,
                                end_time: break_json.end,
                            })
                            .collect(),
                    )
                })
                .collect(),
            self.end_time,
            self.notes,
        )
    }
}

#[derive(Deserialize, Serialize)]
struct YearJson {
    version: String,
    days: HashMap<NaiveDate, DayJson>,
}

pub fn year_to_json(year: &Year) -> anyhow::Result<String> {
    let json = YearJson {
        version: env!("CARGO_PKG_VERSION").to_string(),
        days: year
            .iter()
            .map(|(date, day)| (*date, DayJson::from(day)))
            .collect(),
    };
    let json_str = serde_json::to_string(&json)?;
    Ok(json_str)
}

pub fn year_from_json(json_str: &str) -> anyhow::Result<Year> {
    let json: YearJson = serde_json::from_str(json_str)?;
    let year = json
        .days
        .into_iter()
        .map(|(date, day_json)| {
            let day: Day = day_json.into();
            (date, day)
        })
        .collect();
    Ok(year)
}
