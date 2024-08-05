use lazy_static::lazy_static;
use rusqlite::{params, Connection};
use std::sync::{Mutex, MutexGuard};

static mut DB_PATH: Option<String> = None;

lazy_static! {
    static ref CONNECTION: Mutex<Connection> = {
        let db_path = unsafe { DB_PATH.as_ref() }.expect("DB path not set");
        Mutex::new(Connection::open(&db_path).expect("Failed to open database connection"))
    };
}

pub fn set_path(db_path: &str) {
    unsafe {
        DB_PATH = Some(db_path.to_string());
    }
}

pub fn get_connection() -> MutexGuard<'static, Connection> {
    CONNECTION.lock().expect("Failed to lock connection")
}

pub struct WindowActivity {
    pub id: String,
    pub title: String,
    pub class: String,
    pub execute_binary: Option<String>,
    pub category: Option<String>,
}

pub fn insert_window_metric(time: u64, metric: &WindowActivity) {
    let mut conn = get_connection();
    let tx = conn.transaction().expect("Failed to start transaction");

    let _ = tx.execute(
        "INSERT INTO activity (id, type, category)
            VALUES (?1, 'window', ?2)
            ON CONFLICT(id)
            DO UPDATE SET type = 'window', category = ?2",
        params![&metric.id, &metric.category],
    );
    let _ = tx.execute(
        "INSERT INTO window_activity (id, title, class, execute_binary)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(id)
            DO UPDATE SET title = ?2, class = ?3, execute_binary = ?4",
        params![
            &metric.id,
            &metric.title,
            &metric.class,
            &metric.execute_binary
        ],
    );
    let _ = tx.execute(
        "INSERT INTO log (task_id, activity_id, start_time, end_time)
            VALUES (NULL, ?1, ?2, NULL)",
        params![&metric.id, time],
    );

    // let _ = tx.execute(
    //     "UPDATE log set end_time = ?1 
    //     WHERE 
    //         id = (SELECT MAX(id) FROM log)
    //         AND activity_id = ?2",
    //     ",
    //     params![time],
    // );

    // // UPDATE table set col = 1 WHERE id = (SELECT MAX(id) FROM table)
  
    tx.commit().expect("Failed to commit transaction");
}
