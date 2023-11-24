// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tpulse::watcher::{AFKWatcher, AFKSettings};
use std::thread;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    env_logger::init();

    let afk_settings = AFKSettings::new(5000, 500);
    let afk_watcher = AFKWatcher::new(&afk_settings);
    afk_watcher.run();

    let afk_watch = thread::spawn(move || afk_watcher.run());
    afk_watch.join().unwrap();

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
