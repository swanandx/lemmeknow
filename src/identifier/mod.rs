//! For identifying text / analyzing files

use fancy_regex::Regex;

#[cfg(not(target_arch = "wasm32"))]
use {
    rayon::iter::{IntoParallelRefIterator, ParallelIterator},
    std::sync::{Arc, Mutex},
    std::{fs, str},
};

use crate::Data;
use crate::Match;

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

#[derive(Default)]
pub struct Identify {
    /// Keep Data having minimum Rarity of supplied `min_rarity`
    pub min_rarity: Option<f32>,
    /// Keep Data having maximum Rarity of supplied `max_rarity`
    pub max_rarity: Option<f32>,
    /// Only include the Data which have at least one of the specified `tags`
    pub tags: Vec<String>,
    /// Only include Data which doesn't have any of the `excluded_tags`
    pub exclude_tags: Vec<String>,
    /// Use boundaryless regex
    pub boundaryless: bool,
    /// Scan files having supplied text as filename
    pub file_support: bool,
}

// Filter implementation
// TODO: check if #[inline] provides some perf improvement.
impl Identify {
    pub fn min_rarity(mut self, rarity: f32) -> Self {
        self.min_rarity = Some(rarity);
        self
    }

    pub fn max_rarity(mut self, rarity: f32) -> Self {
        self.max_rarity = Some(rarity);
        self
    }

    pub fn include_tags(mut self, tags: &[String]) -> Self {
        self.tags.extend_from_slice(tags);
        self
    }

    pub fn exclude_tags(mut self, tags: &[String]) -> Self {
        self.exclude_tags.extend_from_slice(tags);
        self
    }

    pub fn boundaryless(mut self, boundaryless: bool) -> Self {
        self.boundaryless = boundaryless;
        self
    }

    pub fn file_support(mut self, support: bool) -> Self {
        self.file_support = support;
        self
    }

    fn filter_json_data(&self, json_data: &mut Vec<Data>) {
        if self.boundaryless {
            for data in json_data.iter_mut() {
                data.regex = Regex::new(r"(?<!\\)\^(?![^\[\]]*(?<!\\)\])")
                    .expect("can't compile for boundaryless")
                    .replace(&data.regex, "")
                    .to_string();
                data.regex = Regex::new(r"(?<!\\)\$(?![^\[\]]*(?<!\\)\])")
                    .expect("can't compile for boundaryless")
                    .replace(&data.regex, "")
                    .to_string();
            }
        }

        if let Some(min_rarity) = self.min_rarity {
            json_data.retain(|x| x.rarity >= min_rarity)
        }
        if let Some(max_rarity) = self.max_rarity {
            json_data.retain(|x| x.rarity <= max_rarity)
        }
        if !self.tags.is_empty() {
            json_data.retain(|x| self.tags.iter().any(|y| x.tags.contains(y)))
        }
        if !self.exclude_tags.is_empty() {
            json_data.retain(|x| self.exclude_tags.iter().any(|y| !x.tags.contains(y)))
        }
    }
}

// Identifier implementation
#[cfg(not(target_arch = "wasm32"))]
impl Identify {
    /// Identify the given text.
    ///
    /// This will read strings from file with text as filename if `file_support` is `true` and the file exists
    ///
    /// Finds all possible identifications.
    ///
    /// # Arguments
    ///
    /// * text: &str - text which we want to identify
    ///
    /// # Examples
    ///
    /// ```
    /// let identifier = lemmeknow::Identify::default();
    /// let result = identifier.identify("UC11L3JDgDQMyH8iolKkVZ4w");
    /// assert_eq!(result[0].data.name, "YouTube Channel ID");
    /// ```
    ///

    pub fn identify(&self, text: &str) -> Vec<Match> {
        let mut json_data: Vec<Data> = load_data();

        self.filter_json_data(&mut json_data);

        let regexes = build_regexes(json_data);

        if self.file_support && is_file(text) {
            let all_matches = Arc::new(Mutex::new(Vec::<Match>::new()));
            let strings = read_file_to_strings(text);
            strings.par_iter().for_each(|text| {
                regexes.par_iter().for_each(|re| {
                    if let Ok(true) = re.compiled_regex.is_match(text) {
                        all_matches
                            .lock()
                            .unwrap()
                            .push(Match::new(text.to_owned(), re.data.clone()))
                    }
                })
            });
            Arc::try_unwrap(all_matches).unwrap().into_inner().unwrap()
        } else {
            // iter has almost same or sometimes better performance than par_iter for single text!
            let mut all_matches = Vec::<Match>::new();
            regexes.iter().for_each(|re| {
                if let Ok(true) = re.compiled_regex.is_match(text) {
                    all_matches.push(Match::new(text.to_owned(), re.data.clone()))
                }
            });
            all_matches
        }
    }

    /// This returns the first identification.
    ///
    /// Due to how data is stored, this means that the returned result has the highest `rarity`.
    ///
    /// # Arguments
    ///
    /// * text: &str - text which we want to identify
    ///
    /// # Examples
    ///
    /// ```
    /// let identifier = lemmeknow::Identify::default();
    /// let some_result = identifier.first_match("8888888888");
    /// let not_gonna_find = identifier.first_match("sjfjsafjlsj");
    ///  
    /// assert_eq!(some_result.unwrap().data.name, "Phone Number");
    /// assert!(not_gonna_find.is_none());
    /// ```
    ///
    pub fn first_match(&self, text: &str) -> Option<Match> {
        let mut json_data: Vec<Data> = load_data();

        self.filter_json_data(&mut json_data);

        for data in json_data {
            // only consider the regex which compiles!
            if let Ok(re) = Regex::new(&data.regex) {
                if let Ok(true) = re.is_match(text) {
                    return Some(Match::new(text.to_owned(), data));
                }
            }
        }

        None
    }
}

// Identifier implementation for wasm
#[cfg(target_arch = "wasm32")]
impl Identify {
    // There is no file system on the web, so we are not reading strings from file.
    // let the user perform the I/O and read the file, then pass the content of it.
    pub fn identify(&self, text: &[String]) -> Vec<Match> {
        let mut json_data: Vec<Data> = load_regexes();

        self.filter_json_data(&mut json_data);

        let regexes = build_regexes(json_data);
        let mut all_matches = Vec::<Match>::new();

        text.iter().for_each(|text| {
            regexes.iter().for_each(|re| {
                if let Ok(true) = re.compiled_regex.is_match(text) {
                    all_matches.push(Match::new(text.to_owned(), re.data.clone()))
                }
            })
        });

        all_matches
    }
}

// Output Implementation
// TODO: try #[inline]
impl Identify {
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
    /// use lemmeknow::Identify;
    /// let identifier = Identify::default();
    /// let result = identifier.identify("UC11L3JDgDQMyH8iolKkVZ4w");
    /// let result_in_json = Identify::to_json(&result);
    /// println!("{result_in_json}");
    /// ```
    ///
    pub fn to_json(result: &[Match]) -> String {
        serde_json::to_string_pretty(result).unwrap_or_default()
    }
}

// helper functions
// TODO: try #[inline]
#[cfg(not(target_arch = "wasm32"))]
fn is_file(name: &str) -> bool {
    if let Ok(s) = fs::metadata(name) {
        s.is_file()
    } else {
        false
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_file_to_strings(filename: &str) -> Vec<String> {
    let file = fs::read(filename).expect("File not found");

    let mut printable_text: Vec<String> = Vec::new();
    let mut buffer: Vec<u8> = Vec::new();
    let mut use_current_buffer = false;

    //we only need the human readable strings from the file.
    for character in file {
        if character.is_ascii_graphic() {
            // Doesn't consider whitespace as a graphic!
            use_current_buffer = true;
            buffer.push(character);
        } else if use_current_buffer {
            // If the char isn't ascii graphic, that means this is the end for our string which we are interested in
            // string with length less than 4 most likely won't be of our use.
            // If it has length more than 4, then push it to our `printable_text`
            if buffer.len() >= 4 {
                printable_text.push(
                    String::from_utf8(buffer.clone()).expect("failed to convert u8 to string"),
                );
            }

            // Clear the buffer so that current contents of it won't affect the next string.
            buffer.clear();
            // We set this to false because we don't want to use buffer until we get a ascii graphic!
            use_current_buffer = false;
        }
    }

    printable_text.push(String::from_utf8(buffer).expect("failed to convert u8 to string"));

    printable_text
}

fn load_data() -> Vec<Data> {
    // include_str! will include the data in binary
    // so we don't have to keep track of JSON file all the time after compiling the binary
    const DATA: &str = include_str!("../data/regex.json");
    serde_json::from_str::<Vec<Data>>(DATA).expect("Failed to parse JSON")
}

fn build_regexes(loaded_data: Vec<Data>) -> Vec<RegexData> {
    let mut regexes: Vec<RegexData> = Vec::new();
    for data in loaded_data {
        // Some regex from pywhat's regex.json might not work with fancy_regex
        // So we are just considering the ones which are valid.
        if let Ok(result) = Regex::new(&data.regex) {
            regexes.push(RegexData::new(result, data))
        } else {
            panic!("Can't compile {data:#?}");
        }
    }
    regexes
}
