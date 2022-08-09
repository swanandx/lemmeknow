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
 * lemmeknow = { version = "0.6", default-features = false }
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

use serde::Serialize;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "cli")]
pub mod output;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "cli")]
pub use self::output::PrintMode;

// TODO: Try not to use String
/// structure for parsing data from JSON file.
#[derive(Serialize, Debug, Clone)]
pub struct Data {
    pub name: &'static str,
    pub regex: &'static str,
    boundaryless: &'static str,
    pub plural_name: bool,
    pub description: Option<&'static str>,
    pub rarity: f32,
    pub url: Option<&'static str>,
    pub tags: &'static [&'static str],
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
