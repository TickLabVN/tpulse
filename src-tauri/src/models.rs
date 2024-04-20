use crate::metrics::WindowMetric;

pub type WindowLogModel = WindowMetric;

#[derive(Debug)]
pub struct BrowserLogModel {
    pub start_time: String,
    pub title: Option<String>,
}
