use crate::app::App;
use crate::document::DirEntry;
use crate::document::Document;
use crate::input::watch;
use crossbeam::channel::bounded;
use crossbeam::channel::Receiver;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};

fn draw(receiver: &Receiver<DirEntry>) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let app = App::default();
    let res = watch(&mut terminal, app, |app| {
        if let Ok(entry) = receiver.recv() {
            if entry.path().is_file() {
                app.doc.paths.insert(entry.path().to_path_buf());
                app.update_total_files();
            }
        }
    });

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

pub fn process() {
    let (sender, receiver) = bounded::<DirEntry>(100);
    let ui_thread = std::thread::spawn(move || {
        draw(&receiver).unwrap();
    });
    Document::collect_paths(&sender);
    drop(sender);
    if ui_thread.join().is_err() {
        println!("Exit!");
    };
}
