use log::info;

use crate::{
    event_handler::categorizer::get_activity_category_tag,
    raw_metric_processor::{ProcessedResult, StartActivity},
    sqlite::{insert_new_log, update_log},
};

use super::categorizer::Category;

pub struct ActivityStartLog {
    pub start_log: StartActivity,
    pub category_tag: Option<Category>,
}

pub fn handle_events(events: Vec<ProcessedResult>) {
    for event in events {
        info!("{:?}", event);
        let category_tag = get_activity_category_tag(event.clone());
        match event {
            ProcessedResult::StartActivity(start_event) => {
                insert_new_log(ActivityStartLog {
                    start_log: start_event,
                    category_tag,
                });
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
        event_handler::logger::handle_events,
        initializer::db,
        raw_metric_processor::{ActivityTag, ProcessedResult, StartActivity, UpdateEndActivity},
        utils::get_data_directory,
    };
    use lazy_static::lazy_static;
    use rusqlite::Connection;
    use std::{sync::mpsc, thread};

    #[test]
    fn test_handle_events() {
        db::initialize();

        let (tx, rx) = mpsc::channel();
        let activity_identifier = "github.com".to_string();
        let activity_identifier_insert = activity_identifier.clone();

        std::thread::spawn(move || {
            let start_event = StartActivity {
                start_time: 682003,
                tag: ActivityTag::BROWSER,
                activity_identifier: activity_identifier_insert,
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
            loop {
                let events = rx.recv().unwrap();
                handle_events(events);
                return;
            }
        });

        handle_events_thread
            .join()
            .expect("Failed to join handle thread");

        lazy_static! {
            static ref DB_PATH: String = format!("{}/tpulse.sqlite3", get_data_directory());
        }

        let conn = Connection::open(&*DB_PATH).expect("Failed to open database connection");

        let activity_entry = conn
            .query_row(
                "SELECT * FROM activity WHERE identifier = ?1",
                [activity_identifier.clone()],
                |row| {
                    let identifier: String = row.get(0)?;
                    let activity_tag: Option<String> = row.get(1)?;
                    let category_tag: Option<String> = row.get(2)?;

                    Ok((identifier, activity_tag, category_tag))
                },
            )
            .unwrap();

        debug_assert_eq!(
            activity_entry,
            (
                activity_identifier.clone(),
                Some("browser".to_string()),
                Some("Code".to_string()),
            )
        );

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
