use rusqlite::{Connection, Error, Result};
use std::{fs, path};

use crate::config;

// TODO: move seed data & migration script to a separate file
fn create_mock_data(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks VALUES
        (1, 1714794000, 1714795000, 'Architecture Design', NULL),
        (2, 1714796000, 1714799600, 'Learn Rust', NULL)",
        [],
    )?;

    conn.execute(
        "INSERT INTO activity (identifier, category_tag) VALUES
            ('tpulse - Visual Studio Code', 'Category X'),
            ('Spotify', 'Category Y'),
            ('youtube.com/watch?v=bS9em7Bg0iU', 'Category Z'),
            ('stackoverflow.com', 'Category X')",
        [],
    )?;

    conn.execute(
        "INSERT INTO log (start_time, end_time, activity_identifier, task_id) VALUES
            (3600, 7200, 'tpulse - Visual Studio Code', NULL),
            (7200, 10800, 'Spotify', NULL),
            (10800, 14400, 'youtube.com/watch?v=bS9em7Bg0iU', NULL),
            (14400, 16200, 'Spotify', NULL),
            (18000, NULL, 'tpulse - Visual Studio Code', NULL)",
        [],
    )?;

    Ok(())
}

pub fn init() {
    let db_path = format!("{}/tpulse.sqlite3", config::user().data_dir);
    if path::Path::new(&db_path).exists() {
        fs::remove_file(&db_path).expect("Failed to remove existing database file");
    }

    if let Some(parent_dir) = path::Path::new(&db_path).parent() {
        fs::create_dir_all(parent_dir).expect("Failed to create directory");
    }
    fs::File::create(&db_path).expect("Failed to create file");

    let conn = Connection::open(&db_path).expect("Failed to open database connection");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            \"from\" INTEGER NOT NULL,
            \"to\" INTEGER NOT NULL,
            name TEXT NOT NULL,
            project_id INTEGER
        )",
        [],
    )
    .expect("Failed to create tasks table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS activity (
            identifier TEXT PRIMARY KEY,
            category_tag TEXT
        )",
        [],
    )
    .expect("Failed to create activity table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS log (
            start_time TEXT PRIMARY KEY,
            end_time TEXT,
            activity_identifier TEXT NOT NULL,
            task_id INTEGER,
            FOREIGN KEY(task_id) REFERENCES tasks(id)
            FOREIGN KEY(activity_identifier) REFERENCES activity(identifier)
        )",
        [],
    )
    .expect("Failed to create log table");

    if activity_is_empty(&conn).expect("Failed to check activity_log table") {
        let _ = create_mock_data(&conn).expect("Failed to create mock data");
    }

    check_mock_data(&conn, "activity").expect("No mock data in 'activity' table");
    check_mock_data(&conn, "log").expect("No mock data in 'log' table");

    conn.execute(
        "CREATE VIEW IF NOT EXISTS activity_log AS
        SELECT activity.identifier AS name, 
            log.start_time, 
            log.end_time, 
            activity.category_tag, 
            log.task_id
        FROM activity 
        JOIN log ON activity.identifier = log.activity_identifier",
        [],
    )
    .expect("Failed to create activity_log view");
}

fn activity_is_empty(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM activity")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    Ok(count == 0)
}

fn check_mock_data(conn: &Connection, table_name: &str) -> Result<(), Error> {
    let mut stmt = conn.prepare(&format!("SELECT COUNT(*) FROM {}", table_name))?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    assert!(count > 0, "No mock data found in table {}", table_name);
    Ok(())
}
