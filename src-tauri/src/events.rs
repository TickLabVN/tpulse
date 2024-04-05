use serde::{Deserialize, Serialize};

pub enum AFKStatus {
    ONLINE = 1,
    OFFLINE = 0,
}

#[derive(Debug)]
pub struct AFKEvent {
    pub status: u8,
    pub time: u64,
}

#[derive(Debug)]
pub struct WindowInformation {
    pub time: u128,
    pub title: Option<String>,
    pub class: Option<Vec<String>>,
    pub exec_path: Option<String>,
}

#[derive(Debug)]
pub struct BrowserInformation {
    pub start_time: u64,
    pub title: Option<String>,
}

pub enum UserMetric {
    AFK(AFKEvent),
    Window(WindowInformation),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BrowserData {
    #[serde(rename = "type")]
    pub data_type: String,
    pub title: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub window_id: Option<u32>,
    #[serde(default)]
    pub start_time: u64,
    pub tabid: u32,
    #[serde(default)]
    pub paused: bool,
}
