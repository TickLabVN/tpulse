// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use tauri_plugin_log::{Target, TargetKind};
use tpulse::initializer::raw_metric_processor;
use tpulse::{
    config,
    google_calendar::{__cmd__handle_google_calendar, handle_google_calendar},
    initializer::db,
    metrics::UserMetric,
    watcher::{watch_afk, watch_browser, watch_window},
};

#[tauri::command]
fn get_data_dir() -> String {
    let user_cfg = config::user::user();
    user_cfg.data_dir.clone()
}

fn main() {
    let setting = config::get_setting();
    db::initialize();
    let mut metric_processor_manager = raw_metric_processor::initialize();

    let (tx, rx): (Sender<UserMetric>, Receiver<UserMetric>) = mpsc::channel();
    let afk_tx = tx.clone();
    let window_tx = tx.clone();
    let browser_tx = tx.clone();

    let workers = vec![
        thread::spawn(move || watch_browser(browser_tx)),
        thread::spawn(move || watch_afk(setting.poll_time, setting.time_out, afk_tx)),
        thread::spawn(move || watch_window(setting.poll_time, window_tx)),
        thread::spawn(move || {
            while let Ok(user_metric) = rx.recv() {
                metric_processor_manager.handle_metric(user_metric);
            }
        }),
    ];

    tauri::Builder::default()
        // We cannot see log when running in bundled app.
        // This is a workaround to print log to stdout in production.
        // Can use other log targets
        .invoke_handler(tauri::generate_handler![
            get_data_dir,
            handle_google_calendar
        ])
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([Target::new(TargetKind::Stdout)])
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
