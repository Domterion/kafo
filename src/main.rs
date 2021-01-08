use std::fs;
use std::panic;
use std::path::Path;

mod file_watcher;
use file_watcher::FileWatcher;

mod config;
use config::Config;

fn main() {
    let config = Config::new();

    if config.make_folders {
        println!("Making folders.");
        println!("----------------");
        for i in &config.dirs {
            let dir = format!("{}/{}", config.path, i.name);

            if Path::new(&dir).exists() {
                println!("{} already exists, skipping!", i.name);
                continue;
            }

            fs::create_dir(&dir).expect(&format!("Err when making folder {}", i.name));
            println!("Made {} folder.", i.name)
        }
        println!("----------------");
        println!("Done making folders.")
    }

    let fw = FileWatcher::new(config);

    if let Err(e) = fw.watch() {
        println!("Err: {:?}", e)
    }
}
