use lazy_static::lazy_static;
use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};
use wasm_bindgen::prelude::*;

/// Implementation of the Porter stemmer for Arabic, Danish, Dutch, English,
/// Finnish, French, German, Greek, Hungarian, Italian, Norwegian, Portuguese,
/// Romanian, Russian, Spanish, Swedish, Tamil, and Turkish.
#[wasm_bindgen]
pub struct LanguageStemmer {
    stemmer: Stemmer,
}

pub static SUPPORTED_LANGUAGES: &[&str] = &[
    "ar", "da", "nl", "en", "fi", "fr", "de", "gr", "hu", "it", "no", "pt", "ro", "ru", "es", "sv",
    "ta", "tr",
];

lazy_static! {
    // This regex includes the Unicode range \u00C0-\u00FF, which covers most of the Latin-1 Supplement characters
    static ref PUNCTUATION_REGEX: Regex = Regex::new(r"[^a-z0-9\u00C0-\u00FF\s]").unwrap();
}

#[wasm_bindgen]
impl LanguageStemmer {
    #[wasm_bindgen(constructor)]
    pub fn new(language: &str) -> Self {
        let algorithm = match language {
            "ar" => Algorithm::Arabic,
            "da" => Algorithm::Danish,
            "nl" => Algorithm::Dutch,
            "en" => Algorithm::English,
            "fi" => Algorithm::Finnish,
            "fr" => Algorithm::French,
            "de" => Algorithm::German,
            "gr" => Algorithm::Greek,
            "hu" => Algorithm::Hungarian,
            "it" => Algorithm::Italian,
            "no" => Algorithm::Norwegian,
            "pt" => Algorithm::Portuguese,
            "ro" => Algorithm::Romanian,
            "ru" => Algorithm::Russian,
            "es" => Algorithm::Spanish,
            "sv" => Algorithm::Swedish,
            "ta" => Algorithm::Tamil,
            "tr" => Algorithm::Turkish,
            _ => Algorithm::English,
        };

        LanguageStemmer {
            stemmer: Stemmer::create(algorithm),
        }
    }

    /// Returns an array of supported languages
    #[wasm_bindgen(js_name = "supportedLanguages")]
    pub fn supported_languages() -> Vec<String> {
        SUPPORTED_LANGUAGES
            .iter()
            .map(|&lang| lang.to_string())
            .collect()
    }

    /// Take a string of text as input, converts it to lowercase, removes all punctuation and splits the text on any amount of whitespace.
    /// Input:
    /// - `inputs`: A string of words (non-empty).
    ///
    /// Output:
    /// - A vector of words (all lowercase, no punctuation or whitespace).
    #[wasm_bindgen(js_name = "cleanText")]
    pub fn clean_text(text: &str) -> Vec<String> {
        PUNCTUATION_REGEX
            .replace_all(&text.to_lowercase(), "")
            .trim()
            .split_whitespace()
            .map(String::from)
            .collect()
    }

    /// Take a word as input and returns a stemmed word.
    ///
    /// For example:
    /// ```js
    /// let input = "Beautiful";
    /// let stemmer = new LanguageStemmer("en");
    /// let stemmedWord = stemmer.stemWord(input);
    /// println!("{:?}", stemmedWord); // Output "beauti"
    /// ```
    ///
    /// Input:
    /// - `inputs`: A word (non-empty).
    ///
    /// Output:
    /// - The stemmed word (lowercase).
    #[wasm_bindgen(js_name = "stemWord")]
    pub fn stem_word(&self, input: &str) -> String {
        self.stemmer.stem(input.to_lowercase().trim()).into_owned()
    }

    /// Takes a string of words as input and returns a vector of stemmed words.
    /// The stemmer used for stemming is a simple one that trims any leading or trailing spaces,
    /// converts the word to lowercase, and then applies the stemming algorithm.
    ///
    /// For example:
    /// ```js
    /// let input = "This is a test";
    /// let stemmer = new LanguageStemmer("en");
    /// let stemmedWords = stemmer.stemWords(input);
    /// println!("{:?}", stemmedWords);
    /// ```
    ///
    /// Input:
    /// - `inputs`: A string of words (non-empty).
    ///
    /// Output:
    /// - A vector of stemmed words (all lowercase).
    #[wasm_bindgen(js_name = "stemText")]
    pub fn stem_text(&self, inputs: &str) -> Vec<String> {
        Self::clean_text(inputs)
            .into_iter()
            .map(|word| self.stemmer.stem(&word).into_owned())
            .collect()
    }
}
