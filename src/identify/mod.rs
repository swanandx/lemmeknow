use fancy_regex::Regex;
use once_cell::sync::Lazy;
use std::{fs, str};

use crate::Data;
use crate::Matches;

static DATA: Lazy<Vec<Data>> = Lazy::new(load_regex);
static REGEXES: Lazy<Vec<Regex>> = Lazy::new(build_regexes);

/// Determine if the given text is a file or a string and then identifies accordingly.
/// 
/// This will call `analyze_file` if the argument is a file, else, will call `identify_text` otherwise.
/// 
/// # Arguments
///
/// * text: &str - Text or path to file which we want to identify.
///
/// # Examples
///
/// ```
/// use lemmeknow::what_is;
/// let result = what_is("0x52908400098527886E0F7030069857D2E4169EE7");
/// assert_eq!(result[0].data.Name, "Ethereum (ETH) Wallet Address");
/// ```


pub fn what_is(text: &str) -> Vec<Matches> {
    if is_file(text) {
        analyze_file(text)
    } else {
        identify_text(text)
    }
}


/// Identify the given text.
/// 
/// Prefer using this if you do not want to analyze files.
/// e.g. in a web API, you might just want to deal with text rather than files.
///
/// # Arguments
///
/// * text: &str - Mysterious text which we want to identify.
///
/// # Examples
///
/// ```
/// use lemmeknow::identify_text;
/// let result = identify_text("UC11L3JDgDQMyH8iolKkVZ4w");
/// assert_eq!(result[0].data.Name, "YouTube Channel ID");
/// ```
///


pub fn identify_text(text: &str) -> Vec<Matches> {
    let mut all_matches = Vec::<Matches>::new();

    for (i, item) in (&*DATA).iter().enumerate() {
        if (REGEXES[i]).is_match(text).unwrap() {
            all_matches.push(Matches::new(text.to_string(), item.clone()));
        }
    }

    all_matches
}

/// Analyze and identify strings present in a file.
/// 
///
/// # Arguments
///
/// * filename: &str - Filename of the file from which we want to identify strings.
///
/// # Examples
///
/// ```
/// use lemmeknow::analyze_file;
/// # let path_to_file = file!();
/// let result = analyze_file(path_to_file);
/// ```
/// Use this if you know the argument is going to be a valid file which you have permission to read.
/// 
/// # Panics
///  
/// Panics if failed to read file.
/// It can be due to file not found, it is directory or permission denied to read the file.
///
pub fn analyze_file(filename: &str) -> Vec<Matches> {
    let mut all_matches = Vec::<Matches>::new();

    let strings = read_file_to_strings(filename);

    for text in &strings {
        all_matches.extend(identify_text(text));
    }

    all_matches
}


// helper functions

fn is_file(name: &str) -> bool {
    fs::metadata(name).is_ok()
}

fn read_file_to_strings(filename: &str) -> Vec<String> {
    let file = fs::read(filename).expect("File not found");

    let mut printable_text: Vec<Vec<u8>> = Vec::new();
    let mut buffer: Vec<u8> = Vec::new();
    let mut current_buffer = false;

    //we only need the human readable strings from the file.
    for charecter in file {
        if charecter.is_ascii_graphic() {
            current_buffer = true;
            buffer.push(charecter);
        } else if current_buffer {
            //text with length smaller than 4 most likely won't be of our use.
            if buffer.len() >= 4 {
                printable_text.push(buffer.clone());
            }

            buffer.clear();
            current_buffer = false;
        }
    }

    printable_text.push(buffer);

    let mut result: Vec<String> = Vec::new();

    for item in &printable_text {
        result.push((str::from_utf8(item).unwrap()).to_string())
    }

    result
}

fn build_regexes() -> Vec<Regex> {
    let mut regexes: Vec<Regex> = Vec::new();
    for data in &*DATA {
        regexes.push(Regex::new(&data.Regex).unwrap());
    }
    regexes
}

fn load_regex() -> Vec<Data> {
    // include_str! will include the data in binary
    // so we don't have to keep track of JSON file all the time after compiling the binary
    // let data = fs::read_to_string(filename).expect("JSON file not found.");
    let data = include_str!("../data/regex.json");
    serde_json::from_str(data).expect("Failed to parse JSON")
}
