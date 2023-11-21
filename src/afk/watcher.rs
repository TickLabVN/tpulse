use device_query::{DeviceQuery, DeviceState};
use log::info;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone)]
pub struct Settings {
    timeout: u64,
    poll_time: u64,
}
impl Settings {
    pub fn new(timeout: u64, poll_time: u64) -> Self {
        assert!(
            timeout >= poll_time,
            "Timeout should be greater than or equal to poll time"
        );

        Settings { timeout, poll_time }
    }
}

pub struct AFKWatcher {
    settings: Settings,
    bucketname: String,
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
        self.watch();
    }
    fn watch(&self) {
        // let mut afk = false;
        let device_state = DeviceState::new();
        loop {
            sleep(Duration::from_secs(self.settings.poll_time));

            let keys = device_state.get_keys();
            info!("Detected key {:?}", keys.len() > 0);
            let mouse = device_state.get_mouse();
            info!("Current mouse position {:?}", mouse);
        }
    }
}
