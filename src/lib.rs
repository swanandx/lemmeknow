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
 * lemmeknow = { version = "0.5", default-features = false }
 * ```
 *
 * OR by using github repository:
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
 * use lemmeknow::Identifier;
 * let identifier = Identifier::default();
 * let result = identifier.identify("UC11L3JDgDQMyH8iolKkVZ4w");
 * let result_in_json = Identifier::to_json(&result);
 * println!("{result_in_json}");
 * ```
 *
 * */

pub mod identifier;
pub use self::identifier::Identifier;

use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "cli")]
pub mod output;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "cli")]
pub use self::output::PrintMode;

// TODO: Try not to use String
/// structure for parsing data from JSON file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Regex")]
    pub regex: String,
    pub plural_name: bool,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Rarity")]
    pub rarity: f32,
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Vec<String>,
}

/// structure containing the text and it's possible identification.
#[derive(Serialize, Debug)]
pub struct Match {
    pub text: String,
    pub data: Data,
}

impl Match {
    pub fn new(text: String, data: Data) -> Match {
        Match { text, data }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
