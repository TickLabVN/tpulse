use crate::{
    metric::categorizer::util::load_categorized_dataset,
    metrics::{BrowserMetric, BrowserMetricType},
};
use lazy_static::lazy_static;
use oauth2::url;
use ruzzy::{fuzzy_match, FuzzyConfig};
use url::Url;

lazy_static! {
    static ref HAYSTACK: Vec<(String, String)> = load_categorized_dataset("browser.csv");
}

pub fn categorize_browser_tab(metric: &mut BrowserMetric) {
    if metric.data_type != BrowserMetricType::BrowserTab {
        return;
    }

    let url = metric.url.as_str();
    let parsed_url = Url::parse(url).unwrap();

    if let Some(hostname) = parsed_url.host_str() {
        if let Some(category) = get_categorize(hostname) {
            metric.tags.push(category);
        }
    }
}

fn get_categorize(hostname: &str) -> Option<String> {
    let needle = hostname.to_owned();
    fuzzy_match(
        &needle,
        &*HAYSTACK,
        FuzzyConfig {
            threshold: 3,
            insertion_penalty: None,
            deletion_penalty: None,
            substitution_penalty: None,
        },
    )
    .map(|s| s.to_string())
}
