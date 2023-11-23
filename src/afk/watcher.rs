use chrono::Utc;
use device_query::{DeviceQuery, DeviceState};
use log::info;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone)]
pub struct Settings {
    /// In milisecs
    timeout: u64,
    /// In milisecs
    poll_time: u64,
}
impl Settings {
    pub fn new(timeout: u64, poll_time: u64) -> Self {
        Settings { timeout, poll_time }
    }
}

pub struct AFKWatcher {
    settings: Settings,
    bucketname: String,
}

enum AFKState {
    ONLINE,
    OFFLINE,
}

impl AFKWatcher {
    pub fn new(settings: &Settings) -> Self {
        AFKWatcher {
            settings: settings.clone(),
            bucketname: "AFKWatcher".to_string(), // TODO: Make this dynamic
        }
    }
    pub fn run(&self) {
        info!("AFK watcher started");
        let device_state = DeviceState::new();
        let mut mouse_pos = device_state.get_mouse().coords;

        let mut timeout = 0;
        let mut afk = false;
        loop {
            sleep(Duration::from_millis(self.settings.poll_time));

            let mut detect_interact = false;

            let current_mouse_pos = device_state.get_mouse().coords;
            if current_mouse_pos.0 != mouse_pos.0 && current_mouse_pos.1 != mouse_pos.1 {
                mouse_pos = current_mouse_pos;
                info!("Detect mouse position change {:?}", mouse_pos);
                detect_interact = true;
            } else {
                let keys = device_state.query_keymap();
                if keys.len() > 0 {
                    info!("Detected key {:?}", keys);
                    detect_interact = true;
                }
            }

            if detect_interact {
                timeout = 0;
                if afk {
                    afk = false;
                    self.send_metric(AFKState::ONLINE);
                }
            } else {
                timeout += self.settings.poll_time;
                if timeout >= self.settings.timeout && !afk {
                    afk = true;
                    self.send_metric(AFKState::OFFLINE);
                }
            }
        }
    }
    fn send_metric(&self, state: AFKState) {
        let now = Utc::now();
        match state {
            AFKState::ONLINE => {
                info!("{}: Reconnected at {}", self.bucketname, now.to_string());
            }
            AFKState::OFFLINE => {
                let afk_from = now - chrono::Duration::milliseconds(self.settings.timeout as i64);
                info!(
                    "{}: Noticed AFK from {} to {}",
                    self.bucketname,
                    afk_from.to_string(),
                    now.to_string()
                );
            }
        }
    }
}
