use crate::metrics::{Activity, BrowserMetric, BrowserMetricType};

pub fn process_youtube(metric: &BrowserMetric, activity: &mut Activity) -> bool {
    if metric.data_type != BrowserMetricType::VideoStatus {
        return false;
    }
    if metric.url.is_none() {
        return false;
    }
    let url = metric.url.clone().unwrap();
    if let Some(video_id) = get_video_id(&url) {
        activity.identifier = format!("youtube.com/watch?v={}", video_id);
        activity.tags.push("youtube".to_string());
        return true;
    }
    false
}

fn get_video_id(url: &str) -> Option<String> {
    // Exmaple URL: https://www.youtube.com/watch?v=<video_id>&list=<playlist_id> ...

    let query_params: Vec<&str> = url.split('?').collect();

    let params = query_params.get(1)?;
    let key_value_pairs: Vec<&str> = params.split('&').collect();

    for pair in key_value_pairs {
        let kv: Vec<&str> = pair.split('=').collect();
        if kv.len() == 2 && kv[0] == "v" {
            return Some(kv[1].to_string());
        }
    }

    None
}
