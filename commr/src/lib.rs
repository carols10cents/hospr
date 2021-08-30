use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file1: String,
    file2: String,
    suppress_col1: bool,
    suppress_col2: bool,
    suppress_col3: bool,
    insensitive: bool,
    delimiter: String,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("commr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust comm")
        .arg(
            Arg::with_name("file1")
                .value_name("FILE1")
                .help("Input file 1")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("file2")
                .value_name("FILE2")
                .help("Input file 2")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("suppress_col1")
                .short("1")
                .value_name("COL1")
                .takes_value(false)
                .help("Suppress printing of column 1"),
        )
        .arg(
            Arg::with_name("suppress_col2")
                .short("2")
                .value_name("COL2")
                .takes_value(false)
                .help("Suppress printing of column 2"),
        )
        .arg(
            Arg::with_name("suppress_col3")
                .short("3")
                .value_name("COL3")
                .takes_value(false)
                .help("Suppress printing of column 3"),
        )
        .arg(
            Arg::with_name("insensitive")
                .short("i")
                .value_name("INSENSITIVE")
                .takes_value(false)
                .help("Case insensitive comparison of lines"),
        )
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("output-delimiter")
                .value_name("DELIM")
                .help("Output delimiter")
                .takes_value(true),
        )
        .get_matches();

    Ok(Config {
        file1: matches.value_of("file1").unwrap().to_string(),
        file2: matches.value_of("file2").unwrap().to_string(),
        suppress_col1: matches.is_present("suppress_col1"),
        suppress_col2: matches.is_present("suppress_col2"),
        suppress_col3: matches.is_present("suppress_col3"),
        insensitive: matches.is_present("insensitive"),
        delimiter: matches.value_of("delimiter").unwrap_or("\t").to_string(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
