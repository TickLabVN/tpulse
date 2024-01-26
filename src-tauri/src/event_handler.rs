use std::fs::File;
use std::io::Write;
use std::sync::mpsc::Receiver;

use lazy_static::lazy_static;
use rusqlite::Connection;

use crate::{
    categorizer::InvertedIndex,
    events::UserMetric,
    sqlite::{insert_afk_log, insert_window_log},
    utils::get_data_directory,
};

lazy_static! {
    static ref DB_PATH: String = format!("{}/tpulse.sqlite3", get_data_directory());
}

pub fn handle_events(rx: Receiver<UserMetric>) {
    loop {
        let metric = rx.recv().unwrap();
        match metric {
            UserMetric::AFK(afk_event) => {
                insert_afk_log(&afk_event);
            }
            UserMetric::Window(window_event) => {
                insert_window_log(&window_event);
            }
        }
    }
}

pub fn categorize_events(
    rx: Receiver<UserMetric>,
    categorizer: InvertedIndex,
    db: &Connection,
    table_name: &str,
) {
    let mut output = File::create("category_result.txt").unwrap();
    loop {
        let metric = rx.recv().unwrap();
        match metric {
            UserMetric::AFK(_) => {}
            UserMetric::Window(window_event) => {
                writeln!(output, "{:?}", window_event.title.as_ref().unwrap()).unwrap();

                let indexes = categorizer.categorize(&window_event.title.unwrap());
                let mut statement = db
                    .prepare(&format!(
                        "select c_name from {} where c_id in ({})",
                        table_name,
                        indexes
                            .iter()
                            .map(|index| index.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    ))
                    .unwrap();

                let list_app_name = statement.query_map([], |row| Ok(row.get(0)?)).unwrap();
                let result = list_app_name
                    .map(|item| item.unwrap())
                    .collect::<Vec<String>>();
                writeln!(output, "{:?}", result).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod event_handler_tests {
    use std::sync::mpsc::{self, Sender};
    use std::thread;

    use rusqlite::Connection;

    use crate::categorizer::{load_table_from_path, Document};
    use crate::watcher::watch_window;

    use super::*;

    #[test]
    fn test_categorize_window_event() {
        let mut conn = Connection::open_in_memory().unwrap();
        let _ = load_table_from_path(
            &mut conn,
            "t",
            "Tracking_Rule_Package_Default_Export.csv",
            b',',
        );
        let documents = Document::build(&conn, "t").unwrap();

        let mut categorizer = InvertedIndex::default();
        categorizer.generate_token_index(&documents);

        let (tx, rx): (Sender<UserMetric>, Receiver<UserMetric>) = mpsc::channel();
        let window_watcher = thread::spawn(move || watch_window(1000, tx));
        let event_categorizer =
            thread::spawn(move || categorize_events(rx, categorizer, &conn, "t"));
        window_watcher.join().unwrap();
        event_categorizer.join().unwrap();
    }
}
