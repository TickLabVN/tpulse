// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use std::{fs, sync::mpsc, thread};
use tauri::Manager;
use tpulse::app::create_app;
use tpulse::processor;
use tpulse::{
    collector::{watch_afk, watch_browser, watch_window},
    db,
    metrics::UserMetric,
    raw_metric_processor,
};

fn main() {
    dotenv().ok();
    env_logger::init();
    let mut metric_processor_manager = raw_metric_processor::initialize();

    let metric_processor = processor::create_processor();

    let (tx, rx) = mpsc::channel::<UserMetric>();
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
                metric_processor_manager.handle_metric(user_metric.clone());
                metric_processor.process(&user_metric);
            }
        }),
    ];

    for worker in workers {
        worker.join().unwrap();
    }
}
