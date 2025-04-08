use crate::console::enter::enter;

mod console;
mod model;

fn main() -> anyhow::Result<()> {
    enter()
}
