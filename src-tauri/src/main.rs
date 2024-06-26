// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::mpsc::{self, Receiver, Sender};
use std::{fs, thread};
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};
use tpulse::initializer::raw_metric_processor;
use tpulse::{
    config, db,
    google_calendar::{__cmd__handle_google_calendar, handle_google_calendar},
    metrics::UserMetric,
    watcher::{watch_afk, watch_browser, watch_window},
};

fn main() {
    let setting = config::get_setting();
    let mut metric_processor_manager = raw_metric_processor::initialize();

    let (tx, rx): (Sender<UserMetric>, Receiver<UserMetric>) = mpsc::channel();
    let afk_tx = tx.clone();
    let window_tx = tx.clone();
    let browser_tx = tx.clone();

    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_google_calendar])
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([Target::new(TargetKind::Stdout)])
                .build(),
        )
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:tpulse.sqlite3", vec![])
                .build(),
        )
        .build(tauri::generate_context!())
        .unwrap();

    let db_path = app
        .path()
        .app_local_data_dir()
        .unwrap()
        .join("tpulse.sqlite3");
    // create folder if not exist
    fs::create_dir_all(db_path.parent().unwrap()).unwrap();

    let db_path = db_path.to_str().unwrap();
    db::set_path(db_path);
    db::apply_migrations();

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
    app.run(|_app_handler, _event| {});

    for worker in workers {
        worker.join().unwrap();
    }
}
