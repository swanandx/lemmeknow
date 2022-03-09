use fancy_regex::Regex;
use std::{fs, str};

use crate::Data;
use crate::Matches;

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

impl Default for Identify {
    fn default() -> Self {
        Self::new()
    }
}

impl Identify {
    fn new() -> Identify {
        Identify {
            min_rarity: None,
            max_rarity: None,
            tags: Vec::new(),
            exclude_tags: Vec::new(),
            boundaryless: false,
            file_support: false,
        }
    }

    pub fn min_rarity(mut self, rarity: f32) -> Identify {
        self.min_rarity = Some(rarity);
        self
    }

    pub fn max_rarity(mut self, rarity: f32) -> Identify {
        self.max_rarity = Some(rarity);
        self
    }

    pub fn include_tags(mut self, tags: &[String]) -> Identify {
        self.tags.extend_from_slice(tags);
        self
    }

    pub fn exclude_tags(mut self, tags: &[String]) -> Identify {
        self.exclude_tags.extend_from_slice(tags);
        self
    }

    pub fn boundaryless(mut self, boundaryless: bool) -> Identify {
        self.boundaryless = boundaryless;
        self
    }
    pub fn file_support(mut self, support: bool) -> Identify {
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

        let mut strings: Vec<String> = Vec::<String>::new();

        if self.file_support && is_file(text) {
            strings.extend(read_file_to_strings(text));
        } else {
            strings.push(text.to_string());
        }

        let regexes = build_regexes(json_data);
        let mut all_matches = Vec::<Matches>::new();

        for text in &strings {
            for i in &regexes {
                if i.0.is_match(text).unwrap() {
                    all_matches.push(Matches::new(text.to_string(), i.1.clone()));
                }
            }
        }

        all_matches
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
    let mut current_buffer = false;

    //we only need the human readable strings from the file.
    for character in file {
        if character.is_ascii_graphic() {
            // Doesn't consider whitespace as a graphic!
            current_buffer = true;
            buffer.push(character);
        } else if current_buffer {
            //text with length smaller than 4 most likely won't be of our use.
            if buffer.len() >= 4 {
                printable_text.push((str::from_utf8(&buffer).unwrap()).to_string());
            }

            buffer.clear();
            current_buffer = false;
        }
    }

    printable_text.push((str::from_utf8(&buffer).unwrap()).to_string());

    printable_text
}

fn load_regexes() -> Vec<Data> {
    // include_str! will include the data in binary
    // so we don't have to keep track of JSON file all the time after compiling the binary
    let data = include_str!("../data/regex.json");
    serde_json::from_str(data).expect("Failed to parse JSON")
}

fn build_regexes(loaded_data: Vec<Data>) -> Vec<(Regex, Data)> {
    let mut regexes: Vec<(Regex, Data)> = Vec::new();
    for data in loaded_data {
        // Some regex from pywhat's regex.json might not work with fancy_regex
        // So we are just considering the ones which are valid.
        if let Ok(result) = Regex::new(&data.Regex) {
            regexes.push((result, data))
        } else {
            panic!("Can't compile {data:#?}");
        }
    }
    regexes
}
