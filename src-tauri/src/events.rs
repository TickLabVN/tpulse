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
    pub time: u64,
    pub title: Option<String>,
    pub class: Option<Vec<String>>,
    pub exec_path: Option<String>,
}

pub enum UserMetric {
    AFK(AFKEvent),
    Window(WindowInformation),
}
