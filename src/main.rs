use clap::Parser;
use lemmeknow::{Identifier, PrintMode};

#[derive(Parser)]
#[clap(
    author,
    version,
    about,
    long_about = "https://github.com/swanandx/lemmeknow"
)]
struct Args {
    /// Text which you want to identify
    text: String,
    /// Output in JSON format
    #[clap(short, long)]
    json: bool,
    /// Minimum Rarity
    #[clap(long = "min", default_value_t = 0.1)]
    min_rarity: f32,
    /// Maximum Rarity
    #[clap(long = "max", default_value_t = 1.0)]
    max_rarity: f32,
    /// Only identify text, Do not scan file
    #[clap(short, long)]
    text_only: bool,
    /// Disable boundaryless mode, if you are getting lot of false positives
    #[clap(short, long)]
    boundary: bool,
    /// Include matches with these tags
    #[clap(short, long, value_delimiter(','))]
    include: Option<Vec<String>>,
    /// Exclude matches having these tags
    #[clap(short, long, value_delimiter(','))]
    exclude: Option<Vec<String>>,
    /// Print output with more details
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let identifier = Identifier::default()
        .min_rarity(args.min_rarity)
        .max_rarity(args.max_rarity)
        .include_tags(&args.include.unwrap_or_default())
        .exclude_tags(&args.exclude.unwrap_or_default())
        .boundaryless(!args.boundary) // boundaryless is true if boundary is false, and vice-versa
        .file_support(!args.text_only); // file_support is true if text_only is false, and vice-versa

    let result = identifier.identify(&args.text);
    if args.json {
        let result_in_json = Identifier::to_json(&result);
        println!("{result_in_json}");
    } else {
        let printer = if args.verbose {
            PrintMode::Verbose
        } else {
            PrintMode::Normal
        };
        printer.print(&result);
    }
}
