mod file_watcher;
use file_watcher::FileWatcher;

mod config;
use config::Config;

fn main() {
    let config = Config::new();

    let fw = FileWatcher::new(config);

    if let Err(e) = fw.watch() {
        println!("Err: {:?}", e)
    }
}
