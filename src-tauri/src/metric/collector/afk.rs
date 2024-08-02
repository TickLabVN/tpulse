use crate::config::get_setting;
use crate::metric::schema::{AFKMetric, AFKStatus, Activity};
use device_query::{DeviceQuery, DeviceState};
use log::info;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

/// Watches for user's AFK (Away From Keyboard) state.
///
/// This function continuously monitors user activity by tracking mouse movements and keyboard inputs.
/// If there is no activity for a specified duration, the user is considered AFK.
///
/// # Arguments
///
/// * `poll_time` - The time interval (in milliseconds) between each check for user activity.
/// * `timeout` - The duration (in milliseconds) of inactivity after which the user is considered AFK.
///
/// # Examples
///
/// ```
/// // Watch for AFK state with a poll time of 1000ms and a timeout of 5000ms
///
/// use tpulse::watcher::watch_afk;
/// watch_afk(1000, 5000);
/// ```
pub fn watch_afk(tx: mpsc::Sender<Activity>) {
    info!("AFK watcher started");
    let device_state = DeviceState::new();
    let mut mouse_pos = device_state.get_mouse().coords;
    let mut total_timeout = 0;
    let mut afk = false;
    loop {
        let setting = get_setting();

        sleep(Duration::from_millis(setting.poll_time));
        let mut detect_interact = false;

        let current_mouse_pos = device_state.get_mouse().coords;
        if current_mouse_pos.0 != mouse_pos.0 || current_mouse_pos.1 != mouse_pos.1 {
            mouse_pos = current_mouse_pos;
            detect_interact = true;
        } else {
            let keys = device_state.query_keymap();
            if keys.len() > 0 {
                detect_interact = true;
            }
        }

        if detect_interact {
            total_timeout = 0;
            if afk {
                afk = false;
                // send metric online
                let unix_ts = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap();

                tx.send(Activity::AFK(AFKMetric {
                    start_time: unix_ts.as_secs() as u64,
                    status: AFKStatus::ONLINE,
                }))
                .unwrap();
            }
        } else {
            total_timeout += setting.poll_time;
            if total_timeout >= setting.time_out && !afk {
                afk = true;
                // send metric offline
                let unix_ts = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap();
                tx.send(Activity::AFK(AFKMetric {
                    start_time: unix_ts.as_secs() as u64,
                    status: AFKStatus::OFFLINE,
                }))
                .unwrap();
            }
        }
    }
}
