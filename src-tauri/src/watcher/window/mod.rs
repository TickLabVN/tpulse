mod window_query;

use log::{error, info};
use std::{sync::mpsc, thread::sleep, time::Duration};

use crate::{metrics::UserMetric, watcher::window::window_query::get_current_window_information};

/// Watches the current window and sends window information through a channel.
///
/// This function continuously polls for window information at a specified interval (`poll_time`)
/// and sends the information through the provided `tx` channel. The window information is obtained
/// using the `get_current_window_information` function.
///
/// # Arguments
///
/// * `poll_time` - The interval in milliseconds at which to poll for window information.
/// * `tx` - The channel sender to send the window information through.
pub fn watch_window(poll_time: u64, tx: mpsc::Sender<UserMetric>) {
    info!("Window watcher started!");
    loop {
        sleep(Duration::from_millis(poll_time));

        // If there is an active window
        if let Some(window_info_result) = get_current_window_information() {
            match window_info_result {
                Ok(window_info) => {
                    tx.send(UserMetric::Window(window_info))
                        .expect("Failed to send window information");
                }
                Err(e) => {
                    error!("Window information error: {}", e);
                }
            }
        } else {
            // No active window, continue looping
            continue;
        }
    }
}
