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

pub fn insert_browser_activity(time: u64, activity: &BrowserActivity) {
    let mut conn = get_connection();
    let mut tx = conn.transaction().expect("Failed to start transaction");

    tx.execute(
        "INSERT INTO activity (id, type, category)
            VALUES (?1, 'browser', NULL)
            ON CONFLICT(id)
            DO UPDATE SET type = 'browser', category = ?2",
        params![&activity.id, &activity.category],
    )
    .unwrap();
    tx.execute(
        "INSERT INTO browser_activity (id, title, url)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(id)
            DO UPDATE SET title = ?2, url = ?3",
        params![&activity.id, &activity.title, &activity.url],
    )
    .unwrap();

    let latest_activity_id = get_latest_activity_id(&mut tx);
    if latest_activity_id != activity.id {
        tx.execute(
            "UPDATE log set end_time = ?1 WHERE id = (SELECT MAX(id) FROM log)
                 AND activity_id = ?2",
            params![time, &latest_activity_id],
        )
        .unwrap();
        tx.execute(
            "INSERT INTO log (activity_id, start_time, end_time)
                 VALUES (?1, ?2, NULL)",
            params![&activity.id, time],
        )
        .unwrap();
    } else {
        tx.execute(
            "UPDATE log set end_time = ?1 WHERE id = (SELECT MAX(id) FROM log)
                 AND activity_id = ?2",
            params![time, &activity.id],
        )
        .unwrap();
    }

    tx.commit().expect("Failed to commit transaction");
}

pub fn insert_window_activity(time: u64, activity: &WindowActivity) {
    let mut conn = get_connection();
    let mut tx = conn.transaction().expect("Failed to start transaction");

    tx.execute(
        "INSERT INTO activity (id, type, category)
            VALUES (?1, 'window', ?2)
            ON CONFLICT(id)
            DO UPDATE SET type = 'window', category = ?2",
        params![&activity.id, &activity.category],
    )
    .unwrap();
    tx.execute(
        "INSERT INTO window_activity (id, title, class, execute_binary)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(id)
            DO UPDATE SET title = ?2, class = ?3, execute_binary = ?4",
        params![
            &activity.id,
            &activity.title,
            &activity.class,
            &activity.execute_binary
        ],
    )
    .unwrap();
    
    let latest_activity_id = get_latest_activity_id(&mut tx);
    if latest_activity_id != activity.id {
        tx.execute(
            "UPDATE log set end_time = ?1 WHERE id = (SELECT MAX(id) FROM log)
                 AND activity_id = ?2",
            params![time, &latest_activity_id],
        )
        .unwrap();
        tx.execute(
            "INSERT INTO log (activity_id, start_time, end_time)
                 VALUES (?1, ?2, NULL)",
            params![&activity.id, time],
        )
        .unwrap();
    } else {
        tx.execute(
            "UPDATE log set end_time = ?1 WHERE id = (SELECT MAX(id) FROM log)
                 AND activity_id = ?2",
            params![time, &activity.id],
        )
        .unwrap();
    }

    tx.commit().expect("Failed to commit transaction");
}

fn get_latest_activity_id(tx: &mut Transaction) -> String {
    let mut get_latest_log = tx
        .prepare("SELECT activity_id FROM log ORDER BY id DESC LIMIT 1")
        .unwrap();
    let latest_activity_iter = get_latest_log.query_map(params![], |row| {
        let activity_id: Result<String, rusqlite::Error> = row.get(0);
        let id = match activity_id {
            Ok(activity_id) => activity_id,
            Err(e) => {
                log::error!("Failed to get latest activity id: {:?}", e);
                "".to_string()
            }
        };
        Ok(id)
    });
    let mut latest_activity_id = "".to_string();
    for id in latest_activity_iter.unwrap() {
        latest_activity_id = id.unwrap();
        break;
    }
    latest_activity_id
}