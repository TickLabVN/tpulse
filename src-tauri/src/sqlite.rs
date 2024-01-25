use crate::{
    events::{AFKEvent, WindowInformation},
    utils::get_data_directory,
};
use lazy_static::lazy_static;
use rusqlite::{params, Connection};

// Initialize the database
lazy_static! {
    static ref DB_PATH: String = format!("{}/tpulse.sqlite3", get_data_directory());
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
    let title = match window_log.title.clone() {
        Some(title) => title,
        None => "".to_string(),
    };
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
