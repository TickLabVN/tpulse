use std::collections::{HashMap, HashSet};

use rusqlite::Connection;

use super::analyzer::{Analyzer, Language};

/// Find the intersection of IDs for given tokens by Inverted Index technique.
///

/// Struct representing an inverted index with a token-to-document mapping
pub struct InvertedIndex {
    idx: HashMap<String, HashSet<i32>>,
    analyzer: Analyzer,
}

/// Struct representing a document with an ID and text content
#[derive(Debug, PartialEq)]
pub struct Document {
    id: i32,
    text: String,
}

impl Document {
    /// Builder method to construct a vector of documents from the database
    pub fn build(
        db: &Connection,
        table_name: &str,
    ) -> Result<Vec<Document>, Box<dyn std::error::Error>> {
        let mut statement = db.prepare(&format!("select c_id, c_name from {}", table_name))?;

        let documents = statement.query_map([], |row| {
            Ok(Document {
                id: row.get(0)?,
                text: row.get(1)?,
            })
        })?;

        let result = documents
            .map(|item| item.unwrap())
            .collect::<Vec<Document>>();
        Ok(result)
    }
}

impl Default for InvertedIndex {
    fn default() -> Self {
        Self::new(Language::English)
    }
}

impl InvertedIndex {
    /// Constructor for InvertedIndex with a specified language for the analyzer
    fn new(language: Language) -> Self {
        Self {
            idx: HashMap::new(),
            analyzer: Analyzer::new(language),
        }
    }

    /// Method to generate an inverted index from a vector of documents
    pub fn generate_token_index(&mut self, docs: &[Document]) {
        for doc in docs {
            for token in self.analyzer.analyze(doc.text.as_str()) {
                match self.idx.get_mut(&token) {
                    None => {
                        let v = HashSet::from([doc.id]);
                        self.idx.insert(token, v);
                    }
                    Some(v) => {
                        v.insert(doc.id);
                    }
                }
            }
        }
    }

    /// Method to categorize a text based on the generated inverted index
    pub fn categorize(&self, text: &str) -> HashSet<i32> {
        let mut result: HashSet<i32> = HashSet::new();
        for token in self.analyzer.analyze(text) {
            match self.idx.get(&token) {
                None => {}
                Some(ids) => {
                    if result.is_empty() {
                        result = ids.clone();
                    }
                    result = result.intersection(ids).copied().collect()
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod indexing_tests {
    use std::fs::File;
    use std::io::Write;

    use crate::categorizer::load_table_from_path;

    use super::*;

    #[test]
    fn test_build_document() {
        let mut conn = Connection::open_in_memory().unwrap();
        let _ = load_table_from_path(
            &mut conn,
            "t",
            "Tracking_Rule_Package_Default_Export.csv",
            b',',
        );

        let result = Document::build(&conn, "t").unwrap();
        let formatted_result = format!("{:#?}", result);
        let mut output = File::create("app_url_name.txt").unwrap();
        write!(output, "{}", formatted_result).unwrap();

        assert_eq!(result.len(), 448);
    }

    #[test]
    fn test_generate_token_index() {
        let mut conn = Connection::open_in_memory().unwrap();
        let _ = load_table_from_path(
            &mut conn,
            "t",
            "Tracking_Rule_Package_Default_Export.csv",
            b',',
        );
        let documents = Document::build(&conn, "t").unwrap();

        let mut token_idx = InvertedIndex::default();
        token_idx.generate_token_index(&documents);

        let formatted_result = format!("{:#?}", token_idx.idx);
        let mut output = File::create("token.txt").unwrap();
        write!(output, "{}", formatted_result).unwrap();

        assert_eq!(token_idx.idx.keys().len(), 453)
    }
}
