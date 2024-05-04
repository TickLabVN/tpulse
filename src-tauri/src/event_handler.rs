use std::sync::mpsc::Receiver;

use crate::{
    models::LogUpdate,
    sqlite::{insert_new_log, update_log},
};

pub fn handle_events(rx: Receiver<Vec<LogUpdate>>) {
    loop {
        let events = rx.recv().unwrap();
        for event in events {
            match event {
                LogUpdate::LogStart(start_event) => {
                    insert_new_log(&start_event);
                }
                LogUpdate::LogEnd(end_event) => {
                    update_log(&end_event);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        initializer::initialize_db,
        models::{LogEndEvent, LogStartEvent, LogUpdate},
        sqlite::{insert_new_log, update_log},
        utils::get_data_directory,
    };
    use lazy_static::lazy_static;
    use rusqlite::Connection;
    use std::{sync::mpsc, thread};

    #[test]
    fn test_handle_events() {
        initialize_db();

        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            let start_event = LogStartEvent {
                start_time: chrono::NaiveDateTime::parse_from_str(
                    "2024-05-05T10:00:00",
                    "%Y-%m-%dT%H:%M:%S",
                )
                .unwrap()
                .to_string(),
                activity_identifier: "activity_id_1".to_string(),
            };
            let end_event = LogEndEvent {
                start_time: chrono::NaiveDateTime::parse_from_str(
                    "2024-05-05T10:00:00",
                    "%Y-%m-%dT%H:%M:%S",
                )
                .unwrap()
                .to_string(),
                end_time: chrono::NaiveDateTime::parse_from_str(
                    "2024-05-05T10:30:00",
                    "%Y-%m-%dT%H:%M:%S",
                )
                .unwrap()
                .to_string(),
            };

            let events = vec![
                LogUpdate::LogStart(start_event),
                LogUpdate::LogEnd(end_event),
            ];

            tx.send(events).unwrap();
        });

        let handle_events_thread = thread::spawn(move || loop {
            //custom body of handle_events
            loop {
                let events = rx.recv().unwrap();
                for event in events {
                    match event {
                        LogUpdate::LogStart(start_event) => {
                            insert_new_log(&start_event);
                        }
                        LogUpdate::LogEnd(end_event) => {
                            update_log(&end_event);
                            return;
                        }
                    }
                }
            }
        });

        handle_events_thread
            .join()
            .expect("Failed to join handle thread");

        lazy_static! {
            static ref DB_PATH: String = format!("{}/tpulse.sqlite3", get_data_directory());
        }

        let conn = Connection::open(&*DB_PATH).expect("Failed to open database connection");

        let log_entry = conn
            .query_row(
                "SELECT * FROM log WHERE start_time = '2024-05-05 10:00:00'",
                [],
                |row| {
                    let start_time_string: String = row.get(0)?;
                    let end_time_string: Option<String> = row.get(1)?;

                    // Parse string representations into NaiveDateTime
                    let start_time = chrono::NaiveDateTime::parse_from_str(
                        &start_time_string,
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap();
                    let end_time = match end_time_string {
                        Some(end_time_string) => Some(
                            chrono::NaiveDateTime::parse_from_str(
                                &end_time_string,
                                "%Y-%m-%d %H:%M:%S",
                            )
                            .unwrap(),
                        ),
                        None => None,
                    };

                    Ok((start_time, end_time))
                },
            )
            .unwrap();

        debug_assert_eq!(
            log_entry,
            (
                chrono::NaiveDateTime::parse_from_str("2024-05-05T10:00:00", "%Y-%m-%dT%H:%M:%S")
                    .unwrap(),
                Some(
                    chrono::NaiveDateTime::parse_from_str(
                        "2024-05-05T10:30:00",
                        "%Y-%m-%dT%H:%M:%S"
                    )
                    .unwrap()
                )
            )
        );
    }
}
