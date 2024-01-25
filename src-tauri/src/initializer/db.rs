use std::{fs, path};

use rusqlite::Connection;

use crate::categorizer::load_table_from_path;
use crate::utils::get_data_directory;

pub fn initialize_db() {
    let db_path = format!("{}/tpulse.sqlite3", get_data_directory());
    if !path::Path::new(&db_path).exists() {
        // If the file does not exist, create the necessary directory and the file
        if let Some(parent_dir) = path::Path::new(&db_path).parent() {
            fs::create_dir_all(parent_dir).expect("Failed to create directory");
        }
        fs::File::create(&db_path).expect("Failed to create file");
    }

    let mut conn = Connection::open(db_path).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS afk_log (
                time            INTEGER PRIMARY KEY,
                status          INTEGER NOT NULL
            )",
        [],
    )
    .expect("create afk_log table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS window_log (
                time            INTEGER PRIMARY KEY,
                title            TEXT,
                class           TEXT,
                execPath        TEXT
            )",
        [],
    )
    .expect("create window_log table");

    load_table_from_path(
        &mut conn,
        "tracking_rule",
        "Tracking_Rule_Package_Default_Export.csv",
        b',',
    )
    .expect("create tracking_rule table");
}
