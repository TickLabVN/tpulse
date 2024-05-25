use crate::metrics::{BrowserMetricType, UserMetric};
use crate::raw_metric_processor::{ActivityTag, MetricProcessor, StartActivity};
use oauth2::url;

pub struct BrowserTabProcessor;
use url::Url;

fn get_base_url(url: &str) -> Option<String> {
    let parsed_url = Url::parse(url).ok()?;
    parsed_url.host_str().map(|s| s.to_string())
}

impl MetricProcessor for BrowserTabProcessor {
    fn process(&mut self, metric: &UserMetric) -> Option<StartActivity> {
        match metric {
            UserMetric::Browser(browser_metric)
                if browser_metric.data_type == BrowserMetricType::BrowserTab =>
            {
                Some(StartActivity {
                    start_time: browser_metric.start_time as u64,
                    activity_identifier: get_base_url(browser_metric.url.as_ref()?)?,
                    tag: ActivityTag::BROWSER,
                })
            }
            UserMetric::AFK(_) => {
                println!("Warning: Metric processor should not receive AFK");
                None
            }
            UserMetric::Browser(browser_metric) => {
                if browser_metric.data_type == BrowserMetricType::BrowserTab {
                    Some(StartActivity {
                        start_time: browser_metric.start_time as u64,
                        activity_identifier: browser_metric
                            .url
                            .clone()
                            .unwrap_or_else(|| String::from("default")),
                        tag: ActivityTag::BROWSER,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
