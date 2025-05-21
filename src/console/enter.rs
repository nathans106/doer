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
    Priority {
        #[command(subcommand)]
        command: PriorityCommand,
    },
    Show {
        /// Date to show
        date: String,
    },
    ShowLast,
    Context {
        #[command(subcommand)]
        command: ContextCommand,
    },
    SetFocus {
        /// name or ID of focus
        focus: String,
    },
    Break {
        #[command(subcommand)]
        command: BreakCommand,
    },
    EndDay,
    Note {
        #[command(subcommand)]
        command: NoteCommand,
    },
}

#[derive(Subcommand)]
enum PriorityCommand {
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
    Tick {
        id: u8,
    },
    UnTick {
        id: u8,
    },
}

#[derive(Subcommand)]
enum ContextCommand {
    Show,
    ShowAll,
    Set {
        /// Context to switch to
        context: String,
    },
}

#[derive(Subcommand)]
enum BreakCommand {
    Start,
    End,
}

#[derive(Subcommand)]
enum NoteCommand {
    Add {
        /// Note for today
        note: String,
    },
    Copy {
        /// Date to copy from. Can be 'today', 'tomorrow', or a date in 'ISO format'
        from: String,
        /// Date to copy to. Can be 'today', 'tomorrow', or a date in 'ISO format'
        to: String,
        /// Note ID to copy
        id: u16,
    },
    CopyAll {
        /// Date to copy from. Can be 'today', 'tomorrow', or a date in 'ISO format'
        from: String,
        /// Date to copy to. Can be 'today', 'tomorrow', or a date in 'ISO format'
        to: String,
    },
    Clear {
        /// Date to clear
        date: String,
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
        Command::Priority { command } => match command {
            PriorityCommand::Plan { date } => {
                client.plan_priorities(&date_from_arg(&date)?)?;
            }
            PriorityCommand::Copy { from, to } => {
                client.copy_priorities(&date_from_arg(&from)?, &date_from_arg(&to)?)?;
            }
            PriorityCommand::Tick { id } => {
                client.tick(&(id - 1))?;
            }
            PriorityCommand::UnTick { id } => {
                client.un_tick(&(id - 1))?;
            }
        },
        Command::Show { date } => {
            client.show(&date_from_arg(&date)?)?;
        }
        Command::ShowLast => {
            let date = client.last_date()?;
            client.show(&date)?;
        }
        Command::Context { command } => match command {
            ContextCommand::Show => {
                client.context()?;
            }
            ContextCommand::ShowAll => {
                client.contexts()?;
            }
            ContextCommand::Set { context } => {
                client.set_context(context)?;
            }
        },
        Command::SetFocus { focus } => match focus.parse::<u8>() {
            Ok(id) => {
                client.set_focus_to_priority(&(id))?;
            }
            Err(_) => {
                client.set_focus(&focus)?;
            }
        },
        Command::Break { command } => match command {
            BreakCommand::Start => {
                client.start_break()?;
            }
            BreakCommand::End => {
                client.end_break()?;
            }
        },
        Command::EndDay => {
            client.end_day()?;
        }
        Command::Note { command } => match command {
            NoteCommand::Add { note } => {
                client.note(note)?;
            }
            NoteCommand::Copy { from, to, id } => {
                client.copy_note(
                    &date_from_arg(&from)?,
                    &date_from_arg(&to)?,
                    (id - 1) as usize,
                )?;
            }
            NoteCommand::CopyAll { from, to } => {
                client.copy_notes(&date_from_arg(&from)?, &date_from_arg(&to)?)?;
            }
            NoteCommand::Clear { date } => {
                client.clear_notes(&date_from_arg(&date)?)?;
            }
        },
    }

    Ok(())
}
