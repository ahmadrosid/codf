use std::{io, time::{Instant, Duration}};
use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Terminal};

use crate::{document::Document, document::Row, ui::render};

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
            current_time: Instant::now(),
        }
    }
}

impl App {
    pub async fn search(&mut self) {
        let duration = self.current_time.elapsed();
        if duration < Duration::from_millis(500) {
            return;
        }
        self.messages = self.doc.search(&self.input).await;
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
}

pub async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    app.doc.collect_paths().await;
    
    loop {
        terminal.draw(|f| render(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('i') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        return Ok(());
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
                    }
                    KeyCode::Up => {
                        app.move_up();
                    }
                    KeyCode::Down => {
                        app.move_down();
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                        app.search().await;
                        app.index = 0;
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                        app.search().await;
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
}
