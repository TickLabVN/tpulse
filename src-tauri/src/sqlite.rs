use crate::{
    config,
    raw_metric_processor::{StartActivity, UpdateEndActivity},
    utils::get_data_directory,
};
use lazy_static::lazy_static;
use rusqlite::{params, Connection, OptionalExtension};

lazy_static! {
    static ref DB_PATH: String = format!("{}/tpulse.sqlite3", get_data_directory());
}

pub fn insert_new_log(start_log_event: &StartActivity) {
    let pool_time = config::get_setting().poll_time;

    let conn = Connection::open(&*DB_PATH).expect("Failed to open database connection");

    let start_time = &start_log_event.start_time;
    let activity_id = &start_log_event.activity_identifier;
    let activity_tag: &String = &start_log_event.tag.to_string();

    let activity_exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM activity WHERE identifier = ?1)",
            params![activity_id],
            |row| row.get(0),
        )
        .unwrap_or(false);

    if !activity_exists {
        conn.execute(
            "INSERT INTO activity (identifier, activity_tag) VALUES (?1, ?2)",
            params![activity_id, activity_tag],
        )
        .expect("Failed to create new activity");
    }

    let newest_log_of_activity: Option<(u64,Option<u64>)> = conn
        .query_row(
            "SELECT start_time, end_time FROM log WHERE activity_identifier = ?1 ORDER BY start_time DESC LIMIT 1",
            params![activity_id],
            |row| {Ok((row.get(0)?, row.get(1)?))},
        ).optional().unwrap();

    if newest_log_of_activity.is_none() {
        conn.execute(
            "INSERT INTO log (start_time, end_time, activity_identifier) VALUES (?1, ?2, ?3)",
            params![start_time, start_time, activity_id],
        )
        .expect("Failed to insert new log");
        return;
    }

    let (start_time_of_previous_log, end_time_of_previous_log) = newest_log_of_activity.unwrap();

    end_time_of_previous_log.map(|end_time| {
        if end_time + pool_time >= *start_time {
            conn.execute(
                "UPDATE log SET end_time = ?1 WHERE start_time = ?2 AND activity_identifier = ?3",
                params![start_time, start_time_of_previous_log, activity_id],
            )
            .expect("Failed to update log");
        }
    });
}

pub fn update_log(end_log_event: &UpdateEndActivity) {
    let conn = Connection::open(&*DB_PATH).expect("Failed to open database connection");

    let start_time_string = &end_log_event.start_time;
    let end_time_string = &end_log_event.end_time;

    let log_exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM log WHERE start_time = ?1)",
            params![start_time_string],
            |row| row.get(0),
        )
        .unwrap_or(false);

    if !log_exists {
        eprintln!("Log entry with start time {} not found", start_time_string);
        return;
    }

    conn.execute(
        "UPDATE log SET end_time = ?1 WHERE start_time = ?2",
        params![end_time_string, start_time_string],
    )
    .expect("Failed to update log");
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use rusqlite::params;

    use crate::{
        config,
        initializer::db,
        raw_metric_processor::{ActivityTag, StartActivity, UpdateEndActivity},
        sqlite::{insert_new_log, update_log},
        utils::get_data_directory,
    };
    use lazy_static::lazy_static;
    use rusqlite::Connection;

    #[test]
    fn test_insert_single_new_log() {
        db::initialize();
        lazy_static! {
            static ref DB_PATH: String = format!("{}/tpulse.sqlite3", get_data_directory());
        }

        let conn = Connection::open(&*DB_PATH).expect("Failed to open database connection");

        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let start_activity = StartActivity {
            start_time,
            activity_identifier: "test_activity".to_string(),
            tag: ActivityTag::BROWSER,
        };

        insert_new_log(&start_activity);

        let activity_exists: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM activity WHERE identifier = ?1)",
                params![&start_activity.activity_identifier],
                |row| row.get(0),
            )
            .unwrap();

        assert!(activity_exists, "Activity should be inserted");

        let log_exists: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM log WHERE activity_identifier = ?1)",
                params![&start_activity.activity_identifier],
                |row| row.get(0),
            )
            .unwrap();

        assert!(log_exists, "Log should be inserted");
    }

    #[test]
    fn test_insert_multiple_logs() {
        db::initialize();
        lazy_static! {
            static ref DB_PATH: String = format!("{}/tpulse.sqlite3", get_data_directory());
        }

        let conn = Connection::open(&*DB_PATH).expect("Failed to open database connection");

        let pool_time = config::get_setting().poll_time;

        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let activity_identifier = "test_activity".to_string();

        let activities = vec![
            StartActivity {
                start_time,
                activity_identifier: activity_identifier.clone(),
                tag: ActivityTag::BROWSER,
            },
            StartActivity {
                start_time: start_time + pool_time,
                activity_identifier: activity_identifier.clone(),
                tag: ActivityTag::BROWSER,
            },
            StartActivity {
                start_time: start_time + 2 * pool_time,
                activity_identifier: activity_identifier.clone(),
                tag: ActivityTag::BROWSER,
            },
        ];

        for activity in &activities {
            insert_new_log(activity);
        }

        let log_entry = conn
            .query_row(
                "SELECT start_time, end_time FROM log WHERE start_time = ?1 AND activity_identifier = ?2",
                params![start_time, activity_identifier],
                |row| {
                    let start_time: u64 = row.get(0)?;
                    let end_time: Option<u64> = row.get(1)?;

                    Ok((start_time, end_time))
                },
            )
            .unwrap();

        assert_eq!(
            log_entry,
            (start_time, Some(activities.last().unwrap().start_time))
        );
    }

    #[test]
    fn test_insert_end_logs() {
        db::initialize();
        lazy_static! {
            static ref DB_PATH: String = format!("{}/tpulse.sqlite3", get_data_directory());
        }

        let conn = Connection::open(&*DB_PATH).expect("Failed to open database connection");

        let pool_time = config::get_setting().poll_time;

        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let activity_identifier = "test_activity".to_string();

        let activities = vec![
            StartActivity {
                start_time,
                activity_identifier: activity_identifier.clone(),
                tag: ActivityTag::BROWSER,
            },
            StartActivity {
                start_time: start_time + pool_time,
                activity_identifier: activity_identifier.clone(),
                tag: ActivityTag::BROWSER,
            },
            StartActivity {
                start_time: start_time + 2 * pool_time,
                activity_identifier: activity_identifier.clone(),
                tag: ActivityTag::BROWSER,
            },
        ];

        let end_time = start_time + (((activities.len() + 3) as u64) * pool_time);

        let end_activity = UpdateEndActivity {
            start_time,
            end_time,
        };

        for activity in &activities {
            insert_new_log(activity);
        }

        update_log(&end_activity);

        let log_entry = conn
            .query_row(
                "SELECT start_time, end_time FROM log WHERE start_time = ?1 AND activity_identifier = ?2",
                params![start_time, activity_identifier],
                |row| {
                    let start_time: u64 = row.get(0)?;
                    let end_time: Option<u64> = row.get(1)?;

                    Ok((start_time, end_time))
                },
            )
            .unwrap();

        assert_eq!(log_entry, (start_time, Some(end_time)));
    }
}
