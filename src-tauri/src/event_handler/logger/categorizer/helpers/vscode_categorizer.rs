use crate::event_handler::logger::categorizer::Category;

pub fn categorize(identifier: String) -> Option<Category> {
    Some(Category(format!("{}/VSCode", identifier)))
}
