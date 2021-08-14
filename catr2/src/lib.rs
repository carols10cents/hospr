use std::error::Error;
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
    .version("1.0.0")
    .author("Carol (Nichols || Goulding)")
    .about("Rust cat")
    .arg(
        Arg::with_name("files")
            .value_name("FILES")
            .help("Input files")
            .required(true)
            .min_values(1),
    )
    .arg(
        Arg::with_name("number_lines")
            .help("Number all lines in the file")
            .takes_value(false)
            .short("n"),
    )
    .arg(
        Arg::with_name("number_nonblank_lines")
            .help("Number only non-blank lines in the file")
            .takes_value(false)
            .short("b"),
    )
    .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

pub fn run() -> MyResult<()> {
    println!("Hello, world!");
    Ok(())
}
