use crate::{document::Document, document::Row, ui::render};
use crossbeam::epoch::Pointable;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::Backend,
    Terminal,
};

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub messages: Vec<Row>,
    pub doc: Document,
    pub index: usize,
    pub total_files: usize,
    current_time: Instant,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Editing,
            messages: Vec::new(),
            doc: Document::new(),
            index: 0,
            total_files: 0,
            current_time: Instant::now(),
        }
    }
}

impl App {
    pub fn search(&mut self) {
        let duration = self.current_time.elapsed();
        if duration < Duration::from_millis(500) {
            return;
        }
        self.messages = self.doc.search(&self.input);
        self.current_time = Instant::now();
    }

    pub fn move_up(&mut self) {
        if self.index == 0 {
            return;
        }
        self.index -= 1;
    }

    pub fn move_down(&mut self) {
        if self.index == self.messages.len() - 1 {
            return;
        }
        self.index += 1;
    }

    pub fn update_total_files(&mut self) {
        self.total_files = self.doc.paths.len();
    }
}

pub fn run<F, B: Backend>(terminal: &mut Terminal<B>, mut app: App, f: F) -> io::Result<String>
where
    F: Fn(&mut App)
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
                            return Ok(String::new());
                        }
                        KeyCode::Up => {
                            app.move_up();
                        }
                        KeyCode::Down => {
                            app.move_down();
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            // TODO: open file with default editor
                            if let Some(row) = app.messages.get(app.index) {
                                return Ok(row.file_name.to_string());
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
                }
            }
        }
        f(&mut app);
    }

    // let walker = WalkBuilder::new(".").threads(6).build_parallel();
    // walker.run(|| {
    //     let tx = tx.clone();
    //     Box::new(move |result| {
    //         use ignore::WalkState::*;

    //         tx.send(DirEntry::Y(result.unwrap())).unwrap();
    //         Continue
    //     })
    // });

    // drop(tx);
    // collect_dirs_thread.join().unwrap();
}
