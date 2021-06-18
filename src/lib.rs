use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("me")
        .about("Rust head")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("Number of bytes")
                .long("bytes")
                .short("c")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .help("Number of lines")
                .long("lines")
                .short("n")
                .takes_value(true)
                .default_value("10"),
        )
        .get_matches();

    let files = matches.values_of_lossy("file").unwrap();
    let bytes = matches.value_of("bytes").map(|b| b.parse().unwrap());
    let lines = matches.value_of("lines").unwrap().parse().unwrap();

    Ok(Config {
        lines,
        bytes,
        files,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
