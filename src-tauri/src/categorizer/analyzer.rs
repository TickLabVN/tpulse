use std::borrow::Cow;
use std::collections::HashSet;

use rust_stemmers::{Algorithm, Stemmer};
use stop_words;
use unicode_segmentation::UnicodeSegmentation;

/// Break a text string into tokens and normalize the tokens.
///

/// Enum to represent supported languages. Unfortunately, Snowball does not yet support Vietnamese.  
/// Snowball is a small string processing language for creating stemming algorithms for use in Information Retrieval, plus a collection of stemming algorithms implemented using it.
/// ## What is stemming?
/// Stemming maps different forms of the same word to a common "stem" [https://github.com/snowballstem/snowball](https://github.com/snowballstem/snowball).
/// ## Example:
/// English stemmer maps `connection`, `connections`, `connective`, `connected`, and `connecting` to `connect`.  
/// So a searching for `connected` would also find documents which only have the other forms.
pub enum Language {
    English,
}

/// Tokenizer struct for breaking text into tokens
pub struct Tokenizer {}

/// Filters struct for applying various text processing filters
pub struct Filters {
    /// HashSet containing stop words specific to the chosen language ([https://github.com/stopwords-iso/stopwords-iso](https://github.com/stopwords-iso/stopwords-iso)).
    /// ## Example:
    /// \["as", "stay", "a", "will"\] -> removes stop words from tokens -> \["stay"\]
    stop_words_list: HashSet<String>,
    /// Stemmer for reducing words to their base or root form.
    stemmer: Stemmer,
}

/// Analyzer struct for combining tokenizer and filters
pub struct Analyzer {
    tokenizer: Tokenizer,
    filters: Filters,
}

impl Language {
    /// Function to get the stemmer algorithm based on language
    fn get(&self) -> Algorithm {
        use self::Language::*;
        match self {
            English => Algorithm::English,
        }
    }

    /// Function to get the stop words for a specific language
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

    /// Function to tokenize input text
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
    /// Constructor for Filters based on language
    fn new(language: Language) -> Self {
        Filters {
            stop_words_list: language.get_stop_words(),
            stemmer: Stemmer::create(language.get()),
        }
    }

    /// Function to convert tokens to lowercase
    fn lowercase<I>(&self, tokens: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = String>,
    {
        tokens.map(|s| s.to_lowercase())
    }

    /// Function to remove stop words from tokens
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

    /// Function to apply stemming to tokens
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
    /// Constructor for Analyzer based on language
    pub fn new(language: Language) -> Self {
        Analyzer {
            tokenizer: Tokenizer::new(),
            filters: Filters::new(language),
        }
    }

    /// Function to analyze text using the defined pipeline
    pub fn analyze(&self, text: &str) -> Vec<String> {
        let tokens = self.tokenizer.tokenize(text);
        let low = self.filters.lowercase(tokens);
        let stopped = self.filters.stop_words(low);
        self.filters.stemming(stopped).collect()
    }
}

#[cfg(test)]
mod analyzer_tests {
    use super::*;

    #[test]
    fn test_analyzer() {
        let analyzer = Analyzer::default();
        let text = "The rain, rain poured and poured, creating a rhythmic symphony of droplets on the windowpane!";
        let res = analyzer.analyze(text);
        // Result: ["rain", "pour", "creat", "rhythmic", "symphoni", "droplet", "windowpan"]
        assert_eq!(res.len(), 7, "text analyze failed");
    }
}
