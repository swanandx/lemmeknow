use clap::{clap_app, crate_authors, crate_description, crate_name, crate_version};
use lemmeknow::{pprint, to_json, what_is};

const BANNER: &str = r#" _                               _                        
| |                             | |                       
| | ___ _ __ ___  _ __ ___   ___| | ___ __   _____      __
| |/ _ \ '_ ` _ \| '_ ` _ \ / _ \ |/ / '_ \ / _ \ \ /\ / /
| |  __/ | | | | | | | | | |  __/   <| | | | (_) \ V  V / 
|_|\___|_| |_| |_|_| |_| |_|\___|_|\_\_| |_|\___/ \_/\_/  
                                                          
<https://www.github.com/swanandx/lemmeknow>
                                                          "#;

fn main() {
    let matches = clap_app!((crate_name!()) =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg TEXT: +takes_value +required "Mysterious text which you want to identify")
        (@arg JSON: -j --json "Output in JSON format")
    )
    .get_matches();

    if let Some(text) = matches.value_of("TEXT") {
        let result = what_is(text);
        if matches.is_present("JSON") {
            println!("{}", to_json(&result));
        } else {
            println!("{}", BANNER);
            pprint(&result);
        }
    }
}
