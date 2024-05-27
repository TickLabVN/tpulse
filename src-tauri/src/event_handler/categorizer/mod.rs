use crate::raw_metric_processor::ProcessedResult;

mod helpers;

#[derive(Debug, PartialEq)]
pub struct Category(String);

pub fn handle_events(events: Vec<ProcessedResult>) {}
