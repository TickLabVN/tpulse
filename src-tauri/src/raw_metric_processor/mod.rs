use std::{future::Future, pin::Pin};

use crate::{metrics::UserMetric, models::LogModel};

pub trait MetricProcessor {
    fn process(&mut self, metric: &UserMetric) -> Option<LogModel>;
}

pub struct RawMetricProcessorManager {
    processor_list: Vec<Box<dyn MetricProcessor>>,
    handler_list: Vec<Box<dyn Fn(LogModel) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>>>,
}

impl RawMetricProcessorManager {
    pub fn register_processor(&mut self, processor: impl MetricProcessor + 'static) {
        self.processor_list.push(Box::new(processor));
    }
    pub fn register_handler(
        &mut self,
        handler: impl Fn(LogModel) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + 'static,
    ) {
        self.handler_list.push(Box::new(handler));
    }

    pub async fn handle_metric(mut self: Pin<&mut Self>, metric: UserMetric) {
        let mut log_model = None;
        for processor in &mut self.processor_list {
            let res = processor.as_mut().process(&metric);
            if let Some(model) = res {
                log_model = Some(model);
                break;
            }
        }

        if let Some(model) = log_model {
            for handler in &mut self.handler_list {
                tokio::spawn(handler(model.clone()));
            }
        }
    }
}
