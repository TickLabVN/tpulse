use crate::{
    metric::categorizer::util::load_categorized_dataset,
    metrics::{BrowserMetric, BrowserMetricType},
};
use lazy_static::lazy_static;
use log::info;
use oauth2::url;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use url::Url;

lazy_static! {
    static ref HAYSTACK: Vec<(String, String)> = load_categorized_dataset("browser.csv");
    static ref MATCHER: SkimMatcherV2 = SkimMatcherV2::default();
}

pub fn categorize_browser_tab(metric: &mut BrowserMetric) {
    if metric.data_type != BrowserMetricType::BrowserTab {
        return;
    }

    let url = metric.url.as_str();
    let parsed_url = Url::parse(url).unwrap();

    if let Some(hostname) = parsed_url.host_str() {
        for (needle, category) in HAYSTACK.iter() {
            info!("Needle: {}, Category: {}, search: {}", needle, category, hostname);
            if MATCHER.fuzzy_match(&needle, hostname).is_some() {
                metric.tags.push(category.clone());
                return;
            }
        }
    }
}
