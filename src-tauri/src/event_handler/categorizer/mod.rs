use crate::raw_metric_processor::{ActivityTag, ProcessedResult, StartActivity};

use self::helpers::inverted_index_categorizer;

mod helpers;

#[derive(Debug, PartialEq)]
pub struct Category(String);

pub fn handle_events(events: Vec<ProcessedResult>) {
    events.into_iter().for_each(handle_event);
}

fn handle_event(event: ProcessedResult) {
    if let StartActivity {
        tag,
        activity_identifier,
        ..
    } = event
    {
        let category = match tag {
            ActivityTag::WINDOW | ActivityTag::BROWSER => {
                inverted_index_categorizer::categorize(activity_identifier)
            }
            ActivityTag::VSCODE => None,
            ActivityTag::YOUTUBE => None,
        };
    }
}
