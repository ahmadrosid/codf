use crate::app::run;
use crate::app::App;
use crate::document::DirEntry;
use crate::document::Document;
use crossbeam::channel::bounded;
use crossbeam::channel::Receiver;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};

fn draw(receiver: Receiver<DirEntry>) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let app = App::default();
    let res = run(&mut terminal, app, |app| match receiver.recv() {
        Ok(entry) => {
            if entry.path().is_file() {
                app.doc.paths.insert(entry.path().to_path_buf());
                app.update_total_files();
            }
        }
        Err(_) => {}
    });

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    match res {
        Err(err) => {
            println!("{:?}", err);
        }
        Ok(message) => {
            println!("{}", message);
        }
    }

    Ok(())
}

pub fn process() -> Result<(), Box<dyn Error>> {
    let (sender, receiver) = bounded::<DirEntry>(100);
    let ui_thread = std::thread::spawn(move || {
        draw(receiver).unwrap();
    });
    Document::collect_paths(&sender);
    drop(sender);
    if let Err(_) = ui_thread.join() {
        println!("Exit!");
    };
    Ok(())
}
