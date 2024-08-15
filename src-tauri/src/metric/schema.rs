use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum AFKStatus {
    ONLINE = 1,
    OFFLINE = 0,
}

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

// const BROWSERS: [&str; 2] = ["firefox", "google-chrome"];

// impl WindowMetric {
//     pub fn is_browser_app (&self) -> bool {
//         if let Some(exec_path) = &self.exec_path {
//             for browser in BROWSERS.iter() {
//                 if exec_path.contains(browser) {
//                     return true;
//                 }
//             }
//         }
//         for class in self.class.iter() {
//             for browser in BROWSERS.iter() {
//                 if class.contains(browser) {
//                     return true;
//                 }
//             }
//         }
//         false
//     }
// }

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
