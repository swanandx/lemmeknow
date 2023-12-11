use fancy_regex::Regex as Fancy;
use regex::Regex;
use serde::Deserialize;
use std::fmt::Write;
use std::{env, fs, path::Path};

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
    #[serde(rename(deserialize = "Exploit"))]
    exploit: Option<String>,
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
        d.boundaryless = Fancy::new(r"(?<!\\)\^(?![^\[\]]*(?<!\\)\])")
            .expect("can't compile for boundaryless")
            .replace(&d.regex, "")
            .to_string();
        d.boundaryless = Fancy::new(r"(?<!\\)\$(?![^\[\]]*(?<!\\)\])")
            .expect("can't compile for boundaryless")
            .replace(&d.boundaryless, "")
            .to_string();
    });

    data.retain(|r| Regex::new(&r.regex).is_ok() && Regex::new(&r.boundaryless).is_ok());

    let mut data_str = format!("{:?}", data);
    // we want reference to [], i.e. &[]
    data_str = data_str.replace("tags: [", "tags: &[");

    let regex_str: String = data.iter().fold(String::new(), |mut output, d| {
        let _ = write!(
            output,
            r#"Lazy::new(|| Regex::new({:?}).unwrap()),"#,
            d.regex
        );
        output
    });

    let boundaryless_regex_str: String = data.iter().fold(String::new(), |mut output, d| {
        let _ = write!(
            output,
            r#"Lazy::new(|| Regex::new({:?}).unwrap()),"#,
            d.boundaryless
        );
        output
    });

    let count = data.len();
    let final_str = format!(
        r#"
        const DATA: [Data; {count}] = {data_str};
    "#
    );
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("data.rs");
    fs::write(dest_path, final_str).unwrap();

    let final_str = format!(
        r#"
        static REGEX: [Lazy<Regex>; {count}] = [{regex_str}];
        static BOUNDARYLESS_REGEX: [Lazy<Regex>; {count}] = [{boundaryless_regex_str}];
    "#
    );
    let regex_dest_path = Path::new(&out_dir).join("regex_data.rs");
    fs::write(regex_dest_path, final_str).unwrap();
}
