mod window_query;
use crate::{config, metric::schema::Activity};
use log::{error, info};
use std::{sync::mpsc, thread::sleep, time::Duration};
use window_query::get_current_window_information;

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
pub fn watch_window(tx: mpsc::Sender<Activity>) {
    const BROWSERS: [&str; 2] = ["firefox", "google-chrome"];
    info!("Window watcher started!");
    loop {
        // If there is an active window
        if let Some(r) = get_current_window_information() {
            if r.is_err() {
                error!("Window information error: {:?}", r);
            }

            let window_info = r.unwrap();
            // Check if the window is a browser
            let mut is_browser = false;
            for browser in BROWSERS {
                for class in &window_info.class {
                    if class.contains(browser) {
                        is_browser = true;
                        break;
                    }
                }
                if is_browser {
                    break;
                }
            }

            if !is_browser {
                tx.send(Activity::Window(window_info))
                    .expect("Failed to send window information");
            }
        }
        let poll_time = config::get_setting().poll_time;
        sleep(Duration::from_millis(poll_time));
    }
}
