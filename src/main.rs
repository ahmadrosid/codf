mod app;
mod document;
mod terminal;
mod ui;
use atty::Stream;

fn main() {
    if atty::is(Stream::Stdout) {
        terminal::process();
    } else {
        println!("Please run from terminal!");
        std::process::exit(0x1);
    }
}
