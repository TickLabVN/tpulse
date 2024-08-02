use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum AFKStatus {
    ONLINE = 1,
    OFFLINE = 0,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AFKMetric {
    pub status: AFKStatus,
    pub start_time: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WindowMetric {
    pub time: u64,
    pub title: String,
    pub class: Vec<String>,
    pub exec_path: String,
    pub category: Option<String>,
}

#[derive(Clone, Deserialize, PartialEq, Debug)]
pub enum BrowserMetricType {
    VideoStatus,
    BrowserTab,
}

#[derive(Clone, Deserialize, Debug)]
pub struct BrowserMetric {
    #[serde(rename = "type")]
    pub data_type: BrowserMetricType,
    pub title: String,
    pub url: String,
    pub time: u64,
    pub paused: bool,
    pub category: Option<String>,
}

#[derive(Deserialize, Clone)]
pub enum Activity {
    AFK(AFKMetric),
    Window(WindowMetric),
    Browser(BrowserMetric),
}
