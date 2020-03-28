extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("minigrep")
        .version("0.1.0")
        .author("some.dude")
        .about("learning some rust with a humble grep clone")
        .arg(Arg::with_name("no_color")
             .long("no-color")
             .takes_value(false)
             .help("no color"))
        .arg(Arg::with_name("files_with_match")
             .short("l")
             .long("files-with-match")
             .takes_value(false)
             .help("print files with match")
             .conflicts_with("files_without_match"))
        .arg(Arg::with_name("files_without_match")
             .short("L")
             .long("files-without-match")
             .takes_value(false)
             .help("print files without match")
             .conflicts_with("files_with_match"))
        .arg(Arg::with_name("query")
             .help("query")
             .required(true)
             .index(1))
        .arg(Arg::with_name("files")
             .help("files to search")
             .multiple(true)
             .required(true)
             .index(2))
        .get_matches();

    let query = matches.value_of("query").unwrap();
    let mut filenames = Vec::new();
    for f in matches.values_of("files").unwrap() {
        filenames.push(f.to_string());
    }

    let output_type = if matches.is_present("files_with_match") {
        minigrep::OutputType::FilesWithMatch
    } else if matches.is_present("files_without_match") {
        minigrep::OutputType::FilesWithoutMatch
    } else if matches.is_present("no_color") {
        minigrep::OutputType::Plain
    } else {
        minigrep::OutputType::Color
    };

    let config = minigrep::Config{
        query: query.to_string(),
        filenames: filenames,
        output_type: output_type,
    };
    minigrep::run(config);
}

