use lemmeknow::{pprint, to_json, Identify};
use clap::Parser;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Text which you want to identify
    text: String,
    /// Output in JSON format
    #[clap(short, long)]
    json: bool,
    /// Minimum Rarity
    #[clap(long="min")]
    min_rarity: Option<f32>,
    /// Maximum Rarity
    #[clap(long="max")]
    max_rarity: Option<f32>,
    /// File support
    #[clap(short, long)]
    file_support: bool,
    /// Match boundaryless regex
    #[clap(short, long)]
    boundaryless: bool,
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
            .file_support(true)
            .exclude_tags(&[String::from("URL")]);
        let result = identifier.identify(&args.text);
        if args.json {
            println!("{}", to_json(&result));
        } else {
            println!("{}", BANNER);
            pprint(&result);
        }
}
