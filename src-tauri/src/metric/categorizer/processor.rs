use log::info;

use super::{browser::categorize_browser_tab, window::categorize_window};
use crate::{
    db, metric::schema::{Activity, BrowserMetric, WindowMetric}
};
pub type ProcessFn<T> = fn(&mut T);

pub struct MetricProcessor {
    window_categorize_fn: Vec<ProcessFn<WindowMetric>>,
    browser_categorize_fn: Vec<ProcessFn<BrowserMetric>>,
}

impl MetricProcessor {
    fn new() -> Self {
        MetricProcessor {
            window_categorize_fn: Vec::new(),
            browser_categorize_fn: Vec::new(),
        }
    }

    fn add_window_categorize_fn(&mut self, processor: ProcessFn<WindowMetric>) {
        self.window_categorize_fn.push(processor);
    }

    fn add_browser_categorize_fn(&mut self, processor: ProcessFn<BrowserMetric>) {
        self.browser_categorize_fn.push(processor);
    }

    pub fn categorize(&self, metric: &mut Activity) {
        match metric {
            Activity::AFK(m) => {
                // TODO: Handle AFK metric
                info!("AFK metric: {:?}", m);
                return;
            }
            Activity::Window(metric) => {
                for cfn in &self.window_categorize_fn {
                    cfn(metric);
                }
                
                let mut id = &metric.title;
                let mut category: Option<String> = None;
                if let Some((app_name, app_category)) = &metric.label {
                    id = app_name;
                    category = Some(app_category.clone());
                }

                db::insert_window_metric(
                    metric.time,
                    &db::WindowActivity {
                        id: id.clone(),
                        title: metric.title.clone(),
                        class: metric.class.join(","),
                        execute_binary: metric.exec_path.clone(),
                        category,
                    },
                );
            }
            Activity::Browser(m) => {
                for cfn in &self.browser_categorize_fn {
                    cfn(m);
                }
            }
        };
    }
}

pub fn create_processor() -> MetricProcessor {
    let mut manager = MetricProcessor::new();
    manager.add_window_categorize_fn(categorize_window);
    manager.add_browser_categorize_fn(categorize_browser_tab);
    manager
}
