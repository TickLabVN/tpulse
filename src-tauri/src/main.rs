// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use tpulse::watcher::watch_afk;

fn main() {
    env_logger::init();

    let afk_watch = thread::spawn(move || watch_afk(5000, 50000));
    let window_watch = thread::spawn(move || tpulse::watcher::watch_window(1000));

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");

    afk_watch.join().unwrap();
    window_watch.join().unwrap();
}
