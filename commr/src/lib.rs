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
                .required(true),
        )
        .arg(
            Arg::with_name("file2")
                .value_name("FILE2")
                .help("Input file 2")
                .required(true),
        )
        .arg(
            Arg::with_name("suppress-col1")
                .help("Suppress printing of column 1")
                .short("1")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("suppress-col2")
                .help("Suppress printing of column 2")
                .short("2")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("suppress-col3")
                .help("Suppress printing of column 3")
                .short("3")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("insensitive")
                .help("Case insensitive comparison of lines")
                .short("i")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("delimiter")
                .value_name("DELIM")
                .help("Output delimiter")
                .short("d")
                .long("output-delimiter")
                .default_value("\t"),
        )
        .get_matches();

    let file1 = matches.value_of("file1").unwrap().to_string();
    let file2 = matches.value_of("file2").unwrap().to_string();
    let suppress_col1 = matches.is_present("suppress-col1");
    let suppress_col2 = matches.is_present("suppress-col2");
    let suppress_col3 = matches.is_present("suppress-col3");
    let insensitive = matches.is_present("insensitive");
    let delimiter = matches.value_of("delimiter").unwrap().to_string();

    Ok(Config {
        file1,
        file2,
        suppress_col1,
        suppress_col2,
        suppress_col3,
        insensitive,
        delimiter,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
