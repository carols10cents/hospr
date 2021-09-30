use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiters: Vec<String>,
    serial: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("paster")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust paste")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("serial")
                .short("s")
                .long("serial")
                .takes_value(false)
                .help("Concatenate lines of each file serially"),
        )
        .arg(
            Arg::with_name("delimiter")
                .value_name("DELIMITER")
                .short("d")
                .long("delimiter")
                .default_value("\\t")
                .help("Delimiter"),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let delimiters: Vec<_> = matches
        .value_of_lossy("delimiter")
        .unwrap()
        .chars()
        .map(|c| c.to_string())
        .collect();
    let serial = matches.is_present("serial");

    Ok(Config {
        files,
        delimiters,
        serial,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
