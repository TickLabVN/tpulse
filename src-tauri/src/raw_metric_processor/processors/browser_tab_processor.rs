use crate::metrics::{BrowserMetricType, UserMetric};
use crate::raw_metric_processor::{MetricProcessor, StartActivity};
use oauth2::url;

pub struct BrowserTabProcessor;
use std::error::Error;
use url::Url;

fn get_base_url(url: &str) -> String {
    let parsed_url = Url::parse(url).unwrap();
    let base_url = parsed_url.host_str().unwrap_or("").to_string();
    base_url
}

impl MetricProcessor for BrowserTabProcessor {
    fn process(&mut self, metric: &UserMetric) -> Option<StartActivity> {
        match metric {
            UserMetric::Browser(browser_metric)
                if browser_metric.data_type == BrowserMetricType::BrowserTab =>
            {
                Some(StartActivity {
                    start_time: browser_metric.start_time as u64,
                    activity_identifier: get_base_url(&browser_metric.url.clone()?),
                })
            }
            UserMetric::AFK(afk_metric) => {
                eprintln!("AFK metric detected");
                None
            }
            _ => None,
        }
    }
}
