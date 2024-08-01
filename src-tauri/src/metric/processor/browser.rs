use crate::metrics::{Activity, BrowserMetric, BrowserMetricType};
use oauth2::url;

use url::Url;

fn get_base_url(url: &str) -> Option<String> {
    let parsed_url = Url::parse(url).ok()?;
    parsed_url.host_str().map(|s| s.to_string())
}

pub fn process_browser_tab(metric: &BrowserMetric, activity: &mut Activity) -> bool {
    if metric.data_type != BrowserMetricType::BrowserTab {
        return false;
    }
    if metric.url.is_none() {
        return false;
    }
    let url = metric.url.as_ref().unwrap();
    let base_url = get_base_url(url);
    if base_url.is_none() {
        return false;
    }
    let base_url = base_url.unwrap();
    activity.identifier = base_url;
    true
}
