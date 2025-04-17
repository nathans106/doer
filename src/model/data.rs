use crate::model::day::Day;
use crate::model::year::{Year, year_to_json};
use anyhow::anyhow;
use chrono::{Datelike, NaiveDate, Utc};
use itertools::Itertools;
use std::fs;
use std::path::PathBuf;

type YearNum = i32;

pub struct Data {
    root: PathBuf,
    context: String,
}

impl Data {
    pub fn new(database: PathBuf, context: String) -> anyhow::Result<Self> {
        fs::create_dir_all(database.clone())?;
        Ok(Self {
            root: database,
            context,
        })
    }

    fn get_year(&self, year_num: &YearNum) -> anyhow::Result<Year> {
        let year_file = self.year_file(year_num);
        if year_file.exists() {
            let year = serde_json::from_str(&fs::read_to_string(year_file)?)?;
            Ok(year)
        } else {
            Ok(Year::new())
        }
    }

    fn year_file(&self, year_num: &YearNum) -> PathBuf {
        self.root
            .join(self.context.clone())
            .join(format!("{year_num}.json"))
    }

    pub fn last_date(&self) -> anyhow::Result<NaiveDate> {
        let year = self.get_year(&self.last_year()?)?;
        let today = Utc::now().naive_utc().date();

        for date in year.keys().sorted().rev() {
            if date <= &today {
                return Ok(*date);
            }
        }

        Err(anyhow!("No dates saved in year"))
    }

    pub fn last_year(&self) -> anyhow::Result<YearNum> {
        let current_year = Utc::now().naive_local().year();
        let years =
            fs::read_dir(self.root.clone())?.filter_map(|entry_result| match entry_result {
                Ok(entry) => {
                    let file_name_os = entry.file_name();
                    let file_name = file_name_os.to_str();
                    if file_name?.ends_with(".json") {
                        return match entry.path().file_stem()?.to_str()?.parse::<YearNum>() {
                            Ok(year_num) => Some(year_num),
                            Err(_) => None,
                        };
                    }

                    None
                }
                Err(_) => None,
            });

        for year_num in years {
            if year_num <= current_year {
                return Ok(year_num);
            }
        }

        Err(anyhow!("No Year data found"))
    }

    pub fn day(&self, date: &NaiveDate) -> anyhow::Result<Day> {
        let year = self.get_year(&date.year())?;
        if year.contains_key(date) {
            Ok(year[date].clone())
        } else {
            Ok(Day::default())
        }
    }

    pub fn set_day(&self, date: NaiveDate, day: Day) -> anyhow::Result<()> {
        let mut year = self.get_year(&date.year())?;
        year.insert(date, day);
        self.set_year(date.year(), year)?;
        Ok(())
    }

    fn set_year(&self, year_num: YearNum, year: Year) -> anyhow::Result<()> {
        let year_file = self.year_file(&year_num);
        let year_map = year_to_json(&year);
        fs::write(year_file, year_map)?;
        Ok(())
    }
}
