use super::{browser::categorize_browser_tab, window::categorize_window};
use crate::{
    db::{self, BrowserActivity, WindowActivity},
    metric::schema::{Activity, BrowserMetric, WindowMetric},
};
use log::info;
use url::Url;

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

                info!("Window metric: {:?}", metric);
                db::insert_window_activity(
                    metric.time,
                    &WindowActivity {
                        id: metric.title.clone(),
                        title: metric.title.clone(),
                        class: metric.class.join(","),
                        execute_binary: metric.exec_path.clone(),
                        category: metric.category.clone(),
                    },
                );
            }
            Activity::Browser(m) => {
                for cfn in &self.browser_categorize_fn {
                    cfn(m);
                }

                let id = {
                    let url = Url::parse(&m.url).unwrap();
                    let hostname = url.host_str().unwrap();
                    hostname.to_string()
                };
                db::insert_browser_activity(
                    m.time,
                    &BrowserActivity {
                        id,
                        title: m.title.clone(),
                        url: m.url.clone(),
                        category: m.category.clone(),
                    },
                );
                info!("Browser metric: {:?}", m);
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
