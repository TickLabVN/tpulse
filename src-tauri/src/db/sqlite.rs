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

pub struct WindowActivity {
    pub id: String,
    pub title: String,
    pub class: String,
    pub execute_binary: Option<String>,
    pub category: Option<String>,
}

pub struct BrowserActivity {
    pub id: String,
    pub title: String,
    pub url: String,
    pub category: Option<String>,
}

pub fn insert_browser_activity(time: u64, new_activity: &BrowserActivity) {
    let mut conn = get_connection();
    let mut tx = conn.transaction().expect("Failed to start transaction");

    tx.execute(
        "INSERT INTO activity (id, type, category)
            VALUES (?1, 'browser', NULL)
            ON CONFLICT(id)
            DO UPDATE SET type = 'browser', category = ?2",
        params![&new_activity.id, &new_activity.category],
    )
    .unwrap();
    tx.execute(
        "INSERT INTO browser_activity (id, title, url)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(id)
            DO UPDATE SET title = ?2, url = ?3",
        params![&new_activity.id, &new_activity.title, &new_activity.url],
    )
    .unwrap();

    if let Some((log_id, log_end_time, log_activity_id)) = get_latest_activity_log(&mut tx) {
        if log_activity_id != new_activity.id {
            let old_log_stale = log_end_time.is_none();
            if old_log_stale {
                // Delete stale log
                tx.execute("DELETE FROM log WHERE id = ?1", params![log_id])
                    .unwrap();
            } else {
                tx.execute(
                    "UPDATE log set end_time = ?1 WHERE id = ?2",
                    params![time, log_id],
                )
                .unwrap();
            }
            tx.execute(
                "INSERT INTO log (activity_id, start_time, end_time)
                     VALUES (?1, ?2, NULL)",
                params![&new_activity.id, time],
            )
            .unwrap();
        } else {
            tx.execute(
                "UPDATE log set end_time = ?1 WHERE id = ?2",
                params![time, log_id],
            )
            .unwrap();
        }
    } else {
        tx.execute(
            "INSERT INTO log (activity_id, start_time, end_time)
                 VALUES (?1, ?2, NULL)",
            params![&new_activity.id, time],
        )
        .unwrap();
    }

    tx.commit().expect("Failed to commit transaction");
}

pub fn insert_window_activity(time: u64, new_activity: &WindowActivity) {
    let mut conn = get_connection();
    let mut tx = conn.transaction().expect("Failed to start transaction");

    tx.execute(
        "INSERT INTO activity (id, type, category)
            VALUES (?1, 'window', ?2)
            ON CONFLICT(id)
            DO UPDATE SET type = 'window', category = ?2",
        params![&new_activity.id, &new_activity.category],
    )
    .unwrap();
    tx.execute(
        "INSERT INTO window_activity (id, title, class, execute_binary)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(id)
            DO UPDATE SET title = ?2, class = ?3, execute_binary = ?4",
        params![
            &new_activity.id,
            &new_activity.title,
            &new_activity.class,
            &new_activity.execute_binary
        ],
    )
    .unwrap();

    if let Some((log_id, log_end_time, log_activity_id)) = get_latest_activity_log(&mut tx) {
        if log_activity_id != new_activity.id {
            let old_log_stale = log_end_time.is_none();
            if old_log_stale {
                // Delete stale log
                tx.execute("DELETE FROM log WHERE id = ?1", params![log_id])
                    .unwrap();
            } else {
                tx.execute(
                    "UPDATE log set end_time = ?1 WHERE id = ?2",
                    params![time, log_id],
                )
                .unwrap();
            }
            tx.execute(
                "INSERT INTO log (activity_id, start_time, end_time)
                     VALUES (?1, ?2, NULL)",
                params![&new_activity.id, time],
            )
            .unwrap();
        } else {
            tx.execute(
                "UPDATE log set end_time = ?1 WHERE id = ?2",
                params![time, log_id],
            )
            .unwrap();
        }
    } else {
        tx.execute(
            "INSERT INTO log (activity_id, start_time, end_time)
                 VALUES (?1, ?2, NULL)",
            params![&new_activity.id, time],
        )
        .unwrap();
    }

    tx.commit().expect("Failed to commit transaction");
}

fn get_latest_activity_log(tx: &mut Transaction) -> Option<(u64, Option<u64>, String)> {
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
        .unwrap().next();

    if log_id > 0 {
        Some((log_id, log_end_time, log_activity_id))
    } else {
        // There are no records in the log table
        None
    }
}
