use crate::db::AFKStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AFKMetric {
    pub status: AFKStatus,
    pub time: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WindowMetric {
    pub time: u64,
    pub title: String,
    pub class: Vec<String>,
    pub exec_path: Option<String>,
    pub category: Option<String>,
}

impl Default for WindowMetric {
    fn default() -> Self {
        WindowMetric {
            time: 0,
            title: "".to_string(),
            class: vec![],
            exec_path: None,
            category: None,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct BrowserMetric {
    pub title: String,
    pub url: String,
    pub time: u64,
    pub category: Option<String>,
}

#[derive(Deserialize)]
pub enum Activity {
    AFK(AFKMetric),
    Window(WindowMetric),
    Browser(BrowserMetric),
}
