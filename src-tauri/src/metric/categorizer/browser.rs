use crate::metric::{categorizer::util::load_categorized_dataset, schema::BrowserMetric};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use lazy_static::lazy_static;
use oauth2::url;
use std::collections::HashMap;
use url::Url;

lazy_static! {
    static ref HAYSTACK: Vec<(String, String)> = load_categorized_dataset("browser.csv");
    static ref MATCHER: SkimMatcherV2 = SkimMatcherV2::default();
}

pub fn categorize_browser_tab(metric: &mut BrowserMetric) {
    let mut score_map: HashMap<String, i64> = HashMap::new();

    let needle = Url::parse(&metric.url);
    if needle.is_err() {
        return;
    }

    let binding = needle.unwrap();
    let hostname = binding.host_str().unwrap();

    let mut max_score = -1;
    let mut max_category: Option<&str> = None;

    for (domain, category) in HAYSTACK.iter() {
        if let Some(score) = MATCHER.fuzzy_match(hostname, &domain) {
            let total_score = score_map.entry(domain.clone()).or_insert(0);
            *total_score += score;

            if *total_score > max_score {
                max_score = *total_score;
                max_category = Some(category);
            }
        }
    }

    if let Some(category) = max_category {
        metric.category = Some(category.to_string());
    }
}
