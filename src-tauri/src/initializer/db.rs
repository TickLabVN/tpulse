use regex::Regex;
use rusqlite::{params, Connection, Error, Result};
use serde_json::Value;
use std::{fs, path, str, thread, time::Duration};

use crate::utils::get_data_directory;

fn handle_parse_data(mock_data: &str) -> Result<Value, serde_json::Error> {
    let re_url = Regex::new(r"(http[s]?:)").unwrap();
    let placeholder_data = re_url.replace_all(mock_data, "$1PLACEHOLDER");
    let re_key = Regex::new(r"([^ ,]+):").unwrap();
    let replaced_data = re_key.replace_all(&placeholder_data, "\"$1\":");
    let final_data = replaced_data.replace("PLACEHOLDER", ":");
    let parsed_data: Value = serde_json::from_str(&final_data)?;
    println!("{}", parsed_data);
    Ok(parsed_data)
}

fn insert_parsed_data_into_db(
    conn: &Connection,
    parsed_data: &Value,
) -> Result<(), rusqlite::Error> {
    let time = parsed_data["time"].as_i64().unwrap();
    let title = parsed_data["title"].as_str().unwrap();
    let mut class: Option<Vec<Value>> = None;
    let mut typ: Option<&str> = None;

    if parsed_data["class"].is_array() {
        class = parsed_data["class"].as_array().cloned();
    }
    if parsed_data["type"].is_string() {
        typ = parsed_data["type"].as_str();
    }

    if let Some(class) = class {
        conn.execute(
            "INSERT INTO activity (identifier, category_tag) VALUES (?1, ?2)",
            params![title, class[0].as_str().unwrap()],
        )?;
    } else if let Some(typ) = typ {
        conn.execute(
            "INSERT INTO activity (identifier, category_tag) VALUES (?1, ?2)",
            params![title, typ],
        )?;
    }
    // Update the end time of the last row in the log table
    match conn.execute(
        "UPDATE log SET end_time = ?1 WHERE rowid = (SELECT MAX(rowid) FROM log)",
        params![time],
    ) {
        Ok(x) => println!("Successfully updated end time of last row: {:?}", x),
        Err(err) => eprintln!("Failed to update end time of last row: {}", err),
    }
    // Insert the new row into the log table
    conn.execute(
        "INSERT INTO log (start_time, end_time, activity_identifier, task_id) VALUES (?1, NULL, ?2, NULL)",
        params![time, title],
    )?;
    Ok(())
}
fn create_mock_data(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (day, start_time, end_time, task_name, category_tag, priority_tag) VALUES
            ('2024-03-21', 3600, 7200, 'Task 1', 'Category A', 'high'),
            ('2024-03-22', 7200, 10800, 'Task 2', 'Category B', 'medium'),
            ('2024-03-23', 10800, 14400, 'Task 3', 'Category C', 'low')",
        [],
    )?;
    const mock_activity_data: &[&str] = &[
        "{ time: 1714712400, title: \"TPulse\", class: [\"tpulse\", \"Tpulse\"], exec_path: \"/home/tan17112003/Desktop/tpulse/src-tauri/target/debug/tpulse\" }",
        "{ time: 1714719600, title: \"routeTree.gen.ts - tpulse - Visual Studio Code\", class: [\"code\", \"Code\"], exec_path: \"/usr/share/code/code\" }",
        "{ type: \"BrowserTab\", title: \"haha - TÃ¬m trÃªn Google\", url: \"https://www.google.com/search?q=haha&oq=haha&gs_lcrp=EgZjaHJvbWUyBggAEEUYOdIBCDE0MDhqMGo3qAIAsAIA&sourceid=chrome&ie=UTF-8\", windowId: 952296832, time: 1714742100, tabId: 952296930 }"
    ];
    for &data in mock_activity_data {
        let parsed_data = handle_parse_data(data).expect("dd");
        insert_parsed_data_into_db(conn, &parsed_data)?;
    }
    // conn.execute(
    //     "INSERT INTO activity (identifier, category_tag) VALUES
    //         ('tpulse - Visual Studio Code', 'Category X'),
    //         ('Spotify', 'Category Y'),
    //         ('youtube.com/watch?v=bS9em7Bg0iU', 'Category Z'),
    //         ('stackoverflow.com', 'Category X')",
    //     [],
    // )?;

    // conn.execute(
    //     "INSERT INTO log (start_time, end_time, activity_identifier, task_id) VALUES
    //         (3600, 7200, 'tpulse - Visual Studio Code', '1'),
    //         (7200, 10800, 'Spotify', NULL),
    //         (10800, 14400, 'youtube.com/watch?v=bS9em7Bg0iU', '2'),
    //         (14400, 16200, 'Spotify', NULL),
    //         (18000, NULL, 'tpulse - Visual Studio Code', '1')",
    //     [],
    // )?;

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
