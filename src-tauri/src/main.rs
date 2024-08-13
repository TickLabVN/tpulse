// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenvy::dotenv;
use std::fs;
use tauri::Manager;
use tpulse::app::create_app;
use tpulse::{db, metric::start_collector};

fn main() {
    dotenv().ok();
    env_logger::init();
    let app = create_app();

    let db_path = app.path().app_config_dir().unwrap().join("tpulse.sqlite3");
    fs::create_dir_all(db_path.parent().unwrap()).unwrap();
    let db_path = db_path.to_str().unwrap();
    db::set_path(db_path);
    db::apply_migrations();

    let workers = start_collector();
    app.run(|_, _| {});
    for w in workers {
        w.join().unwrap();
    }
}
