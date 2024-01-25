use std::borrow::Cow;
use std::collections::HashSet;

use rust_stemmers::{Algorithm, Stemmer};
use stop_words;
use unicode_segmentation::UnicodeSegmentation;

pub enum Language {
    English,
}

pub struct Tokenizer {}

pub struct Filters {
    stop_words_list: HashSet<String>,
    stemmer: Stemmer,
}

pub struct Analyzer {
    tokenizer: Tokenizer,
    filters: Filters,
}

impl Language {
    fn get(&self) -> Algorithm {
        use self::Language::*;
        match self {
            English => Algorithm::English,
        }
    }

    fn get_stop_words(&self) -> HashSet<String> {
        use self::Language::*;
        let stop_words = match self {
            English => stop_words::get(stop_words::LANGUAGE::English),
        };
        stop_words.into_iter().collect::<HashSet<String>>()
    }
}

impl Tokenizer {
    fn new() -> Self {
        Self {}
    }

    fn tokenize(&self, text: &str) -> impl Iterator<Item = String> {
        text.unicode_words()
            .map(str::to_string)
            .collect::<Vec<String>>()
            .into_iter()
    }
}

impl Default for Filters {
    fn default() -> Self {
        Filters::new(Language::English)
    }
}

impl Filters {
    fn new(language: Language) -> Self {
        Filters {
            stop_words_list: language.get_stop_words(),
            stemmer: Stemmer::create(language.get()),
        }
    }

    fn lowercase<I>(&self, tokens: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = String>,
    {
        tokens.map(|s| s.to_lowercase())
    }

    fn stop_words<I>(&self, tokens: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = String>,
    {
        let set_of_tokens: HashSet<String> = tokens.into_iter().collect();
        set_of_tokens
            .difference(&self.stop_words_list)
            .cloned()
            .collect::<Vec<String>>()
            .into_iter()
    }

    fn stemming<'a, I>(&'a self, tokens: I) -> impl Iterator<Item = String> + 'a
    where
        I: Iterator<Item = String> + 'a,
    {
        tokens.map(|t| match self.stemmer.stem(&t) {
            Cow::Owned(stemmed_str) => stemmed_str,
            Cow::Borrowed(stemmed_str) => stemmed_str.to_string(),
        })
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new(Language::English)
    }
}

impl Analyzer {
    pub fn new(language: Language) -> Self {
        Analyzer {
            tokenizer: Tokenizer::new(),
            filters: Filters::new(language),
        }
    }

    pub fn analyze(&self, text: &str) -> Vec<String> {
        let tokens = self.tokenizer.tokenize(text);
        let low = self.filters.lowercase(tokens);
        let stopped = self.filters.stop_words(low);
        self.filters.stemming(stopped).collect()
    }
}
