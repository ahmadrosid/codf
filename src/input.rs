use crate::{
    app::{App, InputMode},
    document::DocResult,
    document::Document,
    ui::render,
};
use crossterm::event::{self, Event, KeyCode};
use std::{fs, io::BufRead, io::BufReader};
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{backend::Backend, Terminal};

pub fn watch<F, B: Backend>(terminal: &mut Terminal<B>, mut app: App, f: F) -> io::Result<()>
where
    F: Fn(&mut App),
{
    let timeout = Duration::from_millis(0);

    loop {
        terminal.draw(|frame| render(frame, &app))?;
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('i') => {
                            app.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') | KeyCode::Esc => {
                            return Ok(());
                        }
                        KeyCode::Enter => {
                            if app.open_file().is_some() {
                                app.input_mode = InputMode::OpenFile;
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            app.move_up();
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            app.move_down();
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            if app.open_file().is_some() {
                                app.input_mode = InputMode::OpenFile;
                            }
                        }
                        KeyCode::Up => {
                            app.move_up();
                        }
                        KeyCode::Down => {
                            app.move_down();
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                            app.search();
                            app.index = 0;
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                            app.search();
                            app.index = 0;
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                    InputMode::OpenFile => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.input_mode = InputMode::Editing;
                            app.file_contents = vec![];
                        }
                        KeyCode::Up
                        | KeyCode::Down
                        | KeyCode::Left
                        | KeyCode::Right
                        | KeyCode::Char('j' | 'k') => app.update_scroll(key.code),
                        _ => {}
                    },
                }
            }
        }
        f(&mut app);
    }
}
