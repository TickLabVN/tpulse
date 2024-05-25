use crate::{
    metrics::{BrowserMetric, BrowserMetricType, UserMetric},
    raw_metric_processor::{ActivityTag, MetricProcessor, StartActivity},
};
pub struct YoutubeProcessor;

impl MetricProcessor for YoutubeProcessor {
    fn process(&mut self, metric: &UserMetric) -> Option<StartActivity> {
        match metric.clone() {
            UserMetric::Browser(BrowserMetric {
                url,
                data_type,
                start_time,
                ..
            }) => {
                if data_type != BrowserMetricType::BrowserTab
                    || !url
                        .as_ref()
                        .is_some_and(|url| url.contains("youtube.com/watch"))
                {
                    return None;
                }

                if let Some(video_id) = extract_video_id_from_url(&url?) {
                    return Some(StartActivity {
                        start_time,
                        activity_identifier: format!("youtube.com/watch?v={}", video_id),
                        tag: ActivityTag::YOUTUBE,
                    });
                }

                None
            }
            UserMetric::AFK(_) => {
                println!("Warning: Metric processor should not receive AFK");
                None
            }
            _ => None,
        }
    }
}

fn extract_video_id_from_url(url: &str) -> Option<String> {
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

#[cfg(test)]
mod tests {
    use crate::metrics::BrowserMetric;

    use super::*;

    #[test]
    fn test_youtube_processor_with_valid_metric() {
        let browser_metric = BrowserMetric {
            data_type: BrowserMetricType::BrowserTab,
            title: "YouTube".to_string(),
            url: Some("https://www.youtube.com/watch?v=example_video_id".to_string()),
            window_id: Some(1),
            start_time: 1620156000,
            tab_id: Some(1),
            paused: None,
        };
        let user_metric = UserMetric::Browser(browser_metric);

        let mut processor = YoutubeProcessor;

        let result = processor.process(&user_metric);
        assert!(result.is_some());

        let activity = result.unwrap();
        assert_eq!(activity.start_time, 1620156000);
        assert_eq!(
            activity.activity_identifier,
            "youtube.com/watch?v=example_video_id"
        );
    }

    #[test]
    fn test_youtube_processor_with_none_video_youtube_url_param() {
        let browser_metric = BrowserMetric {
            data_type: BrowserMetricType::BrowserTab,
            title: "Youtube".to_string(),
            url: Some("https://www.youtube.com".to_string()),
            window_id: Some(1),
            start_time: 1620156000,
            tab_id: Some(1),
            paused: None,
        };
        let user_metric = UserMetric::Browser(browser_metric);

        let mut processor = YoutubeProcessor;

        let result = processor.process(&user_metric);
        assert!(result.is_none())
    }

    #[test]
    fn test_youtube_processor_with_video_youtube_url_and_time_params() {
        let browser_metric = BrowserMetric {
            data_type: BrowserMetricType::BrowserTab,
            title: "Youtube".to_string(),
            url: Some("https://www.youtube.com/watch?v=example_video_id&t=1s".to_string()),
            window_id: Some(1),
            start_time: 1620156000,
            tab_id: Some(1),
            paused: None,
        };
        let user_metric = UserMetric::Browser(browser_metric);

        let mut processor = YoutubeProcessor;

        let result = processor.process(&user_metric);
        assert!(result.is_some());

        let activity = result.unwrap();
        assert_eq!(activity.start_time, 1620156000);
        assert_eq!(
            activity.activity_identifier,
            "youtube.com/watch?v=example_video_id"
        );
    }

    #[test]
    fn test_youtube_processor_with_invalid_metric() {
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

        let mut processor = YoutubeProcessor;

        let result = processor.process(&user_metric);
        assert!(result.is_none());
    }
}
