use crate::event_handler::categorizer::Category;

pub fn categorize(identifier: String) -> Option<Category> {
    Some(Category(format!("{}/VSCode", identifier)))
}
