mod app;
mod document;
mod terminal;
mod ui;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    terminal::run().await
}
