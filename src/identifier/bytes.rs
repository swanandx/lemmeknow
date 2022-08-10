//! For identifying bytes

use once_cell::sync::Lazy;
use regex::bytes::Regex;
use serde::Serialize;

use crate::Data;
use crate::DATA;

static REGEX_DATA: Lazy<Vec<RegexData>> = Lazy::new(build_regexes);
static BOUNDARYLESS_REGEX_DATA: Lazy<Vec<RegexData>> = Lazy::new(build_boundaryless_regexes);

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

struct RegexData {
    compiled_regex: Regex,
    data: Data,
}

impl RegexData {
    fn new(compiled_regex: Regex, data: Data) -> RegexData {
        RegexData {
            compiled_regex,
            data,
        }
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
    /// let identifier = lemmeknow::Identifier::default();
    /// let result = identifier.identify("UC11L3JDgDQMyH8iolKkVZ4w");
    /// assert_eq!(result[0].data.name, "YouTube Channel ID");
    /// ```
    ///
    pub fn identify(&self, text: &[u8]) -> Vec<Match> {
        let regexes = if self.boundaryless {
            &BOUNDARYLESS_REGEX_DATA
        } else {
            &REGEX_DATA
        };

        // iter has almost same or sometimes better performance than par_iter for single text!
        let mut all_matches = Vec::<Match>::new();
        regexes
            .iter()
            .filter(|x| is_valid_filter(self, x))
            .for_each(|re| {
                if re.compiled_regex.is_match(text) {
                    all_matches.push(Match::new(text.to_owned(), re.data.clone()))
                }
            });
        all_matches
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
    /// let identifier = lemmeknow::Identifier::default();
    /// let some_result = identifier.first_match("8888888888");
    /// let not_gonna_find = identifier.first_match("a friend for swanandx");
    ///  
    /// assert_eq!(some_result.unwrap().data.name, "Phone Number");
    /// assert!(not_gonna_find.is_none());
    /// ```
    ///
    pub fn first_match(&self, text: &[u8]) -> Option<Match> {
        let regexes = if self.boundaryless {
            &BOUNDARYLESS_REGEX_DATA
        } else {
            &REGEX_DATA
        };

        for re in regexes.iter().filter(|x| is_valid_filter(self, x)) {
            // only consider the regex which compiles!
            if re.compiled_regex.is_match(text) {
                return Some(Match::new(text.to_owned(), re.data.clone()));
            }
        }

        None
    }
}

// Identifier implementation for wasm
#[cfg(target_arch = "wasm32")]
impl Identifier {
    // There is no file system on the web, so we are not reading strings from file.
    // let the user perform the I/O and read the file, then pass the content of it.
    pub fn identify(&self, text: &[Vec<u8>]) -> Vec<Match> {
        let regexes = if self.boundaryless {
            &BOUNDARYLESS_REGEX_DATA
        } else {
            &REGEX_DATA
        };
        let mut all_matches = Vec::<Match>::new();

        text.iter().for_each(|text| {
            regexes
                .iter()
                .filter(|x| is_valid_filter(self, x))
                .for_each(|re| {
                    if re.compiled_regex.is_match(text) {
                        all_matches.push(Match::new(text.to_owned(), re.data.clone()))
                    }
                })
        });

        all_matches
    }
}

fn is_valid_filter(configs: &Identifier, regex_data: &RegexData) -> bool {
    if regex_data.data.rarity < configs.min_rarity {
        return false;
    }
    if regex_data.data.rarity > configs.max_rarity {
        return false;
    }

    if configs
        .tags
        .iter()
        .any(|y| !regex_data.data.tags.iter().any(|x| x == y))
    {
        return false;
    }
    if configs
        .exclude_tags
        .iter()
        .any(|y| regex_data.data.tags.iter().any(|x| x == y))
    {
        return false;
    }

    true
}

fn build_regexes() -> Vec<RegexData> {
    let mut regexes: Vec<RegexData> = Vec::new();

    for data in DATA.iter() {
        // Some regex from pywhat's regex.json might not work with fancy_regex
        // So we are just considering the ones which are valid.
        let result = Regex::new(data.regex); //call .unwrap() here if you want to see which regexes fail
        if let Ok(result) = result {
            regexes.push(RegexData::new(result, data.to_owned()))
        }
    }
    regexes
}

fn build_boundaryless_regexes() -> Vec<RegexData> {
    let mut regexes: Vec<RegexData> = Vec::new();

    for data in DATA.iter() {
        // Some regex from pywhat's regex.json might not work with fancy_regex
        // So we are just considering the ones which are valid.
        let result = Regex::new(data.boundaryless); //call .unwrap() here if you want to see which regexes fail
        if let Ok(result) = result {
            regexes.push(RegexData::new(result, data.to_owned()))
        }
    }
    regexes
}
