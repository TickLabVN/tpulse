use crate::dtos::{AFKEvent, WindowInformation};
use lazy_static::lazy_static;
use rusqlite::{params, Connection};
use std::{fs, path};

fn get_data_directory() -> String {
    #[cfg(target_os = "linux")]
    {
        let home_dir = dirs::home_dir().expect("Failed to get home directory");
        let data_dir = home_dir.join(".ticklabvn.tpulse");
        data_dir.to_string_lossy().into_owned()
    }

    #[cfg(target_os = "windows")]
    {
        let app_data_dir = dirs::data_local_dir().expect("Failed to get local app data directory");
        let data_dir = app_data_dir.join(".ticklabvn.tpulse"); // Change "YourAppName" to your actual app name
        data_dir.to_string_lossy().into_owned()
    }

    #[cfg(target_os = "macos")]
    {
        let app_support_dir =
            dirs::data_local_dir().expect("Failed to get local app support directory");
        let data_dir = app_support_dir.join(".ticklabvn.tpulse"); // Change "YourAppName" to your actual app name
        data_dir.to_string_lossy().into_owned()
    }
}


// Initialize the database
lazy_static! {
    static ref DB_PATH: String = {
        let db_path = format!("{}/tpulse.sqlite3", get_data_directory());
        if !path::Path::new(&db_path).exists() {
            // If the file does not exist, create the necessary directory and the file
            if let Some(parent_dir) = path::Path::new(&db_path).parent() {
                fs::create_dir_all(parent_dir).expect("Failed to create directory");
            }
            fs::File::create(&db_path).expect("Failed to create file");
        }

        let conn = Connection::open(&*DB_PATH).unwrap();
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
        db_path
    };
}

/// Insert a new afk log entry
pub fn insert_afk_log(afk_log: &AFKEvent) {
    let conn = Connection::open(&*DB_PATH).unwrap();
    conn.execute(
        "INSERT INTO afk_log (time, status) VALUES (?1, ?2)",
        [afk_log.time, afk_log.status as u64],
    )
    .expect("insert afk_log");
}

pub fn insert_window_log(window_log: &WindowInformation) {
    let conn = Connection::open(&*DB_PATH).unwrap();
    let time = window_log.time as i64;
    let title = window_log.title.clone();
    let class = match window_log.class.clone() {
        Some(class) => class.join("|"),
        None => "".to_string(),
    };
    let exec_path = window_log.exec_path.clone().unwrap();
    conn.execute(
        "INSERT INTO window_log VALUES (?1, ?2, ?3, ?4)",
        params![time, title, class, exec_path],
    )
    .expect("insert window_log");
}
