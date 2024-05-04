pub mod processors;

use std::{future::Future, pin::Pin};

use crate::metrics::UserMetric;

#[derive(Clone)]
pub enum ProcessedResult {
    StartActivity {
        start_time: String,
        activity_identifier: String,
    },
    UpdateEndActivity {
        start_time: String,
        end_time: String,
    },
}

pub trait MetricProcessor {
    fn process(&mut self, metric: &UserMetric) -> Option<ProcessedResult>;
}

pub struct RawMetricProcessorManager {
    processor_list: Vec<Box<dyn MetricProcessor>>,
    last_processor_id: Option<isize>,
    handler_list:
        Vec<Box<dyn Fn(Vec<ProcessedResult>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>>>,
}

impl RawMetricProcessorManager {
    pub fn new() -> Self {
        Self {
            processor_list: vec![],
            last_processor_id: None,
            handler_list: vec![],
        }
    }

    pub fn register_processor(&mut self, processor: impl MetricProcessor + 'static) {
        if self.last_processor_id.is_some() {
            panic!("Processors can not be registered after the manager is frozen");
        }
        self.processor_list.push(Box::new(processor));
    }

    pub fn register_handler(
        &mut self,
        handler: impl Fn(Vec<ProcessedResult>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>
            + 'static,
    ) {
        self.handler_list.push(Box::new(handler));
    }

    pub fn frozen(&mut self) {
        if self.last_processor_id.is_some() {
            panic!("The manager is already frozen");
        }
        self.last_processor_id = Some(-1);
    }

    pub async fn handle_metric(mut self: Pin<&mut Self>, metric: UserMetric) {
        if self.last_processor_id.is_none() {
            panic!("The manager must be fronzen before it's set to handle metric");
        }

        let mut result = None;
        for processor in &mut self.processor_list {
            let res = processor.as_mut().process(&metric);
            if let Some(model) = res {
                result = Some(model);
                break;
            }
        }

        if let Some(inner) = result {
            for handler in &mut self.handler_list {
                tokio::spawn(handler(inner.clone()));
            }
        }
    }
}
