use serde::Deserialize;

pub enum AFKStatus {
    ONLINE = 1,
    OFFLINE = 0,
}

#[derive(Debug)]
pub struct AFKEvent {
    pub status: u8,
    pub start_time_unix: u64,
}

#[derive(Debug)]
pub struct WindowInformation {
    pub time: u64,
    pub title: Option<String>,
    pub class: Option<Vec<String>>,
    pub exec_path: Option<String>,
}

#[derive(Debug)]
pub struct BrowserInformation {
    pub start_time: String,
    pub title: Option<String>,
}

pub enum UserMetric {
    AFK(AFKEvent),
    Window(WindowInformation),
}

#[derive(Debug, Deserialize)]
pub struct BrowserData {
    #[serde(rename = "type")]
    pub data_type: BrowserDataType,
    pub title: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(rename = "windowId")]
    pub window_id: Option<u32>,
    #[serde(rename = "time")]
    pub start_time: u64,
    #[serde(rename = "tabId")]
    pub tab_id: Option<u32>,
    #[serde(default)]
    pub paused: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub enum BrowserDataType {
    Tab,
    VideoStatus,
}
