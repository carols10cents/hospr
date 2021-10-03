use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    long: bool,
    show_hidden: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("lsr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust ls")
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .help("Files and/or directories")
                .default_value(".")
                .multiple(true),
        )
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("Show all files")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("long")
                .short("l")
                .long("long")
                .help("Long listing")
                .takes_value(false),
        )
        .get_matches();

    let paths = matches.values_of_lossy("paths").unwrap();
    let long = matches.is_present("long");
    let show_hidden = matches.is_present("all");

    Ok(Config {
        paths,
        long,
        show_hidden,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
