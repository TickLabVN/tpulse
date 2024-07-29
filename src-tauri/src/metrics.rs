use into_variant::VariantFrom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum AFKStatus {
    ONLINE = 1,
    OFFLINE = 0,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AFKMetric {
    pub status: AFKStatus,
    /// Unix timestamp
    pub start_time: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WindowMetric {
    pub time: u64,
    pub title: Option<String>,
    pub class: Option<Vec<String>>,
    pub exec_path: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum BrowserMetricType {
    VideoStatus,
    BrowserTab,
}

#[derive(VariantFrom, Debug, Deserialize, Clone)]
pub enum UserMetric {
    AFK(AFKMetric),
    Window(WindowMetric),
    Browser(BrowserMetric),
}

pub struct Activity {
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub identifier: String,
    pub url: Option<String>,
    pub exec_path: Option<String>,
    pub tags: Vec<String>,
}