use std::collections::{HashMap, HashSet};

use super::analyzer::{Analyzer, Language};
pub struct InvertedIndex {
    idx: HashMap<String, HashSet<u64>>,
    analyzer: Analyzer,
}

pub struct Document {
    id: u64,
    text: String,
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

    fn search(&self, text: &str) -> HashSet<u64> {
        let mut result: HashSet<u64> = HashSet::new();
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
