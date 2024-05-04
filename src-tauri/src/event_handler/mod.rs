use crate::raw_metric_processor::ProcessedResult;

pub mod logger;

pub type EventHandler = Box<dyn Fn(Vec<ProcessedResult>) + Send>;

pub fn make_event_handler(bare_fn: fn(Vec<ProcessedResult>)) -> EventHandler {
    Box::new(bare_fn)
}
