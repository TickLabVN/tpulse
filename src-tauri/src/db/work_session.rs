use crate::config::get_setting;

use super::get_connection;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum AFKStatus {
    ONLINE = 1,
    OFFLINE = 0,
}

struct WorkSession {
    id: u64,
    start_time: u64,
    end_time: Option<u64>,
    status: String,
}

pub fn update_work_session(time: u64, status: AFKStatus) {
    let conn = get_connection();

    let current_session = get_current_session(&conn);
    if current_session.is_none() {
        if status == AFKStatus::ONLINE {
            conn.execute(
                "INSERT INTO work_session (start_time, end_time, status)
                 VALUES (?1, NULL, 'open')",
                params![time],
            )
            .unwrap();
        }
        return;
    }

    let config = get_setting();

    let current_session = current_session.unwrap();
    if current_session.status == "close" {
        if status == AFKStatus::ONLINE {
            conn.execute(
                "INSERT INTO work_session (start_time, end_time, status)
                 VALUES (?1, NULL, 'open')",
                params![time],
            )
            .unwrap();
        }
        return;
    }

    let prev_time = current_session
        .end_time
        .unwrap_or(current_session.start_time);
    let is_offline = status == AFKStatus::OFFLINE || prev_time + config.time_out < time;

    if !is_offline {
        conn.execute(
            "UPDATE work_session SET end_time = ?1, status = 'open' WHERE id = ?2",
            params![time, current_session.id],
        )
        .unwrap();
    } else {
        if current_session.end_time.is_none() {
            conn.execute(
                "DELETE FROM work_session WHERE id = ?1",
                params![current_session.id],
            )
            .unwrap();
        } else {
            conn.execute(
                "UPDATE work_session SET status = 'close' WHERE id = ?1",
                params![current_session.id],
            )
            .unwrap();
        }

        conn.execute(
            "INSERT INTO work_session (start_time, end_time, status)
             VALUES (?1, NULL, 'open')",
            params![time],
        ).unwrap();
    }
}

fn get_current_session(conn: &Connection) -> Option<WorkSession> {
    let mut stmt = conn
        .prepare(
            "SELECT id, start_time, end_time, status FROM work_session ORDER BY id DESC LIMIT 1",
        )
        .unwrap();

    let iter = stmt
        .query_map(params![], |row| {
            let sid: u64 = row.get(0).unwrap();
            let start: u64 = row.get(1).unwrap();
            let end: Option<u64> = row.get(2).unwrap();
            let status: String = row.get(3).unwrap();
            Ok(WorkSession {
                id: sid,
                start_time: start,
                end_time: end,
                status,
            })
        })
        .unwrap();
    for session in iter {
        return Some(session.unwrap());
    }
    None
}
