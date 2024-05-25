pub mod processors;

use std::{fmt::Debug, future::Future, pin::Pin};

use chrono::DateTime;
use into_variant::{IntoVariant, VariantFrom};

use crate::metrics::{AFKMetric, AFKStatus, UserMetric};

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
    last_activity: Option<StartActivity>,
    handler_list:
        Vec<Box<dyn Fn(Vec<ProcessedResult>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>>>,
}

impl RawMetricProcessorManager {
    pub fn new() -> Self {
        Self {
            processor_list: vec![],
            last_activity: None,
            handler_list: vec![],
        }
    }

    pub fn register_processor(&mut self, processor: impl MetricProcessor + 'static) {
        self.processor_list.push(Box::new(processor));
    }

    pub fn register_handler(
        &mut self,
        handler: impl Fn(Vec<ProcessedResult>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>
            + 'static,
    ) {
        self.handler_list.push(Box::new(handler));
    }

    pub async fn handle_metric(mut self: Pin<&mut Self>, metric: UserMetric) {
        let mut results = vec![];

        // handle AFK metrics specially
        if let UserMetric::AFK(afk_metric) = metric {
            if self.last_activity.is_none() {
                println!("Warning: AFK while there's no previous activity?");
            } else {
                results.push(handle_afk_metric(
                    self.last_activity.as_ref().unwrap().clone(),
                    afk_metric,
                ));
            }
        } else {
            // only window and browser metrics are passed here
            // TODO: Find a way to push this (at least partially) to a compile-time check
            for processor in &mut self.processor_list {
                let res = processor.as_mut().process(&metric);
                if let Some(model) = res {
                    self.last_activity = Some(model.clone());
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
    last_activity: StartActivity,
    AFKMetric {
        start_time_unix,
        status,
    }: AFKMetric,
) -> ProcessedResult {
    let datetime = DateTime::from_timestamp_nanos(start_time_unix as i64);
    let timestamp = datetime.format("%Y-%m-%dT%H:%M:%S").to_string();
    if status == AFKStatus::ONLINE {
        (StartActivity {
            start_time: timestamp,
            ..last_activity
        })
        .into_variant()
    } else {
        (UpdateEndActivity {
            start_time: last_activity.start_time,
            end_time: timestamp,
        })
        .into_variant()
    }
}
