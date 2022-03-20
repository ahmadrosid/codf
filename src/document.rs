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

#[cfg(not(feature = "compact"))]
type ScoreType = i64;
#[cfg(feature = "compact")]
type ScoreType = i32;

pub struct Document {
    pub paths: HashSet<PathBuf>,
    pub matcher: ClangdMatcher,
}

#[derive(Clone, Default)]
pub struct Row {
    pub line: usize,
    pub raw: String,
    pub score: ScoreType
}

pub struct DocResult {
    pub path: PathBuf,
    pub file_name: String,
    pub contents: Vec<Row>,
}

pub enum DirEntry {
    Message(ignore::DirEntry),
}

impl DocResult {
    pub fn new(file_path: &PathBuf) -> Self {
        Self {
            path: file_path.clone(),
            file_name: file_path.file_name().unwrap().to_string_lossy().to_string(),
            contents: vec![],
        }
    }

    pub fn push(&mut self, row: Row) {
        self.contents.push(row);
    }
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
                    if send.send(DirEntry::Message(entry)).is_err() {
                        return Quit;
                    }
                }
                Continue
            })
        });
    }

    pub fn search(&self, query: &str) -> Vec<DocResult> {
        let mut results: Vec<DocResult> = vec![];
        let max_file = 120;
        let mut count_file = 0;
        for path in &self.paths {
            count_file += 1;
            if count_file > max_file {
                break;
            }

            let mut result = DocResult::new(path);
            let file = File::open(path);

            let max_line: usize = 20;
            let mut current_line: usize = 0;
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

                            if let Some((score, _)) = self.matcher.fuzzy_indices(&line, query) {
                                let row = Row {
                                    line: index,
                                    raw: line,
                                    score
                                };
                                result.push(row);

                                current_line += 1;
                                if current_line > max_line {
                                    break;
                                }
                            }
                        }
                        _ => break,
                    };
                }

                if !result.contents.is_empty() {
                    // result.contents.sort_by(|a, b| a.score.cmp(&b.score));
                    results.push(result);
                }
            }
        }
        results
    }
}
