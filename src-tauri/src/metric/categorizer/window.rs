use crate::{metric::categorizer::util::load_categorized_dataset, metrics::WindowMetric};
use lazy_static::lazy_static;
use ruzzy::{fuzzy_match, FuzzyConfig};

lazy_static! {
    static ref HAYSTACK: Vec<(String, String)> = load_categorized_dataset("window.csv");
}

pub fn categorize_window(metric: &mut WindowMetric) {
    let categorize = fuzzy_match(
        &metric.title,
        &*HAYSTACK,
        FuzzyConfig {
            threshold: 3,
            insertion_penalty: None,
            deletion_penalty: None,
            substitution_penalty: None,
        },
    )
    .map(|s| s.to_string());

    if let Some(category) = categorize {
        metric.tags.push(category);
    }
}
