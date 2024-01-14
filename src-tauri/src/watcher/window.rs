use log::{info, error};
use std::{thread::sleep, time::Duration};

use super::window_query::get_current_window_information;

pub fn watch_window(poll_time: u64) {
    info!("Window watcher started !");
    loop {
        sleep(Duration::from_millis(poll_time));
        let window_info = get_current_window_information();
        match window_info {
            std::result::Result::Ok(window_info) => {
                info!("{}", serde_json::to_string_pretty(&window_info).unwrap());
            }
            Err(e) => {
                error!("Window information error: {}", e);
            }
        }
    }
}
