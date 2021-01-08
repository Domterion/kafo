use colored::*;
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub path: String,
    pub delay: u64,
    pub make_folders: bool,
    pub dirs: Vec<Dirs>,
}

#[derive(Serialize, Deserialize)]
pub struct Dirs {
    pub name: String,
    pub exts: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        let config = "config.json";
        let contents =
            fs::read_to_string(config).expect(&format!("[x] {}", "Failed to read config.".red()));

        let config: Config = serde_json::from_str(&contents).expect(&format!(
            "[x] {}",
            "Improper json formatting in config.".red()
        ));

        return config;
    }
}
