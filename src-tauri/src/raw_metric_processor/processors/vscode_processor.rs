use crate::{
    metrics::{UserMetric, WindowMetric},
    raw_metric_processor::{ActivityTag, MetricProcessor, StartActivity},
};
pub struct VSCodeProcessor;

#[cfg(target_os = "linux")]
const VSCODE_CLASS_NAME: &str = "code";

#[cfg(target_os = "windows")]
const VSCODE_CLASS_NAME: &str = "code.exe";

impl MetricProcessor for VSCodeProcessor {
    fn process(&mut self, metric: &UserMetric) -> Option<StartActivity> {
        match metric.clone() {
            UserMetric::Window(WindowMetric {
                class, title, time, ..
            }) => {
                if !class?.contains(&VSCODE_CLASS_NAME.to_string()) {
                    return None;
                }
                extract_project_name_from_vscode_title(&title?).map(|activity_identifier| {
                    StartActivity {
                        activity_identifier,
                        start_time: time,
                        tag: ActivityTag::VSCODE,
                    }
                })
            }
            UserMetric::AFK(_) => {
                println!("Warning: Metric processor should not receive AFK");
                None
            }
            _ => None,
        }
    }
}

fn extract_project_name_from_vscode_title(title: &str) -> Option<String> {
    let title_components: Vec<&str> = title.split(" - ").collect();

    let project_name = title_components.get(title_components.len() - 2);

    project_name.map(|name| name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vscode_processor_with_vscode_title() {
        #[cfg(target_os = "linux")]
        let window_metric = WindowMetric {
            time: 1620156000,
            title: Some("Welcome to Settings Sync - tpulse - Visual Studio Code".to_string()),
            class: Some(vec!["code".to_string()]),
            exec_path: None,
        };

        #[cfg(target_os = "windows")]
        let window_metric = WindowMetric {
            time: 1620156000,
            title: Some("Welcome to Settings Sync - tpulse - Visual Studio Code".to_string()),
            class: Some(vec!["code.exe".to_string()]),
            exec_path: None,
        };

        let user_metric = UserMetric::Window(window_metric);

        let mut processor = VSCodeProcessor;

        let result = processor.process(&user_metric);
        assert!(result.is_some());

        let activity = result.unwrap();
        assert_eq!(activity.start_time, 1620156000);
        assert_eq!(activity.activity_identifier, "tpulse".to_string());
    }

    #[test]
    fn test_vscode_processor_with_non_vscode_title() {
        let window_metric = WindowMetric {
            time: 1620156000, // Arbitrary timestamp
            title: Some("#chung | Just a simple cloud - Discord".to_string()),
            class: Some(vec!["discord".to_string()]),
            exec_path: None,
        };

        let user_metric = UserMetric::Window(window_metric);

        let mut processor = VSCodeProcessor;

        let result = processor.process(&user_metric);
        assert!(result.is_none());
    }
}
