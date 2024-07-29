use crate::metrics::{Activity, WindowMetric};

pub fn process_vscode(metric: &WindowMetric, activity: &mut Activity) -> bool {
    if let Some(ref class) = metric.class {
        if !class.contains(&"code".to_string()) && !class.contains(&"Code".to_string()) {
            return false;
        }

        activity.identifier = parse_title(&metric.title.as_ref().unwrap());
        activity.exec_path = metric.exec_path.clone();
        activity
            .tags
            .append(&mut vec!["programming".to_owned(), "vscode".to_owned()]);
        return true;
    }
    false
}

fn parse_title(title: &str) -> String {
    // Title pattern: [file_name] - [project_name] - Visual Studio Code
    let title_components: Vec<&str> = title.split(" - ").collect();
    if title_components.len() == 1 {
        return title_components[0].to_string();
    }
    let project_name = title_components[title_components.len() - 2];
    project_name.to_string()
}
