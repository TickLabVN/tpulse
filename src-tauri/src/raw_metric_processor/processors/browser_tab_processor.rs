use crate::metrics::{BrowserInformation, BrowserMetric, BrowserMetricType, UserMetric};
use crate::utils::get_data_directory;
use rusqlite::Connection;
use serde_json;
use std::error::Error;
use tpulse::initializer::insert_parsed_data_into_db;

pub fn handle_process_browser_data(data: &str) -> Result<(), Box<dyn Error>> {
    let browser_info: BrowserInformation = serde_json::from_str(data)?;
    let browser_metric = BrowserMetric {
        data_type: BrowserMetricType::BrowserTab,
        title: browser_info.title.unwrap(),
        url: None,
        window_id: None,
        start_time: browser_info.start_time.parse::<u64>()?,
        tab_id: None,
        paused: None,
    };
    let user_metric = UserMetric::Browser(browser_metric);
    let conn = Connection::open(&format!("{}/tpulse.sqlite3", get_data_directory()))?;
    insert_parsed_data_into_db(conn, &user_metric)?;
    Ok(())
}
