use lazy_static::lazy_static;
use regex::Regex;

use crate::event_handler::logger::categorizer::Category;

lazy_static! {
    static ref REGEX: Regex = Regex::new("\"category\":\"(.*?)\"").unwrap();
}

pub fn categorize(identifier: String) -> Option<Category> {
    let response = reqwest::blocking::get(identifier).ok()?;
    let text = response.text().ok()?;
    REGEX
        .captures(&text)
        .and_then(|captures| captures.get(1))
        .map(|cat| Category(cat.as_str().to_string()))
}

#[cfg(test)]
mod test {
    use crate::event_handler::categorizer::{helpers::youtube_categorizer::categorize, Category};

    #[test]
    fn test_mr_beast() {
        assert_eq!(
            categorize("https://www.youtube.com/watch?v=0e3GPea1Tyg".to_string()),
            Some(Category("Entertainment".to_string()))
        );
    }
}
