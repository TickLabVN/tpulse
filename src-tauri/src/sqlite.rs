use crate::{
    raw_metric_processor::{StartActivity, UpdateEndActivity},
    utils::get_data_directory,
};
use lazy_static::lazy_static;
use rusqlite::{params, Connection};

// Initialize the database
lazy_static! {
    static ref DB_PATH: String = format!("{}/tpulse.sqlite3", get_data_directory());
}

pub fn insert_new_log(start_log_event: &StartActivity) {
    let conn = Connection::open(&*DB_PATH).expect("Failed to open database connection");

    let start_time = &start_log_event.start_time;
    let activity_id = &start_log_event.activity_identifier;
    let activity_tag: &String = &start_log_event.tag.to_string();

    let activity_exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM activity WHERE identifier = ?1)",
            params![activity_id],
            |row| row.get(0),
        )
        .unwrap_or(false);

    if !activity_exists {
        conn.execute(
            "INSERT INTO activity (identifier, activity_tag) VALUES (?1, ?2)",
            params![activity_id, activity_tag],
        )
        .expect("Failed to create new activity");
    }

    let log_exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM log WHERE start_time = ?1)",
            params![start_time],
            |row| row.get(0),
        )
        .unwrap_or(false);
    if !log_exists {
        conn.execute(
            "INSERT INTO log (start_time, activity_identifier) VALUES (?1, ?2)",
            params![start_time, activity_id],
        )
        .expect("Failed to insert new log");
    }
}

pub fn update_log(end_log_event: &UpdateEndActivity) {
    let conn = Connection::open(&*DB_PATH).expect("Failed to open database connection");

    let start_time_string = &end_log_event.start_time;
    let end_time_string = &end_log_event.end_time;

    let log_exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM log WHERE start_time = ?1)",
            params![start_time_string],
            |row| row.get(0),
        )
        .unwrap_or(false);

    if !log_exists {
        eprintln!("Log entry with start time {} not found", start_time_string);
        return;
    }

    conn.execute(
        "UPDATE log SET end_time = ?1 WHERE start_time = ?2",
        params![end_time_string, start_time_string],
    )
    .expect("Failed to update log");
}
