use std::fs;
use std::panic;
use std::path::Path;

use colored::*;

mod file_watcher;
use file_watcher::FileWatcher;

mod config;
use config::Config;

fn main() {
    let config = Config::new();

    if config.make_folders {
        println!("{}", "Making folders.".green().bold());
        for i in &config.dirs {
            let dir = format!("{}/{}", config.path, i.name);

            if Path::new(&dir).exists() {
                println!(
                    "[x] {}{}",
                    i.name.red().bold(),
                    " already exists, skipping!".red()
                );
                continue;
            }

            fs::create_dir(&dir).expect(&format!("Err when making folder {}", i.name));
            println!("{}{}", i.name.green().bold(), " was made".green());
            println!("Made {} folder.", i.name)
        }
        println!("{}", "Done making folders.".green().bold());
    }

    let fw = FileWatcher::new(config);

    if let Err(e) = fw.watch() {
        println!("[x] {}{:?}", "Err: ".red(), e.to_string().red().bold())
    }
}
