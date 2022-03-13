mod app;
mod document;
mod indexer;
mod terminal;
mod ui;

fn main() {
    indexer::index().unwrap();
    // terminal::process();
}
