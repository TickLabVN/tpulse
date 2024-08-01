use crate::metrics::Activity;
use lazy_static::lazy_static;
use log::error;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new("\"category\":\"(.*?)\"").unwrap();
}

pub fn categorize_video(activity: &mut Activity) {
    if activity.url.is_none() {
        return;
    }
    let url = activity.url.as_ref().unwrap();
    let response = reqwest::blocking::get(url);
    match response {
        Ok(response) => {
            let text = response.text();
            if text.is_err() {
                return;
            }
            let text = text.unwrap();
            let captures = REGEX
                .captures(&text)
                .and_then(|captures| captures.get(1))
                .map(|cat| cat.as_str().to_string());

            if let Some(c) = captures {
                activity.tags.push(c);
            }
        }
        Err(e) => {
            error!("Failed to get response from URL: {}", e);
        }
    }
}
