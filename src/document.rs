use crossbeam::channel::Sender;
use fuzzy_matcher::clangd::ClangdMatcher;
use fuzzy_matcher::FuzzyMatcher;
use ignore::WalkBuilder;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

pub struct Document {
    pub paths: HashSet<PathBuf>,
    pub matcher: ClangdMatcher,
}

#[derive(Clone, Default)]
pub struct Row {
    pub line: usize,
    pub raw: String,
    pub file_name: String,
}

pub enum DirEntry {
    Message(ignore::DirEntry),
}

impl DirEntry {
    pub fn path(&self) -> &Path {
        match *self {
            DirEntry::Message(ref entry) => entry.path(),
        }
    }
}

impl Document {
    pub fn new() -> Self {
        Self {
            paths: HashSet::default(),
            matcher: ClangdMatcher::default(),
        }
    }

    pub fn collect_paths(send: &Sender<DirEntry>) {
        let walker = WalkBuilder::new("./").threads(2).build_parallel();
        walker.run(|| {
            let send = send.clone();
            Box::new(move |result| {
                use ignore::WalkState::{Continue, Quit};
                if let Ok(entry) = result {
                    if let Err(_) = send.send(DirEntry::Message(entry)) {
                        return Quit;
                    }
                }
                Continue
            })
        });
    }

    pub fn search(&self, query: &str) -> Vec<Row> {
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
