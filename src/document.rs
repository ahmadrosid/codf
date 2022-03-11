use ignore::Walk;
use std::{path::PathBuf, fs::File, io::{BufReader, BufRead}};

pub struct Document {
    pub paths: Vec<PathBuf>,
}

impl Document {
    pub fn new() -> Self {
        let paths = Walk::new("./")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.path().to_path_buf())
            .collect::<Vec<PathBuf>>();
        Self { paths }
    }

    pub fn search(&self, query: &str) -> Vec<String> {
        let mut result = vec![];
        for path in &self.paths {
            let file = File::open(path);
            if let Ok(file) = file {
                let mut reader = BufReader::new(file);
                loop {
                    let mut line = String::new();
                    match reader.read_line(&mut line) {
                        Ok(_) => {
                            if line.is_empty() {
                                break;
                            }
                            if line.contains(query) {
                                result.push(line);
                            }
                        }
                        _ => break,
                    };
                }
            }
        }
        result
    }
}
