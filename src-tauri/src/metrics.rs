use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum AFKStatus {
    ONLINE = 1,
    OFFLINE = 0,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AFKMetric {
    pub status: AFKStatus,
    pub start_time_unix: u64,
}

#[derive(Debug)]
pub struct WindowMetric {
    pub time: u128,
    pub title: Option<String>,
    pub class: Option<Vec<String>>,
    pub exec_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BrowserMetric {
    #[serde(rename = "type")]
    pub data_type: BrowserMetricType,
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
pub enum BrowserMetricType {
    VideoStatus,
    BrowserTab,
}

pub enum UserMetric {
    AFK(AFKMetric),
    Window(WindowMetric),
    Browser(BrowserMetric),
}
