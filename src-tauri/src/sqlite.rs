use crate::{
    events::{AFKEvent, BrowserInformation, WindowInformation},
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
    let start_time = afk_log.start_time_unix as i64;
    //TODO: add logic to handle end_time
    let end_time = start_time + 5 * 60;
    conn.execute(
        "INSERT INTO afk_log (start_time, end_time, status) VALUES (?1, ?2, ?3)",
        params![start_time, end_time, afk_log.status as i64],
    )
    .expect("Failed to insert into afk_log");
}

pub fn insert_window_log(window_log: &WindowInformation) {
    let conn = Connection::open(&*DB_PATH).unwrap();

    let start_time = window_log.time as i64;
    //TODO: add logic to handle end_time
    let end_time = start_time + 5 * 60;
    let title = window_log.title.clone();
    let class = match &window_log.class {
        Some(class) => class.clone().join("|"),
        None => "".to_string(),
    };
    let exec_path = window_log
        .exec_path
        .clone()
        .unwrap_or_else(|| "".to_string());

    conn.execute(
        "INSERT INTO window_log (start_time, end_time, title, class, execPath) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![start_time, end_time, title, class, exec_path],
    )
    .expect("Failed to insert into window_log");
}

pub fn insert_browser_log(browser_log: &BrowserInformation) {
    let conn = Connection::open(&*DB_PATH).unwrap();

    println!("Inserting browser log: {:?}", browser_log);
    println!("DB_PATH: {:?}", &*DB_PATH);

    let start = &browser_log.start_time;
    let end = browser_log.start_time.clone();
    let title = browser_log.title.clone();

    // Update the end time of the last row
    match conn.execute(
        "UPDATE browser_log SET end = ?1 WHERE rowid = (SELECT MAX(rowid) FROM browser_log)",
        params![start],
    ) {
        Ok(x) => println!("Successfully updated end time of last row: {:?}", x),
        Err(err) => eprintln!("Failed to update end time of last row: {}", err),
    }

    // Insert the new row
    let params = params![1, start, end, title];
    match conn.execute(
        "INSERT INTO browser_log (activity_id, start, end, title) VALUES (?1, ?2, ?3, ?4)",
        params,
    ) {
        Ok(x) => println!("Successfully inserted into browser_log: {:?}", x),
        Err(err) => eprintln!("Failed to insert into browser_log: {}", err),
    }
}
