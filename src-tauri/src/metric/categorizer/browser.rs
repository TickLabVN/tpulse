use crate::{metric::categorizer::util::load_categorized_dataset, metric::schema::BrowserMetric};
use lazy_static::lazy_static;
use oauth2::url;
use url::Url;

lazy_static! {
    static ref HAYSTACK: Vec<(String, String)> = load_categorized_dataset("browser.csv");
}

pub fn categorize_browser_tab(metric: &mut BrowserMetric) {
    let url = Url::parse(&metric.url);
    if url.is_err() {
        return;
    }

    let binding = url.unwrap();
    let hostname = binding.host_str().unwrap();

    for (recorded_hostname, category) in HAYSTACK.iter() {
        if hostname.eq(recorded_hostname) {
            metric.label = Some((hostname.to_string(), category.clone()));
            return;
        }
    }
}
