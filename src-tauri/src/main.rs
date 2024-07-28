// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use tpulse::app::create_app;
use std::sync::mpsc::{self, Receiver, Sender};
use std::{fs, thread};
use tauri::Manager;
use tpulse::{
    db,
    metrics::UserMetric,
    collector::{watch_afk, watch_browser, watch_window},
    raw_metric_processor,
};

fn main() {
    dotenv().ok();
    env_logger::init();
    let mut metric_processor_manager = raw_metric_processor::initialize();

    let (tx, rx): (Sender<UserMetric>, Receiver<UserMetric>) = mpsc::channel();
    let afk_tx = tx.clone();
    let window_tx = tx.clone();
    let browser_tx = tx.clone();

    let app = create_app();

    let db_path = app.path().app_config_dir().unwrap().join("tpulse.sqlite3");
    fs::create_dir_all(db_path.parent().unwrap()).unwrap();
    let db_path = db_path.to_str().unwrap();
    db::set_path(db_path);
    db::apply_migrations();

    let workers = vec![
        thread::spawn(move || watch_browser(browser_tx)),
        thread::spawn(move || watch_afk(afk_tx)),
        thread::spawn(move || watch_window(window_tx)),
        thread::spawn(move || {
            while let Ok(user_metric) = rx.recv() {
                metric_processor_manager.handle_metric(user_metric);
            }
        }),
    ];

    for worker in workers {
        worker.join().unwrap();
    }
}
