use crate::{document::Document, document::Row, ui::render};
use crossterm::event::{self, Event, KeyCode};
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
    OpenFile,
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
                            if let Some(_) = app.messages.get(app.index) {
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
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Editing;
                        }
                        _ => {}
                    },
                }
            }
        }
        f(&mut app);
    }
}

