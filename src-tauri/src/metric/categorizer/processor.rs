use super::{browser::categorize_browser_tab, window::categorize_window};
use crate::{
    db::{self, BrowserActivity, WindowActivity},
    metric::schema::{AFKStatus, Activity, BrowserMetric, WindowMetric},
};
use url::Url;

pub type ProcessFn<T> = fn(&mut T);

pub struct MetricProcessor {
    window_categorize_fn: Vec<ProcessFn<WindowMetric>>,
    browser_categorize_fn: Vec<ProcessFn<BrowserMetric>>,
    is_afk: bool,
}

impl MetricProcessor {
    fn new() -> Self {
        MetricProcessor {
            window_categorize_fn: Vec::new(),
            browser_categorize_fn: Vec::new(),
            is_afk: false,
        }
    }

    fn add_window_categorize_fn(&mut self, processor: ProcessFn<WindowMetric>) {
        self.window_categorize_fn.push(processor);
    }

    fn add_browser_categorize_fn(&mut self, processor: ProcessFn<BrowserMetric>) {
        self.browser_categorize_fn.push(processor);
    }

    pub fn categorize(&mut self, metric: &mut Activity) {
        match metric {
            Activity::AFK(metric) => {
                log::info!("AFK metric: {:?}", metric);
                self.is_afk = metric.status == AFKStatus::OFFLINE;
            }
            Activity::Window(metric) => {
                if self.is_afk {
                    return;
                }
                for cfn in &self.window_categorize_fn {
                    cfn(metric);
                }

                log::info!("Window metric: {:?}", metric);
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
                if self.is_afk {
                    return;
                }
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
