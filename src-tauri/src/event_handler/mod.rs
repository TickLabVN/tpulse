use crate::raw_metric_processor::ProcessedResult;
use futures::{future::BoxFuture, Future};

pub mod logger;

pub type EventHandler = Box<dyn Fn(Vec<ProcessedResult>) -> BoxFuture<'static, ()>>;

pub fn make_event_handler<F: Future<Output = ()> + Send + 'static>(
    bare_fn: fn(Vec<ProcessedResult>) -> F,
) -> EventHandler {
    Box::new(move |v| Box::pin(bare_fn(v)))
}
