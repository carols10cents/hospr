use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    println!("Hello, world!");
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Carol (Nichols || Goulding)")
        .about("Rust cat")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file")
                .multiple(true),
        )
        .arg(
            Arg::with_name("number")
                .help("number all output lines")
                .takes_value(false)
                .long("number")
                .short("n"),
        )
        .arg(
            Arg::with_name("number_noblank")
                .help("number nonempty output lines, overrides -n")
                .takes_value(false)
                .long("number-nonblank")
                .short("b"),
        )
        .get_matches();

    let files = matches.values_of_lossy("file").unwrap();
    let number_lines = matches.is_present("number");
    let number_nonblank_lines = matches.is_present("number_noblank");

    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}
