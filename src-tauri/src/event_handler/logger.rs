use crate::{
    raw_metric_processor::ProcessedResult,
    sqlite::{insert_new_log, update_log},
};

pub fn handle_events(events: Vec<ProcessedResult>) {
    for event in events {
        match event {
            ProcessedResult::StartActivity(start_event) => {
                insert_new_log(&start_event);
            }
            ProcessedResult::UpdateEndActivity(end_event) => {
                update_log(&end_event);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        initializer::db,
        raw_metric_processor::{ActivityTag, ProcessedResult, StartActivity, UpdateEndActivity},
        sqlite::{insert_new_log, update_log},
        utils::get_data_directory,
    };
    use lazy_static::lazy_static;
    use rusqlite::Connection;
    use std::{sync::mpsc, thread};

    #[test]
    fn test_handle_events() {
        db::initialize();

        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            let start_event = StartActivity {
                start_time: 682003,
                activity_identifier: "activity_id_1".to_string(),
                tag: ActivityTag::WINDOW,
            };
            let end_event = UpdateEndActivity {
                start_time: 682003,
                end_time: 12072003,
            };

            let events = vec![
                ProcessedResult::StartActivity(start_event),
                ProcessedResult::UpdateEndActivity(end_event),
            ];

            tx.send(events).unwrap();
        });

        let handle_events_thread = thread::spawn(move || loop {
            //custom body of handle_events
            loop {
                let events = rx.recv().unwrap();
                for event in events {
                    match event {
                        ProcessedResult::StartActivity(start_event) => {
                            insert_new_log(&start_event);
                        }
                        ProcessedResult::UpdateEndActivity(end_event) => {
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
            .query_row("SELECT * FROM log WHERE start_time = 682003", [], |row| {
                let start_time: usize = row.get(0)?;
                let end_time: Option<usize> = row.get(1)?;

                Ok((start_time, end_time))
            })
            .unwrap();

        assert_eq!(log_entry, (682003, Some(12072003)));
    }
}
