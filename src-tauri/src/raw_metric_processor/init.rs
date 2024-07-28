use crate::event_handler::{logger, make_event_handler};

use super::{
    processors::{
        browser_tab_processor::BrowserTabProcessor, default_processor::DefaultProcessor,
        vscode_processor::VSCodeProcessor, youtube_processor::YoutubeProcessor,
    },
    RawMetricProcessorManager,
};

pub fn initialize() -> RawMetricProcessorManager {
    let mut raw_metric_processor = RawMetricProcessorManager::new();

    // Register raw metric processors here
    raw_metric_processor.register_processor(YoutubeProcessor);
    raw_metric_processor.register_processor(BrowserTabProcessor);
    raw_metric_processor.register_processor(VSCodeProcessor);
    raw_metric_processor.register_processor(DefaultProcessor);

    // Register event handlers here
    raw_metric_processor.register_handler(make_event_handler(logger::handle_events));

    raw_metric_processor
}
