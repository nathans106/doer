use crate::console::enter::enter;

mod config;
mod console;
mod model;

fn main() -> anyhow::Result<()> {
    enter()
}
