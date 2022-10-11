use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Default)]
struct Example {
    #[serde(rename(deserialize = "Valid"))]
    valid: Vec<String>,
    #[serde(rename(deserialize = "Invalid"), default)]
    invalid: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TestCase {
    #[serde(rename(deserialize = "Name"))]
    name: String,
    #[serde(rename(deserialize = "Regex"))]
    regex: String,
    plural_name: bool,
    #[serde(rename(deserialize = "Description"))]
    description: Option<String>,
    #[serde(rename(deserialize = "Rarity"))]
    rarity: f64,
    #[serde(rename(deserialize = "URL"))]
    url: Option<String>,
    #[serde(rename(deserialize = "Tags"))]
    tags: Vec<String>,
    #[serde(rename(deserialize = "Examples"), default)]
    examples: Example,
}

#[test]
fn validate_regex_examples() {
    let path = format!("{}/src/data/regex.json", env!("CARGO_MANIFEST_DIR"));
    let data: String = fs::read_to_string(path).unwrap();

    let test_cases: Vec<TestCase> = serde_json::from_str(&data).unwrap();
    for test_case in &test_cases {
        let re = Regex::new(&test_case.regex);
        match re {
            Ok(re) => {
                for example in &test_case.examples.valid {
                    let matched = re.is_match(example);
                    if !matched {
                        println!(
                            "SHOULD MATCH for {}\nexample: {}\nregex: {}",
                            test_case.name, example, test_case.regex
                        );
                    }
                    assert!(matched);
                }

                for example in &test_case.examples.invalid {
                    let matched = re.is_match(example);
                    if matched {
                        println!(
                            "INVALID MATCH for {}\nexample: {}\nregex: {}",
                            test_case.name, example, test_case.regex
                        );
                    }
                    assert!(!matched);
                }
            }
            Err(re) => {
                println!(
                    "Failed to compile regex for {} due to {:#?}",
                    test_case.name, re
                );
            }
        };
    }
}
