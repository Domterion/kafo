use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

use colored::*;

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

        if self.config.move_existing {
            self.do_existing();
        }

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

        let ext = FileWatcher::ext(&e).unwrap_or("".to_string());
        let name = e.file_name();

        let dir = self.get_match(ext);

        if let None = dir {
            return;
        }

        let dest = format!(
            "{}/{}/{}",
            self.config.path,
            dir.unwrap(),
            name.unwrap().to_str().unwrap().to_string()
        );

        let _res = fs::rename(e, dest);
    }

    pub fn do_existing(&self) {
        let path = PathBuf::from(&self.config.path);

        if path.is_dir() {
            println!("{}", "Moving existing files.".green().bold());
            for entry in fs::read_dir(path).expect(&format!(
                "[x] {}",
                "Failed to read path to do existing.".red()
            )) {
                let entry = entry.expect(&format!(
                    "[x] {}",
                    "Unknown error when reading entry.".red()
                ));
                let path = entry.path();
                if !path.is_dir() {
                    self.handle_create_event(path)
                }
            }
            println!("{}", "Done moving existing files.".green().bold());
        }
    }

    fn ext(file: &PathBuf) -> Option<String> {
        let ext = file.extension();

        match ext {
            Some(s) => return Some(s.to_str().unwrap().to_string().to_ascii_lowercase()),
            None => return None,
        }
    }
}
