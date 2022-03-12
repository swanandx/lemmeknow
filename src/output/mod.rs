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
/// use lemmeknow::{Identify, to_json};
/// let identifier = Identify::default();
/// let result = identifier.identify("UC11L3JDgDQMyH8iolKkVZ4w");
/// let result_in_json = to_json(&result);
/// println!("{result_in_json}");
/// ```
///

pub fn to_json(result: &[Matches]) -> String {
    serde_json::to_string_pretty(result).unwrap()
}

/// Modes defining how the output shall be printed
pub enum PrintMode {
    Normal,
    Verbose,
}

impl PrintMode {
    /// Print `Vec<Matches>` in a tabular form.
    ///
    /// Use this if you want to print the possible identification in terminal
    /// with a pretty table.
    ///
    /// * `PrintMode::Normal` will print "Matched text", "Identified as" and "Description" columns.
    /// * `PrintMode::Verbose` will print "Rarity" and "Tags" along with other columns.
    ///
    /// # Arguments
    ///
    /// * result: &[Matches] - Reference to `Vec<Matches>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lemmeknow::{Identify, PrintMode};
    /// let identifier = Identify::default();
    /// let result = identifier.identify("UC11L3JDgDQMyH8iolKkVZ4w");
    /// let printer = PrintMode::Normal;
    ///
    /// printer.print(&result);
    /// ```
    ///
    pub fn print(self, result: &[Matches]) {
        pretty_print(result, self)
    }
}

fn pretty_print(result: &[Matches], output_format: PrintMode) {
    let mut table = Table::new();
    let mut headers = vec![
        Cell::new("Matched text")
            .add_attribute(Attribute::Bold)
            .fg(Color::Magenta),
        Cell::new("Identified as")
            .add_attribute(Attribute::Bold)
            .fg(Color::Magenta),
        Cell::new("Description")
            .add_attribute(Attribute::Bold)
            .fg(Color::Magenta),
    ];

    if let PrintMode::Verbose = output_format {
        headers.extend([
            Cell::new("Rarity")
                .add_attribute(Attribute::Bold)
                .fg(Color::Magenta),
            Cell::new("Tags")
                .add_attribute(Attribute::Bold)
                .fg(Color::Magenta),
        ]);
    }

    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        // .set_table_width(80)
        .set_header(headers);

    if result.is_empty() {
        println!("\x1b[0;31mNo Possible Identifications :(\x1b[0m");
    } else {
        println!("\x1b[0;32mFound Possible Identifications :)\x1b[0m");

        result.iter().for_each(|item| {
            let description = match (item.data.Description.as_ref(), item.data.URL.as_ref()) {
                (Some(des), Some(url)) => format!("{des}\n Check URL: {url}{}", &item.text),
                (Some(des), None) => des.to_string(),
                (None, Some(url)) => format!("URL:\n {url}{}", &item.text),
                (None, None) => "None".to_string(),
            };

            let mut row = vec![
                Cell::new(&item.text),
                Cell::new(&item.data.Name),
                Cell::new(description),
            ];

            if let PrintMode::Verbose = output_format {
                row.extend([
                    Cell::new(&item.data.Rarity),
                    Cell::new(&item.data.Tags.join(", ")),
                ]);
            }

            table.add_row(row);
        });

        println!("{table}");
    }
}
