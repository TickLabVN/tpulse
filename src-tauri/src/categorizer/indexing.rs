use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use rusqlite::Connection;

use super::analyzer::{Analyzer, Language};
use crate::utils::get_data_directory;

lazy_static! {
    static ref DB_PATH: String = format!("{}/tpulse.sqlite3", get_data_directory());
}

pub struct InvertedIndex {
    idx: HashMap<String, HashSet<i32>>,
    analyzer: Analyzer,
}

#[derive(Debug)]
pub struct Document {
    id: i32,
    text: String,
}

impl Document {
    fn build(
        db: Connection,
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
    fn new(language: Language) -> Self {
        Self {
            idx: HashMap::new(),
            analyzer: Analyzer::new(language),
        }
    }

    fn add(&mut self, docs: &[Document]) {
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

    fn search(&self, text: &str) -> HashSet<i32> {
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
    use crate::categorizer::load_table_from_path;

    use super::*;

    #[test]
    fn test_select_document() {
        use std::fs::File;
        use std::io::Write;

        let mut conn = Connection::open_in_memory().unwrap();
        let _ = load_table_from_path(
            &mut conn,
            "t",
            "Tracking_Rule_Package_Default_Export.csv",
            b',',
        );

        let result = Document::build(conn, "t").unwrap();
        let formatted_result = format!("{:#?}", result);
        let mut output = File::create("app_url_name.txt").unwrap();
        write!(output, "{}", formatted_result).unwrap();

        assert_eq!(result.len(), 448);
    }
}
