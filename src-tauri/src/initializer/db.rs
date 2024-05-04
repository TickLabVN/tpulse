use std::{fs, path};

use rusqlite::{Connection, Error, Result};

use crate::utils::get_data_directory;

fn create_mock_data(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (day, start_time, end_time, task_name, category_tag, priority_tag) VALUES
            ('2024-03-21', 11111111, 22222222, 'Task 1', 'Category A', 'high'),
            ('2024-03-22', 22222222, 33333333, 'Task 2', 'Category B', 'medium'),
            ('2024-03-23', 33333333, 44444444, 'Task 3', 'Category C', 'low')",
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
            (11111111, 22222222, 'tpulse - Visual Studio Code', '1'),
            (22222222, 33333333, 'Spotify', NULL),
            (33333333, 44444444, 'youtube.com/watch?v=bS9em7Bg0iU', '2'),
            (44444444, NULL, 'Spotify', NULL),
            (55555555, NULL, 'tpulse - Visual Studio Code', '1')",
        [],
    )?;

    Ok(())
}

pub fn initialize_db() {
    let db_path = format!("{}/tpulse.sqlite3", get_data_directory());

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
            day DATE NOT NULL,
            start_time INTEGER,
            end_time INTEGER,
            task_name TEXT NOT NULL,
            category_tag TEXT,
            priority_tag TEXT CHECK(priority_tag IN ('high', 'medium', 'low'))
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
            start_time INTEGER PRIMARY KEY,
            end_time INTEGER,
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

    check_mock_data(&conn, "tasks").expect("No mock data in 'tasks' table");
    check_mock_data(&conn, "activity").expect("No mock data in 'activity' table");
    check_mock_data(&conn, "log").expect("No mock data in 'activity' table");

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
