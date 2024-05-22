// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use tauri_plugin_log::LogTarget;
use tpulse::{
    config,
    initializer::initialize_db,
    metrics::UserMetric,
    watcher::{watch_afk, watch_browser, watch_window},
};

#[tauri::command]
fn get_home_dir() -> String {
    dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "".to_string())
}

fn main() {
    let setting = config::get_setting();
    initialize_db();

    let (tx, _rx): (Sender<UserMetric>, Receiver<UserMetric>) = mpsc::channel();
    let afk_tx = tx.clone();
    let window_tx = tx.clone();

    let workers = vec![
        thread::spawn(move || watch_browser()),
        thread::spawn(move || watch_afk(setting.poll_time, setting.poll_time, afk_tx)),
        thread::spawn(move || watch_window(setting.poll_time, window_tx)),
    ];

    tauri::Builder::default()
        // We cannot see log when running in bundled app.
        // This is a workaround to print log to stdout in production.
        // Can use other log targets
        .invoke_handler(tauri::generate_handler![get_home_dir])
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::Stdout])
                .build(),
        )
        // This plugin support us access sqlite database directly from Frontend-side
        .plugin(tauri_plugin_sql::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");

    for worker in workers {
        worker.join().unwrap();
    }
}
