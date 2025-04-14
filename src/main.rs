use crate::console::enter::enter;

mod config;
mod console;
mod model;
mod storage;

fn main() -> anyhow::Result<()> {
    enter()
}
