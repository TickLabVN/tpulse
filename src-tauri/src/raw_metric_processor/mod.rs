pub mod processors;

use into_variant::{IntoVariant, VariantFrom};

use crate::{
    event_handler::EventHandler,
    metrics::{AFKMetric, AFKStatus, UserMetric},
};

#[derive(Clone)]
pub struct StartActivity {
    pub start_time: u64,
    pub activity_identifier: String,
}

#[derive(Clone)]
pub struct UpdateEndActivity {
    pub start_time: u64,
    pub end_time: u64,
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
    processor_list: Vec<Box<dyn MetricProcessor + Send>>,
    last_activity: Option<StartActivity>,
    handler_list: Vec<EventHandler>,
}

impl RawMetricProcessorManager {
    pub fn new() -> Self {
        Self {
            processor_list: vec![],
            last_activity: None,
            handler_list: vec![],
        }
    }

    pub fn register_processor(&mut self, processor: impl MetricProcessor + Send + 'static) {
        self.processor_list.push(Box::new(processor));
    }

    pub fn register_handler(&mut self, handler: EventHandler) {
        self.handler_list.push(handler);
    }

    pub fn handle_metric(&mut self, metric: UserMetric) {
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
                if let None = res {
                    continue;
                }
                if let Some(model) = res {
                    self.last_activity = Some(model.clone());
                    results.push(model.into_variant());
                    break;
                }

                let model = res.unwrap();

                results.push(model.clone().into_variant());
                let current_activity = model.clone();

                if !self.last_activity.as_ref().is_some_and(|activity| {
                    activity.activity_identifier == current_activity.activity_identifier
                }) {
                    let last_activity = self.last_activity.as_ref().unwrap().clone();
                    results.push(
                        (UpdateEndActivity {
                            start_time: last_activity.start_time,
                            end_time: current_activity.start_time,
                        })
                        .into_variant(),
                    );
                    self.last_activity = Some(current_activity);
                }

                break;
            }
        }

        if results.len() > 0 {
            for handler in &mut self.handler_list {
                handler(results.clone());
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
    if status == AFKStatus::ONLINE {
        (StartActivity {
            start_time: start_time_unix,
            ..last_activity
        })
        .into_variant()
    } else {
        (UpdateEndActivity {
            start_time: last_activity.start_time,
            end_time: start_time_unix,
        })
        .into_variant()
    }
}
