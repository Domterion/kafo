use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::config::Config;

pub struct FileWatcher {
    pub config: Config,
}

impl FileWatcher {
    pub fn new(config: Config) -> Self {
        FileWatcher { config }
    }

    pub fn watch(&self) -> Result<(), notify::Error> {
        let (tx, rx) = channel();

        let mut watcher: RecommendedWatcher =
            Watcher::new(tx, Duration::from_secs(self.config.delay))?;

        watcher.watch(&self.config.path, RecursiveMode::Recursive)?;

        loop {
            match rx.recv() {
                Ok(e) => self.dispatch(e),
                Err(e) => println!("File watch err: {:?}", e),
            }
        }
    }

    fn dispatch(&self, e: notify::DebouncedEvent) {
        match e {
            notify::DebouncedEvent::Create(ev) => self.handle_create_event(ev),
            _ => (),
        }
    }

    fn get_match(&self, ext: String) -> Option<String> {
        for i in &self.config.dirs {
            if i.exts.contains(&ext) {
                return Some(i.name.clone());
            }
        }

        return None;
    }

    fn handle_create_event(&self, e: PathBuf) {
        if e.is_dir() {
            return;
        };

        let ext = FileWatcher::ext(&e).expect("Error getting extension in create event.");
        let _name = e.file_name();

        println!("{:?}", self.get_match(ext));
    }

    fn ext(file: &PathBuf) -> Option<String> {
        let ext = file.extension();

        match ext {
            Some(s) => return Some(s.to_str().unwrap().to_string().to_ascii_lowercase()),
            None => return None,
        }
    }
}
