mod helpers;

use helpers::{inverted_index_categorizer, vscode_categorizer, youtube_categorizer};

use crate::raw_metric_processor::{ActivityTag, ProcessedResult, StartActivity};

#[derive(Debug, PartialEq)]
pub struct Category(pub String);

impl Category {
    pub fn value(&self) -> String {
        self.0.clone()
    }
}

pub fn get_activity_category_tag(event: ProcessedResult) -> Option<Category> {
    if let ProcessedResult::StartActivity(StartActivity {
        tag,
        activity_identifier,
        ..
    }) = event
    {
        match tag {
            ActivityTag::WINDOW | ActivityTag::BROWSER => {
                return inverted_index_categorizer::categorize(activity_identifier.clone())
            }
            ActivityTag::VSCODE => {
                return vscode_categorizer::categorize(activity_identifier.clone())
            }
            ActivityTag::YOUTUBE => {
                return youtube_categorizer::categorize(activity_identifier.clone())
            }
        };
    } else {
        return None;
    }
}
