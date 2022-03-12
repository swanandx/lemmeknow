use clap::Parser;
use lemmeknow::{pprint, to_json, Identify};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
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
    /// Disable boundaryless regex mode
    #[clap(short, long)]
    boundary: bool,
    /// Only include matches with these tags
    #[clap(short, long, value_delimiter(','))]
    include: Option<Vec<String>>,
    /// Exclude matches having these tags
    #[clap(short, long, value_delimiter(','))]
    exclude: Option<Vec<String>>,
}

const BANNER: &str = r#" _                               _                        
| |                             | |                       
| | ___ _ __ ___  _ __ ___   ___| | ___ __   _____      __
| |/ _ \ '_ ` _ \| '_ ` _ \ / _ \ |/ / '_ \ / _ \ \ /\ / /
| |  __/ | | | | | | | | | |  __/   <| | | | (_) \ V  V / 
|_|\___|_| |_| |_|_| |_| |_|\___|_|\_\_| |_|\___/ \_/\_/  
                                                          
<https://www.github.com/swanandx/lemmeknow>
                                                          "#;

fn main() {
    let args = Args::parse();

    let identifier = Identify::default()
        .min_rarity(args.min_rarity)
        .max_rarity(args.max_rarity)
        .include_tags(&args.include.unwrap_or_default())
        .exclude_tags(&args.exclude.unwrap_or_default())
        .boundaryless(!args.boundary) // boundaryless is true if boundary is false, and vice-versa
        .file_support(!args.text_only); // file_support is true if text_only is false, and vice-versa

    let result = identifier.identify(&args.text);
    if args.json {
        let result_in_json = to_json(&result);
        println!("{result_in_json}");
    } else {
        println!("{BANNER}");
        pprint(&result);
    }
}
