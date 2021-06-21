use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
    str::FromStr,
};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<i64>,
    quiet: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("tailr")
        .version("0.1.0")
        .author("me")
        .about("Rust tail")
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
                .takes_value(true)
                .conflicts_with("lines"),
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
        .arg(
            Arg::with_name("quiet")
                .help("don't print headers")
                .takes_value(false)
                .long("quiet")
                .short("q"),
        )
        .get_matches();

    let files = matches.values_of_lossy("file").unwrap();

    let bytes = match matches.value_of("bytes") {
        Some(b) => Some(parse_int(b).map_err(|e| format!("illegal byte count -- {}", e))?),
        None => None,
    };

    let lines = matches
        .value_of("lines")
        .expect("lines has a default value");
    let lines = parse_int(lines).map_err(|e| format!("illegal line count -- {}", e))?;

    Ok(Config {
        lines,
        bytes,
        files,
        quiet: matches.is_present("quiet"),
    })
}

fn parse_int<T: FromStr + num::Zero>(val: &str) -> MyResult<T> {
    match val.trim().parse::<T>() {
        Ok(n) if !n.is_zero() => Ok(n),
        _ => Err(From::from(val)),
    }
}
