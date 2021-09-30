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
        // What goes here?
        .get_matches();

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
