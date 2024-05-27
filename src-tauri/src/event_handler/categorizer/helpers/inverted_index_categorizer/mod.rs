use lazy_static::lazy_static;

use crate::event_handler::categorizer::Category;

mod utils;

lazy_static! {
    static ref HAYSTACK: Vec<(String, String)> = {
        let index_file_path = std::env::current_dir()
            .expect("Should be able to retrieve current path from inverted_index_category")
            .join("src")
            .join("event_handler")
            .join("categorizer")
            .join("helpers")
            .join("inverted_index_categorizer")
            .join("index.csv")
            .to_str()
            .expect("Should be able to get index.csv path")
            .to_string();
        utils::read_csv(index_file_path).into_iter().collect()
    };
}

// TODO: improve this fuzzy_match function
pub fn fuzzy_match<'a, T>(needle: &'a String, haystack: &'a Vec<(String, T)>) -> Option<&'a T> {
    let hay = haystack.iter().find(|(key, _)| key == needle);
    hay.map(|(_, value)| value)
}

pub fn categorize(identifier: String) -> Option<Category> {
    let needle = identifier;
    fuzzy_match(&needle, &HAYSTACK).map(|s| Category(s.clone()))
}

#[cfg(test)]
mod test {
    use crate::event_handler::categorizer::helpers::inverted_index_categorizer::categorize;
    use crate::event_handler::categorizer::Category;

    #[test]
    fn test_website_categories() {
        assert_eq!(
            categorize("atlassian.net".to_string()),
            Some(Category("Task Management".to_string()))
        );
        assert_eq!(
            categorize("github.com".to_string()),
            Some(Category("Code".to_string()))
        );
        assert_eq!(
            categorize("docs.google.com".to_string()),
            Some(Category("Documenting".to_string()))
        );
        assert_eq!(categorize("pornhub.com".to_string()), None);
    }

    #[test]
    fn test_window_categories() {
        assert_eq!(
            categorize("Chrome".to_string()),
            Some(Category("Browsing".to_string()))
        );
        assert_eq!(
            categorize("Mozilla Firefox".to_string()),
            Some(Category("Browsing".to_string()))
        );
        assert_eq!(
            categorize("Spotify".to_string()),
            Some(Category("Entertainment".to_string()))
        );
        assert_eq!(
            categorize("Rize".to_string()),
            Some(Category("Productivity".to_string()))
        );
        assert_eq!(categorize("tpulse".to_string()), None);
    }
}
