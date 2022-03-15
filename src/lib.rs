/*!
 * lemmeknow can be used for identifying mysterious text
 * or to analyze hard-coded strings from captured network packets, malwares, or just about anything.
 *
 * # Usage
 *
 * If you want to use it as a library and do not want to pretty print output as table
 * then set `default-features=false` in your `Cargo.toml`:
 *
 * ```toml
 * [dependencies]
 * lemmeknow = { version = "0.3.0", default-features = false }
 * ```
 *
 * OR by using github repositoy:
 *
 * ```toml
 * [dependencies]
 * lemmeknow = { git = "https://github.com/swanandx/lemmeknow", default-features = false }
 * ```
 *
 * # Example: To identify a text
 *
 * Let us say we want to identify a text and then get the output as pretty JSON
 *
 * ```rust
 * use lemmeknow::Identify;
 * let identifier = Identify::default();
 * let result = identifier.identify("UC11L3JDgDQMyH8iolKkVZ4w");
 * let result_in_json = Identify::to_json(&result);
 * println!("{result_in_json}");
 * ```
 *
 * */

pub mod identifier;
pub use self::identifier::Identify;

use serde::{Deserialize, Serialize};

#[cfg(feature = "cli")]
pub mod output;

#[cfg(feature = "cli")]
pub use self::output::PrintMode;

// TODO: Try not to use String
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
