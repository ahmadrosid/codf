mod app;
mod document;
mod terminal;
mod ui;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    terminal::run()
}
