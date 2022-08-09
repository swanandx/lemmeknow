use std::{env, fs, path::Path};

use fancy_regex::Regex;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Data {
    #[serde(rename(deserialize = "Name"))]
    name: String,
    #[serde(rename(deserialize = "Regex"))]
    regex: String,
    #[serde(skip_deserializing)]
    boundaryless: String,
    plural_name: bool,
    #[serde(rename(deserialize = "Description"))]
    description: Option<&'static str>,
    #[serde(rename(deserialize = "Rarity"))]
    rarity: f32,
    #[serde(rename(deserialize = "URL"))]
    url: Option<&'static str>,
    #[serde(rename(deserialize = "Tags"))]
    tags: Vec<&'static str>,
}

fn main() {
    let mut data: Vec<Data> = serde_json::from_str(include_str!("./src/data/regex.json")).unwrap();

    data.iter_mut().for_each(|d| {
        d.boundaryless = Regex::new(r"(?<!\\)\^(?![^\[\]]*(?<!\\)\])")
            .expect("can't compile for boundaryless")
            .replace(&d.regex, "")
            .to_string();
        d.boundaryless = Regex::new(r"(?<!\\)\$(?![^\[\]]*(?<!\\)\])")
            .expect("can't compile for boundaryless")
            .replace(&d.boundaryless, "")
            .to_string();
    });

    let mut out_data_str = format!("{:?}", data);

    // we want reference to [], i.e. &[]
    out_data_str = out_data_str.replace("tags: [", "tags: &[");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("regex_data.rs");
    fs::write(
        &dest_path,
        format!("const DATA: [Data; {}] = {};", data.len(), out_data_str),
    )
    .unwrap();
}
