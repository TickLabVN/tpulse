use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, sync::{Mutex, MutexGuard}};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoogleSetting {
    #[serde(rename = "accessToken")]
    pub access_token: String,

    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Setting {
    #[serde(rename = "pollTime")]
    pub poll_time: u64,

    #[serde(rename = "timeOut")]
    pub time_out: u64,

    pub google: Option<GoogleSetting>,
}

lazy_static! {
    pub static ref SETTING: Mutex<Setting> = {
        let file = File::open("setting.json");
        match file {
            Ok(f) => {
                let reader = std::io::BufReader::new(f);
                let setting: Setting = serde_json::from_reader(reader).unwrap();
                Mutex::new(setting)
            }
            Err(_) => {
                let s = Setting {
                    poll_time: 1000,
                    time_out: 10000,
                    google: None,
                };
                Mutex::new(s)
            }
        }
    };
}

pub fn get_setting() -> MutexGuard<'static, Setting> {
    SETTING.lock().unwrap()
}

pub fn save_setting(setting: &Setting) {
    let json_value = serde_json::to_string_pretty(setting).unwrap();

    // This function will create a file if it does not exist, and will truncate it if it does.
    let mut file = File::create("setting.json").unwrap();
    file.write(json_value.as_bytes()).unwrap();
}
