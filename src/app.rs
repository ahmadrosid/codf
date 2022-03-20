use crate::{document::DocResult, document::Document, ui::render};
use crossterm::event::{self, Event, KeyCode};
use std::{fs, io::BufRead, io::BufReader};
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{backend::Backend, Terminal};

pub enum InputMode {
    Normal,
    Editing,
    OpenFile,
}

#[derive(Default)]
pub struct Scroll {
    pub x: u16,
    pub y: u16,
}

pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub messages: Vec<DocResult>,
    pub doc: Document,
    pub index: usize,
    pub total_files: usize,
    pub file_contents: Vec<String>,
    pub scroll: Scroll,
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
            file_contents: Vec::new(),
            scroll: Scroll::default(),
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

    pub fn update_scroll(&mut self, key: KeyCode) {
        match key {
            KeyCode::Down | KeyCode::Char('j') => {
                if self.scroll.y >= self.file_contents.len() as u16 - 1 {
                    return;
                }
                self.scroll.y += 1;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.scroll.y == 1 {
                    return;
                }
                self.scroll.y -= 1;
            }
            KeyCode::Right => {
                let max_x = self
                    .file_contents
                    .get(self.scroll.y as usize)
                    .unwrap_or(&"".into())
                    .len() as u16;
                if self.scroll.x >= max_x - 1 {
                    return;
                }

                self.scroll.x += 1;
            }
            KeyCode::Left => {
                if self.scroll.x == 1 {
                    return;
                }
                self.scroll.x -= 1;
            }
            _ => {}
        }
    }

    pub fn update_total_files(&mut self) {
        self.total_files = self.doc.paths.len();
    }

    pub fn open_file(&mut self) -> Option<()> {
        let row = self.messages.get(self.index)?;
        self.scroll.y = if row.contents[0].line > 7 {
            row.contents[0].line - 6
        } else {
            1
        } as u16;

        let file = fs::File::open(&row.path);
        if let Ok(file) = file {
            let mut reader = BufReader::new(file);
            loop {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(_) => {
                        if line.is_empty() {
                            break;
                        }
                        line = line.replace("\t", "    ");
                        self.file_contents.push(line);
                    }
                    _ => break,
                };
            }
            return Some(());
        }
        None
    }
}
