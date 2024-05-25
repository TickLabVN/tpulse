use crate::{
    metrics::{UserMetric, WindowMetric},
    raw_metric_processor::{ActivityTag, MetricProcessor, StartActivity},
};
pub struct DefaultProcessor;

impl MetricProcessor for DefaultProcessor {
    fn process(&mut self, metric: &UserMetric) -> Option<StartActivity> {
        match metric.clone() {
            UserMetric::Window(WindowMetric { title, time, .. }) => {
                title.map(|activity_identifier| StartActivity {
                    activity_identifier,
                    start_time: time,
                    tag: ActivityTag::WINDOW,
                })
            }
            UserMetric::AFK(_) => {
                println!("Warning: Metric processor should not receive AFK");
                None
            }
            UserMetric::Browser(_) => {
                println!("Warning: Default processor should not receive Browser");
                None
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::metrics::{BrowserMetric, BrowserMetricType};

    use super::*;

    #[test]
    fn test_default_processor_github_desktop_window_metric() {
        let window_metric = WindowMetric {
            time: 1620156000,
            title: Some("GitHub Desktop".to_string()),
            class: Some(vec![
                "github desktop".to_string(),
                "Github Desktop".to_string(),
            ]),
            exec_path: Some("/usr/lib/github-desktop/github-desktop".to_string()),
        };

        let user_metric = UserMetric::Window(window_metric);

        let mut processor = DefaultProcessor;

        let result = processor.process(&user_metric);
        assert!(result.is_some());

        let activity = result.unwrap();
        assert_eq!(activity.start_time, 1620156000);
        assert_eq!(activity.activity_identifier, "GitHub Desktop".to_string());
    }

    #[test]
    fn test_default_processor_with_browser_metric() {
        let browser_metric = BrowserMetric {
            data_type: BrowserMetricType::BrowserTab,
            title: "Google".to_string(),
            url: Some("https://www.google.com".to_string()),
            window_id: Some(1),
            start_time: 1620156000,
            tab_id: Some(1),
            paused: None,
        };

        let user_metric = UserMetric::Browser(browser_metric);

        let mut processor = DefaultProcessor;

        let result = processor.process(&user_metric);
        assert!(result.is_none());
    }
}
