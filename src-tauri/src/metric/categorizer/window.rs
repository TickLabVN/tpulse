use std::collections::hash_map;

use super::util::normalize_str;
use crate::{metric::categorizer::util::load_categorized_dataset, metric::schema::WindowMetric};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use hash_map::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref HAYSTACK: Vec<(String, String)> = load_categorized_dataset("window.csv");
    static ref MATCHER: SkimMatcherV2 = SkimMatcherV2::default();
}

pub fn categorize_window(metric: &mut WindowMetric) {
    let mut score_map: HashMap<String, i64> = HashMap::new();

    let mut needle = "".to_owned();
    needle.push_str(&metric.title);
    for class in metric.class.iter() {
        needle.push_str(class);
    }
    needle = normalize_str(&needle);

    let mut max_score = -1;
    let mut max_category: Option<&str> = None;

    for (title, category) in HAYSTACK.iter() {
        if let Some(score) = MATCHER.fuzzy_match(&needle, &title) {
            let total_score = score_map.entry(category.clone()).or_insert(0);
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
