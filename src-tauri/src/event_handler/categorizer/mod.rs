mod helpers;

use helpers::{inverted_index_categorizer, vscode_categorizer, youtube_categorizer};

use crate::raw_metric_processor::{ActivityTag, ProcessedResult, StartActivity};

#[derive(Debug, PartialEq)]
pub struct Category(String);

pub fn handle_events(events: Vec<ProcessedResult>) {
    events.into_iter().for_each(handle_event);
}

fn handle_event(event: ProcessedResult) {
    if let ProcessedResult::StartActivity(StartActivity {
        tag,
        activity_identifier,
        ..
    }) = event
    {
        let category = match tag {
            ActivityTag::WINDOW | ActivityTag::BROWSER => {
                inverted_index_categorizer::categorize(activity_identifier)
            }
            ActivityTag::VSCODE => vscode_categorizer::categorize(activity_identifier),
            ActivityTag::YOUTUBE => youtube_categorizer::categorize(activity_identifier),
        };
    }
}
