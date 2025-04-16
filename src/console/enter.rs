use crate::console::client::Client;
use chrono::{Datelike, Local, NaiveDate};
use clap::{Parser, Subcommand};
use std::str::FromStr;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Plan {
        /// Date to plan
        date: String,
    },
    Copy {
        /// Date to copy from. Can be 'today', 'tomorrow', or a date in 'ISO format'
        from: String,
        /// Date to copy to. Can be 'today', 'tomorrow', or a date in 'ISO format'
        to: String,
    },
    Show {
        /// Date to show
        date: String,
    },
    ShowLast,
    Tick {
        id: u8,
    },
    UnTick {
        id: u8,
    },
    Context,
    Contexts,
    SetContext {
        /// Context to switch to
        context: String,
    },
    SetFocus {
        /// name or ID of focus
        focus: String,
    },
    StartBreak,
    EndBreak,
    EndDay,
    Note {
        /// Note for today
        note: String,
    },
}

fn weekday(id: &i64) -> NaiveDate {
    let now = Local::now().naive_local().date();
    let diff = id - (now.weekday().num_days_from_monday() as i64);
    now + chrono::TimeDelta::days(diff)
}

fn date_from_arg(date: &str) -> anyhow::Result<NaiveDate> {
    match date {
        // Shorthand for nearby days
        "yesterday" => Ok(Local::now().naive_local().date() - chrono::Duration::days(1)),
        "today" => Ok(Local::now().naive_local().date()),
        "tomorrow" => Ok(Local::now().naive_local().date() + chrono::Duration::days(1)),
        // Days of the current week
        "monday" => Ok(weekday(&0)),
        "tuesday" => Ok(weekday(&1)),
        "wednesday" => Ok(weekday(&2)),
        "thursday" => Ok(weekday(&3)),
        "friday" => Ok(weekday(&4)),
        "saturday" => Ok(weekday(&5)),
        "sunday" => Ok(weekday(&6)),
        _ => {
            let result = NaiveDate::from_str(date)?;
            Ok(result)
        }
    }
}

pub fn enter() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut client = Client::new()?;

    match cli.command {
        Command::Plan { date } => {
            client.plan_priorities(&date_from_arg(&date)?)?;
        }
        Command::Copy { from, to } => {
            client.copy_priorities(&date_from_arg(&from)?, &date_from_arg(&to)?);
        }
        Command::Show { date } => {
            client.show(&date_from_arg(&date)?);
        }
        Command::ShowLast => {
            let date = client.last_date();
            client.show(&date);
        }
        Command::Tick { id } => {
            client.tick(&(id - 1));
        }
        Command::UnTick { id } => {
            client.un_tick(&(id - 1));
        }
        Command::Context => {
            client.context()?;
        }
        Command::Contexts => {
            client.contexts()?;
        }
        Command::SetContext { context } => {
            client.set_context(context)?;
        }
        Command::SetFocus { focus } => match focus.parse::<u8>() {
            Ok(id) => {
                client.set_focus_to_priority(&(id));
            }
            Err(_) => {
                client.set_focus(&focus);
            }
        },
        Command::StartBreak => {
            client.start_break()?;
        }
        Command::EndBreak => {
            client.end_break()?;
        }
        Command::EndDay => {
            client.end_day();
        }
        Command::Note { note } => {
            client.note(note);
        }
    }

    Ok(())
}
