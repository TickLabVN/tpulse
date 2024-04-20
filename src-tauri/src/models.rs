use crate::metrics::WindowMetric;

pub type WindowLogModel = WindowMetric;

#[derive(Debug)]
pub struct BrowserLogModel {
    pub start_time: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LogModel {
    pub start_time: String,
    pub activity_identifier: String,
}

pub struct UpdateEndTimeForLogModel {
    pub start_time: String,
    pub end_time: String,
}