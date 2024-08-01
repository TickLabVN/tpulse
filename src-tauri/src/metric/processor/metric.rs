use super::{browser::process_browser_tab, vscode::process_vscode, youtube::process_youtube};
use crate::{metric::categorizer::categorize_video, metrics::{Activity, BrowserMetric, UserMetric, WindowMetric}};
pub type ProcessFn<T> = fn(&T, &mut Activity) -> bool;

pub struct MetricProcessor {
    window_process_fns: Vec<ProcessFn<WindowMetric>>,
    browser_process_fns: Vec<ProcessFn<BrowserMetric>>,
    categorize_fns: Vec<fn(&mut Activity)>,
}

impl MetricProcessor {
    fn new() -> Self {
        MetricProcessor {
            window_process_fns: Vec::new(),
            browser_process_fns: Vec::new(),
            categorize_fns: Vec::new(),
        }
    }

    fn add_window_process_fn(&mut self, processor: ProcessFn<WindowMetric>) {
        self.window_process_fns.push(processor);
    }

    fn add_browser_process_fn(&mut self, processor: ProcessFn<BrowserMetric>) {
        self.browser_process_fns.push(processor);
    }

    fn add_categorize_fn(&mut self, categorize_fn: fn(&mut Activity)) {
        self.categorize_fns.push(categorize_fn);
    }

    pub fn process(&self, metric: &UserMetric) {
        let activity = match metric {
            UserMetric::AFK(_) => {
                // TODO: Handle AFK metric
                None
            }
            UserMetric::Window(m) => {
                let mut activity = Activity {
                    start_time: m.time,
                    end_time: None,
                    identifier: "".to_string(),
                    url: None,
                    exec_path: m.exec_path.clone(),
                    tags: vec![],
                };
                for process_fn in &self.window_process_fns {
                    let proceeded = process_fn(&m, &mut activity);
                    if proceeded {
                        break;
                    }
                }
                Some(activity)
            }
            UserMetric::Browser(m) => {
                let mut activity = Activity {
                    start_time: m.start_time,
                    end_time: None,
                    identifier: "".to_string(),
                    url: m.url.clone(),
                    exec_path: None,
                    tags: vec![],
                };
                for process_fn in &self.browser_process_fns {
                    let proceeded = process_fn(&m, &mut activity);
                    if proceeded {
                        break;
                    }
                }
                Some(activity)
            }
        };

        if let Some(mut activity) = activity {
            for categorize_fn in &self.categorize_fns {
                categorize_fn(&mut activity);
            }   
        }
    }
}

pub fn create_processor() -> MetricProcessor {
    let mut manager = MetricProcessor::new();
    manager.add_window_process_fn(process_vscode);
    manager.add_browser_process_fn(process_youtube);
    manager.add_browser_process_fn(process_browser_tab);

    manager.add_categorize_fn(categorize_video);
    manager
}
