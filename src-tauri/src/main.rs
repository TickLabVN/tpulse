// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use tauri_plugin_log::LogTarget;
use tpulse::{initializer::initialize_db, watcher::watch_afk};

fn main() {
    initialize_db();

    let afk_watch = thread::spawn(move || watch_afk(5000, 50000));
    let window_watch = thread::spawn(move || tpulse::watcher::watch_window(1000));

    tauri::Builder::default()
        // We cannot see log when running in bundled app.
        // This is a workaround to print log to stdout in production.
        // Can use other log targets
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::Stdout])
                .build(),
        )
        // This plugin support us access sqlite database directly from Frontend-side
        .plugin(tauri_plugin_sql::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");

    afk_watch.join().unwrap();
    window_watch.join().unwrap();
}
