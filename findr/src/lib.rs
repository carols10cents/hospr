use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use std::{error::Error, fs};
use walkdir::{DirEntry, WalkDir};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
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
            Arg::with_name("dirs")
                .value_name("DIR")
                .help("Search directory")
                .default_value(".")
                .min_values(1),
        )
        .arg(
            Arg::with_name("names")
                .value_name("NAME")
                .help("Name")
                .short("n")
                .long("name")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("types")
                .value_name("TYPE")
                .help("Entry type")
                .short("t")
                .long("type")
                .possible_values(&["f", "d", "l"])
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    let mut names = vec![];
    if let Some(vals) = matches.values_of_lossy("names") {
        for name in vals {
            match Regex::new(&name) {
                Ok(re) => names.push(re),
                _ => return Err(From::from(format!("Invalid --name \"{}\"", name))),
            }
        }
    }

    let entry_types = matches.values_of_lossy("types").map(|vals| {
        vals.iter()
            .filter_map(|val| match val.as_str() {
                "d" => Some(Dir),
                "f" => Some(File),
                "l" => Some(Link),
                _ => None,
            })
            .collect()
    });

    Ok(Config {
        dirs: matches.values_of_lossy("dirs").unwrap(),
        names: if names.is_empty() { None } else { Some(names) },
        entry_types,
    })
}

fn matches_type(config_entry_types: &Option<Vec<EntryType>>, entry: &DirEntry) -> bool {
    config_entry_types
        .as_ref()
        .map(|entry_types| {
            let ft = entry.file_type();
            (entry_types.contains(&Dir) && ft.is_dir())
                || (entry_types.contains(&File) && ft.is_file())
                || (entry_types.contains(&Link) && ft.is_symlink())
        })
        .unwrap_or(true)
}

fn matches_name(config_names: &Option<Vec<Regex>>, entry: &DirEntry) -> bool {
    config_names
        .as_ref()
        .map(|regexes| {
            let path = entry.file_name().to_str().unwrap(); // cheating
            regexes.iter().any(|regex| regex.is_match(&path))
        })
        .unwrap_or(true)
}

pub fn run(config: Config) -> MyResult<()> {
    for dirname in config.dirs {
        match fs::read_dir(&dirname) {
            Err(e) => eprintln!("{}: {}", dirname, e),
            _ => {
                for entry in WalkDir::new(dirname) {
                    let entry = entry?;
                    if let Some(types) = &config.entry_types {
                        if !types.iter().any(|type_| match type_ {
                            Link => entry.path_is_symlink(),
                            Dir => entry.file_type().is_dir(),
                            File => entry.file_type().is_file(),
                        }) {
                            continue;
                        }
                    }
                    println!("{}", entry.path().display());
                }
            }
        }
    }
    Ok(())
}
