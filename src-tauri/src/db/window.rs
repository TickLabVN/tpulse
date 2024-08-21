use rusqlite::params;
use super::{get_connection, get_latest_activity_log};

pub struct WindowActivity {
    pub id: String,
    pub title: String,
    pub class: String,
    pub execute_binary: Option<String>,
    pub category: Option<String>,
}

// TODO: Explain the logic of this function
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
