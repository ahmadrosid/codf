use std::{
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc, Mutex},
};

enum Message {
    Work(Work),
    Quit,
}

struct Work {
    path: PathBuf,
}

struct Worker {
    stack: Arc<Mutex<Vec<Message>>>,
    quit_now: Arc<AtomicBool>,
}
