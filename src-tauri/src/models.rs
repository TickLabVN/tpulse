use crate::metrics::WindowMetric;

pub type WindowLogModel = WindowMetric;

#[derive(Debug)]
pub struct BrowserLogModel {
    pub start_time: String,
    pub title: Option<String>,
}

pub struct LogStartEvent {
    pub start_time: String,
    pub activity_identifier: String,
}

pub struct LogEndEvent {
    pub start_time: String,
    pub end_time: String,
}

pub enum LogUpdate {
    LogStart(LogStartEvent),
    LogEnd(LogEndEvent),
}
