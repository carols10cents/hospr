use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use std::{error::Error, str::FromStr};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl FromStr for EntryType {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "d" => Ok(Dir),
            "f" => Ok(File),
            "l" => Ok(Link),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    dirs: Vec<String>,
    names: Option<Vec<Regex>>,
    entry_types: Option<Vec<EntryType>>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust find")
        .arg(
            Arg::with_name("name")
                .value_name("NAME")
                .help("Name")
                .short("n")
                .long("name")
                .multiple(true),
        )
        .arg(
            Arg::with_name("type")
                .value_name("TYPE")
                .help("Entry type")
                .short("t")
                .long("type")
                .possible_values(&["f", "d", "l"])
                .multiple(true),
        )
        .arg(
            Arg::with_name("dir")
                .value_name("DIR")
                .help("Search directory")
                .multiple(true)
                .default_value("."),
        )
        .get_matches();

    let names = match matches.values_of_lossy("name") {
        Some(names) => {
            let mut regexes = vec![];
            for name in &names {
                regexes.push(Regex::new(name).map_err(|_| format!("Invalid --name \"{}\"", name))?);
            }
            Some(regexes)
        }
        None => None,
    };

    Ok(Config {
        dirs: matches.values_of_lossy("dir").unwrap(),
        names,
        entry_types: matches
            .values_of_lossy("type")
            .map(|types_list| types_list.iter().flat_map(|s| s.parse()).collect()),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
