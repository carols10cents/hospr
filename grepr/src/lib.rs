use clap::{App, Arg};
use regex::{Regex, RegexBuilder};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;
#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
    count: bool,
    invert_match: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("grepr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust grep")
        .arg(
            Arg::with_name("pattern")
                .value_name("PATTERN")
                .help("Search pattern")
                .required(true),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("count")
                .help("Count occurrences")
                .short("c")
                .long("count")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("insensitive")
                .help("Case-insensitive")
                .short("i")
                .long("insensitive")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("invert-match")
                .help("Invert match")
                .short("v")
                .long("invert-match")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("recursive")
                .help("Recursive search")
                .short("r")
                .long("recursive")
                .takes_value(false),
        )
        .get_matches();

    let raw_pattern = matches.value_of("pattern").unwrap();
    let pattern = match Regex::new(raw_pattern) {
        Ok(p) => p,
        Err(_) => return Err(format!("Invalid pattern \"{}\"", raw_pattern).into()),
    };
    let files = matches.values_of_lossy("files").unwrap();
    let recursive = matches.is_present("recursive");
    let count = matches.is_present("count");
    let invert_match = matches.is_present("invert-match");

    Ok(Config {
        pattern,
        files,
        recursive,
        count,
        invert_match,
    })
}
pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
