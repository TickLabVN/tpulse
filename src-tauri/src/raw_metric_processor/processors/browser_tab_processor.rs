use crate::metrics::{BrowserMetricType, UserMetric};
use crate::raw_metric_processor::{MetricProcessor, StartActivity};

pub struct BrowserTabProcessor;

impl MetricProcessor for BrowserTabProcessor {
    fn process(&mut self, metric: &UserMetric) -> Option<StartActivity> {
        match metric {
            UserMetric::Browser(browser_metric) => {
                if browser_metric.data_type == BrowserMetricType::BrowserTab {
                    Some(StartActivity {
                        start_time: browser_metric.start_time as u64,
                        activity_identifier: browser_metric
                            .url
                            .clone()
                            .unwrap_or_else(|| String::from("default")),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
