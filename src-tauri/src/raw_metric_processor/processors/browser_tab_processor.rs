use crate::metrics::{BrowserMetricType, UserMetric};
use crate::raw_metric_processor::{MetricProcessor, StartActivity};
use oauth2::url;

pub struct BrowserTabProcessor;
use std::error::Error;
use url::Url;

// fn get_base_url(url: &str) -> Result<String, Box<dyn Error>> {
//     let parsed_url = Url::parse(url)?;
//     let base_url = format!(
//         "{}://{}",
//         parsed_url.scheme(),
//         parsed_url.host_str().unwrap_or("")
//     );
//     Ok(base_url)
// }

fn get_base_url(url: &str) -> String {
    let parsed_url = Url::parse(url).unwrap();
    let base_url = format!(
        "{}://{}",
        parsed_url.scheme(),
        parsed_url.host_str().unwrap_or("")
    );
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
            _ => None,
        }
    }
}
