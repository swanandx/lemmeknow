//! For identifying text / analyzing files

use fancy_regex::Regex;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::sync::{Arc, Mutex};
use std::{fs, str};

use crate::Data;
use crate::Matches;

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
    /// Keep Data having minimun Rarity of supplied `min_rarity`
    pub min_rarity: Option<f32>,
    /// Keep Data having maximun Rarity of supplied `max_rarity`
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
                data.Regex = Regex::new(r"(?<!\\)\^(?![^\[\]]*(?<!\\)\])")
                    .unwrap()
                    .replace(&data.Regex, "")
                    .to_string();
                data.Regex = Regex::new(r"(?<!\\)\$(?![^\[\]]*(?<!\\)\])")
                    .unwrap()
                    .replace(&data.Regex, "")
                    .to_string();
            }
        }

        if let Some(min_rarity) = self.min_rarity {
            json_data.retain(|x| x.Rarity >= min_rarity)
        }
        if let Some(max_rarity) = self.max_rarity {
            json_data.retain(|x| x.Rarity <= max_rarity)
        }
        if !self.tags.is_empty() {
            json_data.retain(|x| self.tags.iter().any(|y| x.Tags.contains(y)))
        }
        if !self.exclude_tags.is_empty() {
            json_data.retain(|x| self.exclude_tags.iter().any(|y| !x.Tags.contains(y)))
        }
    }
}

// Identifier implementation
impl Identify {
    /// Identify the given text.
    ///
    /// This will read strings from file with text as filename if `file_support` is `true` and the file exists
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
    /// assert_eq!(result[0].data.Name, "YouTube Channel ID");
    /// ```
    ///
    pub fn identify(&self, text: &str) -> Vec<Matches> {
        let mut json_data: Vec<Data> = load_regexes();

        self.filter_json_data(&mut json_data);

        let regexes = build_regexes(json_data);
        let all_matches = Arc::new(Mutex::new(Vec::<Matches>::new()));

        if self.file_support && is_file(text) {
            let strings = read_file_to_strings(text);
            strings.par_iter().for_each(|text| {
                regexes.par_iter().for_each(|re| {
                    if re.compiled_regex.is_match(text).unwrap() {
                        all_matches
                            .lock()
                            .unwrap()
                            .push(Matches::new(text.to_owned(), re.data.clone()))
                    }
                })
            });
        } else {
            regexes.par_iter().for_each(|re| {
                if re.compiled_regex.is_match(text).unwrap() {
                    all_matches
                        .lock()
                        .unwrap()
                        .push(Matches::new(text.to_owned(), re.data.clone()))
                }
            })
        }

        Arc::try_unwrap(all_matches).unwrap().into_inner().unwrap()
    }
}

// Output Implementation
impl Identify {
    /// Convert [`Vec<Matches>`] to JSON
    ///
    /// Returns prettified JSON string.
    ///
    /// Helpful if you want to convert possible identifications to JSON
    /// for using in web APIs or something else.
    ///
    /// # Arguments
    ///
    /// * result: &[Matches] - Reference to `Vec<Matches>`.
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
    pub fn to_json(result: &[Matches]) -> String {
        serde_json::to_string_pretty(result).unwrap_or_default()
    }
}

// helper functions
fn is_file(name: &str) -> bool {
    fs::metadata(name).is_ok()
}

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
            // If the char isn't ascii graphic, that means this is the end for our string which we are intresed in
            // string with length less than 4 most likely won't be of our use.
            // If it has length more than 4, then push it to our `printable_text`
            if buffer.len() >= 4 {
                printable_text.push(String::from_utf8(buffer.clone()).unwrap());
            }

            // Clear the buffer so that current contents of it won't affect the next string.
            buffer.clear();
            // We set this to false because we don't want to use buffer until we get a ascii graphic!
            use_current_buffer = false;
        }
    }

    printable_text.push(String::from_utf8(buffer).unwrap());

    printable_text
}

fn load_regexes() -> Vec<Data> {
    // include_str! will include the data in binary
    // so we don't have to keep track of JSON file all the time after compiling the binary
    let data = include_str!("../data/regex.json");
    serde_json::from_str::<Vec<Data>>(data).expect("Failed to parse JSON")
}

fn build_regexes(loaded_data: Vec<Data>) -> Vec<RegexData> {
    let mut regexes: Vec<RegexData> = Vec::new();
    for data in loaded_data {
        // Some regex from pywhat's regex.json might not work with fancy_regex
        // So we are just considering the ones which are valid.
        if let Ok(result) = Regex::new(&data.Regex) {
            regexes.push(RegexData::new(result, data))
        } else {
            panic!("Can't compile {data:#?}");
        }
    }
    regexes
}
