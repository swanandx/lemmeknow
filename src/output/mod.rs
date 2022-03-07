use crate::Matches;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

/// Convert `Vec<Matches>` to JSON
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
/// use lemmeknow::{Identify, pprint};
/// let identifier = Identify::default();
/// let result = identifier.identify("UC11L3JDgDQMyH8iolKkVZ4w");
/// pprint(&result);
/// ```
///

pub fn to_json(result: &[Matches]) -> String {
    serde_json::to_string_pretty(result).unwrap()
}

/// Pretty print `Vec<Matches>` in a tabular form.
///
/// Use this if you want to print the possible identification in terminal
/// with a pretty table.
///
/// # Arguments
///
/// * result: &[Matches] - Reference to `Vec<Matches>`.
///
/// # Examples
///
/// ```
/// use lemmeknow::{Identify, pprint};
/// let identifier = Identify::default();
/// let result = identifier.identify("UC11L3JDgDQMyH8iolKkVZ4w");
/// pprint(&result);
/// ```
///

pub fn pprint(result: &[Matches]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        // .set_table_width(80)
        .set_header(vec![
            Cell::new("Matched text")
                .add_attribute(Attribute::Bold)
                .fg(Color::Magenta),
            Cell::new("Identified as")
                .add_attribute(Attribute::Bold)
                .fg(Color::Magenta),
            Cell::new("Description")
                .add_attribute(Attribute::Bold)
                .fg(Color::Magenta),
        ]);
    if result.is_empty() {
        println!("\x1b[0;31mNo Possible Identifications :(\x1b[0m");
    } else {
        println!("\x1b[0;32mFound Possible Identifications :)\x1b[0m");
        for item in result.iter() {
            let description = match (item.data.Description.as_ref(), item.data.URL.as_ref()) {
                (Some(des), Some(url)) => format!("{}\n Check URL: {}{}", des, url, &item.text),
                (Some(des), None) => des.to_string(),
                (None, Some(url)) => format!("URL:\n {}{}", url, &item.text),
                (None, None) => "None".to_string(),
            };

            table.add_row(vec![
                Cell::new(&item.text),
                Cell::new(&item.data.Name),
                Cell::new(description),
            ]);
        }
        println!("{}", table);
    }
}
