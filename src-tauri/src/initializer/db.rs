use std::{fs, path};

use rusqlite::{Connection, Error, Result};

use crate::utils::get_data_directory;

fn create_mock_data(conn: &Connection) -> Result<()> {
    // Insert mock data into tasks table
    conn.execute(
        "INSERT INTO tasks (day, start_time, end_time, task_name, category_tag, priority_tag) VALUES
            ('2024-03-21', 3600, 7200, 'Task 1', 'Category A', 'high'),
            ('2024-03-22', 7200, 10800, 'Task 2', 'Category B', 'medium'),
            ('2024-03-23', 10800, 14400, 'Task 3', 'Category C', 'low')",
        [],
    )?;

    // Insert mock data into activity_log table
    conn.execute(
        "INSERT INTO activity_log (title, start, end, category_tag, task_id) VALUES
            ('Activity 1', '7:00', '8:00', 'Category X', 1),
            ('Activity 2', '10:00', '14:00', 'Category Y', 2),
            ('Activity 3', '12:00', '18:00', 'Category Z', 3)",
        [],
    )?;

    // Insert mock data into afk_log table
    conn.execute(
        "INSERT INTO afk_log (start_time, end_time, status) VALUES
            (100, 200, 1),
            (300, 400, 0),
            (500, 600, 1)",
        [],
    )?;

    // Insert mock data into window_log table
    conn.execute(
        "INSERT INTO window_log (activity_id, start_time, end_time, title, class, execPath) VALUES
            (1, 3600, 7200, 'Window 1', 'Class A', '/path/to/executable'),
            (2, 7200, 10800, 'Window 2', 'Class B', '/path/to/executable'),
            (3, 10800, 14400, 'Window 3', 'Class C', '/path/to/executable')",
        [],
    )?;

    // Insert mock data into browser_log table
    conn.execute(
        "INSERT INTO browser_log (activity_id, start_time, end_time, title) VALUES
            (1, 3600, 7200, 'Page 1'),
            (2, 7200, 10800, 'Page 2'),
            (3, 10800, 14400, 'Page 3')",
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
        "CREATE TABLE IF NOT EXISTS activity_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            start TEXT NOT NULL,
            end TEXT,
            class TEXT,
            execPath TEXT,
            category_tag TEXT,
            task_id INTEGER,
            FOREIGN KEY(task_id) REFERENCES tasks(id)
        )",
        [],
    )
    .expect("Failed to create activity_log table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS afk_log (
            start_time INTEGER PRIMARY KEY,
            end_time INTEGER,
            status INTEGER NOT NULL
        )",
        [],
    )
    .expect("Failed to create afk_log table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS window_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            activity_id INTEGER,
            start_time INTEGER NOT NULL,
            end_time INTEGER,
            title TEXT,
            class TEXT,
            execPath TEXT,
            FOREIGN KEY(activity_id) REFERENCES activity_log(id)
        )",
        [],
    )
    .expect("Failed to create window_log table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS browser_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            activity_id INTEGER,
            start_time TEXT NOT NULL,
            end_time TEXT,
            title TEXT NOT NULL,
            FOREIGN KEY(activity_id) REFERENCES activity_log(id)
        )",
        [],
    )
    .expect("Failed to create browser_log table");

    // Create mock data
    if activity_log_is_empty(&conn).expect("Failed to check activity_log table") {
        // Create mock data only if activity log is empty
        let _ = create_mock_data(&conn).expect("Failed to create mock data");
    }

    check_mock_data(&conn, "tasks").expect("No mock data in tasks table");
    check_mock_data(&conn, "activity_log").expect("No mock data in activity_log table");
    check_mock_data(&conn, "afk_log").expect("No mock data in afk_log table");
    check_mock_data(&conn, "window_log").expect("No mock data in window_log table");
    check_mock_data(&conn, "browser_log").expect("No mock data in browser_log table");
}
fn activity_log_is_empty(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM activity_log")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    Ok(count == 0)
}

fn check_mock_data(conn: &Connection, table_name: &str) -> Result<(), Error> {
    let mut stmt = conn.prepare(&format!("SELECT COUNT(*) FROM {}", table_name))?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    assert!(count > 0, "No mock data found in table {}", table_name);
    Ok(())
}
