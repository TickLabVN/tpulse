pub mod analyzer;
mod indexing;
mod loader;
pub use indexing::{Document, InvertedIndex};
pub use loader::load_table_from_path;
