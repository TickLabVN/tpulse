use chrono::Utc;
use device_query::{DeviceQuery, DeviceState};
use log::info;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone)]
pub struct AFKSettings {
    /// In milisecs
    timeout: u64,
    /// In milisecs
    poll_time: u64,
}
impl AFKSettings {
    pub fn new(timeout: u64, poll_time: u64) -> Self {
        AFKSettings { timeout, poll_time }
    }
}

pub struct AFKWatcher {
    settings: AFKSettings,
}

enum AFKState {
    ONLINE,
    OFFLINE,
}

impl AFKWatcher {
    pub fn new(settings: &AFKSettings) -> Self {
        AFKWatcher {
            settings: settings.clone(),
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
            if current_mouse_pos.0 != mouse_pos.0 || current_mouse_pos.1 != mouse_pos.1 {
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
        let base_url = dotenv::var("SERVER_URL").unwrap();
        let url = base_url + "/api/members/1234567891234567891234567/worklogs/afk";

        let res = match state {
            AFKState::ONLINE => {
                let body = format!("{{\"type\":\"{}\",\"time\":{}}}", "ONLINE", now.timestamp());
                ureq::post(&url)
                    .set("Content-Type", "application/json")
                    .send_string(&body)
            }
            AFKState::OFFLINE => {
                let afk_from = now - chrono::Duration::milliseconds(self.settings.timeout as i64);
                let body = format!(
                    "{{\"type\":\"{}\",\"time\":{}}}",
                    "OFFLINE",
                    afk_from.timestamp()
                );

                ureq::post(&url)
                    .set("Content-Type", "application/json")
                    .send_string(&body)
            }
        };

        match res {
            Ok(_) => {
                info!("{}", "Metric sent");
            }
            Err(e) => {
                info!("Error sending metric: {}", e);
                return;
            }
        }
    }
}
