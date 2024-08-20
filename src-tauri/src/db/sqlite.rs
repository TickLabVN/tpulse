use lazy_static::lazy_static;
use rusqlite::{params, Connection, Transaction};
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

pub fn get_latest_activity_log(tx: &mut Transaction) -> Option<(u64, Option<u64>, String)> {
    let mut get_latest_log = tx
        .prepare("SELECT id, end_time, activity_id FROM log ORDER BY id DESC LIMIT 1")
        .unwrap();

    let mut log_id: u64 = 0;
    let mut log_end_time: Option<u64> = None;
    let mut log_activity_id: String = String::new();

    let _ = get_latest_log
        .query_map(params![], |row| {
            let id: Result<u64, rusqlite::Error> = row.get(0);
            let end_time: Result<Option<u64>, rusqlite::Error> = row.get(1);
            let activity_id: Result<String, rusqlite::Error> = row.get(2);

            match (id, end_time, activity_id) {
                (Ok(id), Ok(end_time), Ok(activity_id)) => {
                    log_id = id;
                    log_end_time = end_time;
                    log_activity_id = activity_id;
                }
                _ => {}
            }
            Ok(())
        })
        .unwrap()
        .next();

    if log_id > 0 {
        Some((log_id, log_end_time, log_activity_id))
    } else {
        // There are no records in the log table
        None
    }
}
