mod window_query;
use crate::{
    collector::window::window_query::get_current_window_information, config, metrics::UserMetric,
};
use log::info;
use std::{sync::mpsc, thread::sleep, time::Duration};

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
pub fn watch_window(tx: mpsc::Sender<UserMetric>) {
    info!("Window watcher started!");
    loop {
        // If there is an active window
        let window_info_result = get_current_window_information();
        match window_info_result {
            Some(Ok(window_info)) => {
                tx.send(UserMetric::Window(window_info))
                    .expect("Failed to send window information");
            }
            Some(Err(e)) => {
                eprintln!("Window information error: {}", e);
            }
            // No active window
            None => {}
        }
        let poll_time = config::get_setting().poll_time;
        sleep(Duration::from_millis(poll_time));
    }
}
