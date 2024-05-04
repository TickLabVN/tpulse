use crate::raw_metric_processor::ProcessedResult;
use std::{future::Future, pin::Pin};

pub mod logger;

pub type EventHandler =
    Box<dyn Fn(Vec<ProcessedResult>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>>;
