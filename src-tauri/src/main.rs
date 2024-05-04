// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_log::LogTarget;
use tpulse::config;
use tpulse::google_calendar::__cmd__handle_google_calendar;

use tpulse::{db::sqlite, google_calendar::handle_google_calendar};

#[tauri::command]
fn get_data_dir() -> String {
    let user_cfg = config::user();
    user_cfg.data_dir.clone()
}

fn main() {
    // let poll_time: u64 = read_setting::<u64>(Setting::PollTime)
    //     .unwrap_or_else(|err| Some(handle_setting_error(Setting::PollTime, &err, 500)))
    //     .unwrap_or_default();

    // let time_out: u64 = read_setting::<u64>(Setting::Timeout)
    //     .unwrap_or_else(|err| Some(handle_setting_error(Setting::Timeout, &err, 100)))
    //     .unwrap_or_default();

    sqlite::init();

    // let (tx, rx): (Sender<UserMetric>, Receiver<UserMetric>) = mpsc::channel();
    // let afk_tx = tx.clone();
    // let window_tx = tx.clone();
    // let browser_tx = tx.clone();

    // let workers = vec![
    //     thread::spawn(move || watch_browser()),
    //     thread::spawn(move || watch_afk(poll_time, time_out, afk_tx)),
    //     thread::spawn(move || watch_window(poll_time, window_tx)),
    //     thread::spawn(move || handle_events(rx)),
    // ];

    tauri::Builder::default()
        // We cannot see log when running in bundled app.
        // This is a workaround to print log to stdout in production.
        // Can use other log targets
        .invoke_handler(tauri::generate_handler![
            get_data_dir,
            handle_google_calendar
        ])
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::Stdout])
                .build(),
        )
        // This plugin support us access sqlite database directly from Frontend-side
        .plugin(tauri_plugin_sql::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");

    // for worker in workers {
    //     worker.join().unwrap();
    // }
}
