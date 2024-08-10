use crate::{config, metric::schema::Activity};
use log::info;
use std::{sync::mpsc, thread::sleep, time::Duration};

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::get_current_window_information;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::get_current_window_information;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::get_current_window_information;

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
        if let Some(window_info) = get_current_window_information() {
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
