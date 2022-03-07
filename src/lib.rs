//! # lemmeknow
//!
//! Identify any mysterious text or analyze strings from a file, just ask `lemmeknow`.

pub mod identifier;
pub mod output;

use serde::{Deserialize, Serialize};

pub use self::identifier::Identify;
pub use self::output::pprint;
pub use self::output::to_json;

/// structure for parsing data from JSON file.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub Name: String,
    pub Regex: String,
    pub plural_name: bool,
    pub Description: Option<String>,
    pub Rarity: f32,
    pub URL: Option<String>,
    pub Tags: Vec<String>,
}

/// structure containing the text and it's possible identification.
#[derive(Serialize, Debug)]
pub struct Matches {
    pub text: String,
    pub data: Data,
}

impl Matches {
    pub fn new(text: String, data: Data) -> Matches {
        Matches { text, data }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
