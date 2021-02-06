use core::panic;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

mod config;
mod repo;
use config::Config;

fn sync(config: &Config, path: &PathBuf) {
    let path_str = path.to_str().unwrap();
    match config.find(path_str) {
        Some(dir) => {
            println!("notes_sync: Syncing for {} in {}", path_str, dir);
            repo::sync(&dir).unwrap();
        }
        _ => {
            println!("notes_sync: No repo found for path {}", path_str);
        }
    };
}

fn main() {
    let config = Config::read().unwrap();
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(config.debounce)).unwrap();

    for dir in config.directories.iter() {
        println!("notes_sync: Watching {}", dir);
        watcher.watch(dir, RecursiveMode::Recursive).unwrap();
    }

    loop {
        match rx.recv() {
            Ok(f) => match f {
                DebouncedEvent::NoticeWrite(p) => sync(&config, &p),
                DebouncedEvent::NoticeRemove(p) => sync(&config, &p),
                DebouncedEvent::Create(p) => sync(&config, &p),
                DebouncedEvent::Write(p) => sync(&config, &p),
                DebouncedEvent::Chmod(p) => sync(&config, &p),
                DebouncedEvent::Remove(p) => sync(&config, &p),
                DebouncedEvent::Rename(p, _) => sync(&config, &p),
                _ => {}
            },
            Err(e) => panic!("Bad event!"),
        }
    }
}
