//! For identifying bytes

use once_cell::sync::Lazy;
use regex::bytes::Regex;
use serde::Serialize;

use crate::Data;
use crate::DATA;

// this is REGEX_DATA and BOUNDARYLESS_REGEX_DATA
include!(concat!(env!("OUT_DIR"), "/regex_data.rs"));

pub struct Identifier {
    /// Keep Data having minimum Rarity of supplied `min_rarity`
    pub min_rarity: f32,
    /// Keep Data having maximum Rarity of supplied `max_rarity`
    pub max_rarity: f32,
    /// Only include the Data which have at least one of the specified `tags`
    pub tags: Vec<String>,
    /// Only include Data which doesn't have any of the `excluded_tags`
    pub exclude_tags: Vec<String>,
    /// Use boundaryless regex
    pub boundaryless: bool,
    /// Scan files having supplied text as filename
    pub file_support: bool,
}

impl Identifier {
    #[inline]
    pub fn min_rarity(mut self, rarity: f32) -> Self {
        self.min_rarity = rarity;
        self
    }

    #[inline]
    pub fn max_rarity(mut self, rarity: f32) -> Self {
        self.max_rarity = rarity;
        self
    }

    #[inline]
    pub fn include_tags(mut self, tags: &[String]) -> Self {
        self.tags.extend_from_slice(tags);
        self
    }

    #[inline]
    pub fn exclude_tags(mut self, tags: &[String]) -> Self {
        self.exclude_tags.extend_from_slice(tags);
        self
    }

    #[inline]
    pub fn boundaryless(mut self, boundaryless: bool) -> Self {
        self.boundaryless = boundaryless;
        self
    }

    #[inline]
    pub fn file_support(mut self, support: bool) -> Self {
        self.file_support = support;
        self
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Identifier {
            min_rarity: 0.0,
            max_rarity: 1.0,
            tags: vec![],
            exclude_tags: vec![],
            boundaryless: false,
            file_support: false,
        }
    }
}

/// structure containing the bytes and it's possible identification.
#[derive(Serialize, Debug)]
pub struct Match {
    pub text: Vec<u8>,
    pub data: Data,
}

impl Match {
    pub fn new(text: Vec<u8>, data: Data) -> Match {
        Match { text, data }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Identifier {
    /// Identify the given bytes.
    ///
    /// Finds all possible identifications.
    ///
    /// # Arguments
    ///
    /// * text: &[u8] - text which we want to identify
    ///
    /// # Examples
    ///
    /// ```
    /// let identifier = lemmeknow::bytes::Identifier::default();
    /// let text = b"UC11L3JDgDQMyH8iolKkVZ4w";
    /// let result = identifier.identify(text);
    /// assert_eq!(result[0].data.name, "YouTube Channel ID");
    /// ```
    ///
    pub fn identify(&self, text: &[u8]) -> Vec<Match> {
        let regexes = if self.boundaryless {
            &BOUNDARYLESS_REGEX
        } else {
            &REGEX
        };
        DATA.iter()
            .enumerate()
            .filter_map(|(i, e)| {
                if is_valid_filter(self, e) && regexes[i].is_match(text) {
                    Some(Match::new(text.to_owned(), e.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<Match>>()
    }

    /// This returns the first identification.
    ///
    /// Due to how data is stored, this means that the returned result has the highest `rarity`.
    ///
    /// # Arguments
    ///
    /// * text: &[u8] - text which we want to identify
    ///
    /// # Examples
    ///
    /// ```
    /// let identifier = lemmeknow::bytes::Identifier::default();
    /// let some_result = identifier.first_match(b"8888888888");
    /// let not_gonna_find = identifier.first_match(b"a friend for swanandx");
    ///  
    /// assert_eq!(some_result.unwrap().data.name, "Phone Number");
    /// assert!(not_gonna_find.is_none());
    /// ```
    ///
    pub fn first_match(&self, text: &[u8]) -> Option<Match> {
        let regexes = if self.boundaryless {
            &BOUNDARYLESS_REGEX
        } else {
            &REGEX
        };

        for (i, x) in DATA
            .iter()
            .enumerate()
            .filter(|(_, x)| is_valid_filter(self, x))
        {
            // only consider the regex which compiles!
            if regexes[i].is_match(text) {
                return Some(Match::new(text.to_owned(), x.clone()));
            }
        }

        None
    }
}

// Output Implementation
impl Identifier {
    /// Convert [`Vec<Match>`] to JSON
    ///
    /// Returns prettified JSON string.
    ///
    /// Helpful if you want to convert possible identifications to JSON
    /// for using in web APIs or something else.
    ///
    /// # Arguments
    ///
    /// * result: &[Match] - Reference to `Vec<Match>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lemmeknow::Identifier;
    /// let identifier = Identifier::default();
    /// let result = identifier.identify("UC11L3JDgDQMyH8iolKkVZ4w");
    /// let result_in_json = Identifier::to_json(&result);
    /// println!("{result_in_json}");
    /// ```
    ///
    #[inline]
    pub fn to_json(result: &[Match]) -> String {
        serde_json::to_string_pretty(result).unwrap_or_default()
    }
}

// Identifier implementation for wasm
#[cfg(target_arch = "wasm32")]
impl Identifier {
    // There is no file system on the web, so we are not reading strings from file.
    // let the user perform the I/O and read the file, then pass the content of it.
    pub fn identify(&self, text: &[Vec<u8>]) -> Vec<Match> {
        let regexes = if self.boundaryless {
            &BOUNDARYLESS_REGEX
        } else {
            &REGEX
        };

        DATA.iter()
            .enumerate()
            .filter_map(|(i, e)| {
                if is_valid_filter(self, e) && regexes[i].is_match(text) {
                    Some(Match::new(text.to_owned(), e.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<Match>>()
    }
}

fn is_valid_filter(configs: &Identifier, regex_data: &Data) -> bool {
    if regex_data.rarity < configs.min_rarity {
        return false;
    }
    if regex_data.rarity > configs.max_rarity {
        return false;
    }

    if configs
        .tags
        .iter()
        .any(|y| !regex_data.tags.iter().any(|x| x == y))
    {
        return false;
    }
    if configs
        .exclude_tags
        .iter()
        .any(|y| regex_data.tags.iter().any(|x| x == y))
    {
        return false;
    }

    true
}
