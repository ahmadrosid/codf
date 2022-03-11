use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use ignore::Walk;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub struct Document {
    pub paths: Vec<PathBuf>,
    pub matcher: SkimMatcherV2,
}

#[derive(Clone)]
pub struct Row {
    pub line: usize,
    pub raw: String,
    pub file_name: String,
}

impl Document {
    pub fn new() -> Self {
        Self {
            paths: vec![],
            matcher: SkimMatcherV2::default(),
        }
    }

    pub async fn collect_paths(&mut self) {
        let paths = Walk::new("./")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.path().to_path_buf())
            .collect::<Vec<PathBuf>>();
        self.paths = paths;
    }

    pub async fn search(&self, query: &str) -> Vec<Row> {
        let mut result = vec![];
        let max_len: usize = 120;
        let mut total_index: usize = 0;
        for path in &self.paths {
            if total_index > max_len {
                break;
            }

            let file = File::open(path);
            if let Ok(file) = file {
                let mut reader = BufReader::new(file);
                let mut index = 0;
                loop {
                    index += 1;
                    let mut line = String::new();
                    match reader.read_line(&mut line) {
                        Ok(_) => {
                            if line.is_empty() {
                                break;
                            }

                            if let Some((_, _)) = self.matcher.fuzzy_indices(&line, query) {
                                let row = Row {
                                    line: index,
                                    file_name: path.to_str().unwrap().to_string(),
                                    raw: line,
                                };
                                result.push(row);

                                total_index += 1;
                                if total_index > max_len {
                                    break;
                                }
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
