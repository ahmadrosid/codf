mod app;
mod document;
mod terminal;
mod ui;
mod worker;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    terminal::process()
}
