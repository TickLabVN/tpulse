pub mod processors;

use std::{future::Future, pin::Pin};

use into_variant::{IntoVariant, VariantFrom};

use crate::metrics::{AFKMetric, UserMetric};

#[derive(Clone)]
pub struct StartActivity {
    pub start_time: String,
    pub activity_identifier: String,
}

#[derive(Clone)]
pub struct UpdateEndActivity {
    pub start_time: String,
    pub end_time: String,
}

#[derive(Clone, VariantFrom)]
pub enum ProcessedResult {
    StartActivity(StartActivity),
    UpdateEndActivity(UpdateEndActivity),
}

pub trait MetricProcessor {
    fn process(&mut self, metric: &UserMetric) -> Option<StartActivity>;
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

        let mut results = vec![];

        let last_processor_id = self.last_processor_id.unwrap();

        if let UserMetric::AFK(afk_metric) = metric {
            results.push(handle_afk_metric(afk_metric));
        } else {
            for processor in &mut self.processor_list {
                let res = processor.as_mut().process(&metric);
                if let Some(model) = res {
                    results.push(model.into_variant());
                    break;
                }
            }
        }

        if results.len() > 0 {
            for handler in &mut self.handler_list {
                tokio::spawn(handler(results.clone()));
            }
        }
    }
}

fn handle_afk_metric(
    AFKMetric {
        start_time_unix,
        status,
    }: AFKMetric,
) -> ProcessedResult {
    // stub only
    (UpdateEndActivity {
        start_time: String::new(),
        end_time: String::new(),
    })
    .into_variant()
}
