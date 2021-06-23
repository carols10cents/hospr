use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    entries: Vec<String>,
    long: bool,
    all: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("lsr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust tail")
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("Show all files, including hidden ones"),
        )
        .arg(
            Arg::with_name("long")
                .short("l")
                .long("long")
                .help("Display the long format"),
        )
        .arg(
            Arg::with_name("entries")
                .value_name("ENTRIES")
                .help("Input file(s)")
                .required(true)
                .min_values(1)
                .default_value("."),
        )
        .get_matches();

    Ok(Config {
        entries: matches.values_of_lossy("entries").unwrap(),
        long: matches.is_present("long"),
        all: matches.is_present("all"),
    })
}
