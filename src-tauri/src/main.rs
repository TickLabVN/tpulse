// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use tauri_plugin_log::LogTarget;
use tpulse::setting::read_setting;
use tpulse::{
    event_handler::handle_events,
    events::UserMetric,
    initializer::initialize_db,
    metrics_handler::handle_metrics,
    setting::{handle_setting_error, Setting},
    watcher::{watch_afk, watch_window},
};
fn main() {
    let poll_time: u64 = read_setting::<u64>(Setting::PollTime)
        .unwrap_or_else(|err| Some(handle_setting_error(Setting::PollTime, &err, 500)))
        .unwrap_or_default();

    let time_out: u64 = read_setting::<u64>(Setting::Timeout)
        .unwrap_or_else(|err| Some(handle_setting_error(Setting::Timeout, &err, 100)))
        .unwrap_or_default();

    initialize_db();

    let (tx, rx): (Sender<UserMetric>, Receiver<UserMetric>) = mpsc::channel();
    let afk_tx = tx.clone();
    let window_tx = tx.clone();
    // let initialize_db = thread::spawn(move || initialize_db());
    // let open_pipe_server = thread::spawn(move || handle_metrics());
    // let afk_watcher = thread::spawn(move || watch_afk(5000, 50000, afk_tx));
    // let window_watcher = thread::spawn(move || watch_window(1000, window_tx));
    // let event_handler = thread::spawn(move || handle_events(rx));

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
        // .invoke_handler(tauri::generate_handler![get_db_path])
        .plugin(tauri_plugin_sql::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
    // initialize_db.join().unwrap();
    // afk_watcher.join().unwrap();
    // window_watcher.join().unwrap();
    // event_handler.join().unwrap();
    // open_pipe_server.join().unwrap();
}
