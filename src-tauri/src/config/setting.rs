use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct Setting {
    #[serde(rename = "pollTime")]
    pub poll_time: u64,

    #[serde(rename = "timeOut")]
    pub time_out: u64,

    #[serde(rename = "googleRefreshToken")]
    pub google_refresh_token: String,

    #[serde(rename = "googleAccessToken")]
    pub google_access_token: String,

    #[serde(rename = "redirectPort")]
    pub redirect_port: u16,
}

lazy_static! {
    pub static ref SETTING: Setting = read_from_file();
}

fn read_from_file() -> Setting {
    let file = File::open("setting.json");
    match file {
        Ok(f) => {
            let reader = std::io::BufReader::new(f);
            let setting: Setting = serde_json::from_reader(reader).unwrap();
            setting
        }
        Err(_) => {
            let s = Setting {
                poll_time: 1000,
                time_out: 10000,
                google_refresh_token: "".to_string(),
                google_access_token: "".to_string(),
                redirect_port: 8080,
            };
            serde_json::to_writer_pretty(&File::create("setting.json").unwrap(), &s).unwrap();
            s
        }
    }
}

pub fn get_setting() -> &'static Setting {
    &SETTING
}
