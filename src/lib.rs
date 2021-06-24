use clap::{App, Arg};
use std::error::Error;
use std::fs::Metadata;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    entries: Vec<String>,
    long: bool,
    all: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    let (entries, errors) = find_files(&config)?;
    for error in errors {
        eprintln!("{}", error);
    }
    for entry in entries {
        println!("{}", format_output(&entry, &config)?);
    }
    Ok(())
}

fn find_files(config: &Config) -> MyResult<(Vec<FileInfo>, Vec<String>)> {
    let mut results = vec![];
    let mut errors = vec![];
    for path in &config.entries {
        // ... Check each entry
    }
    Ok((results, errors))
}

fn format_output(entry: &FileInfo, config: &Config) -> MyResult<String> {
    unimplemented!();
}

#[derive(Debug)]
pub struct FileInfo {
    path: String,
    metadata: Metadata,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("lsr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust ls")
        .arg(
            Arg::with_name("entries")
                .value_name("ENTRY")
                .help("Files and/or directories")
                .required(true)
                .default_value(".")
                .min_values(1),
        )
        .arg(
            Arg::with_name("long")
                .value_name("LONG")
                .takes_value(false)
                .help("Long listing")
                .short("-l")
                .long("--long"),
        )
        .arg(
            Arg::with_name("all")
                .value_name("ALL")
                .takes_value(false)
                .help("Show all files")
                .short("-a")
                .long("--all"),
        )
        .get_matches();
    Ok(Config {
        entries: matches.values_of_lossy("entries").unwrap(),
        long: matches.is_present("long"),
        all: matches.is_present("all"),
    })
}

fn mk_triple(mode: u16, read: u16, write: u16, execute: u16) -> String {}
fn format_mode(mode: u16) -> String {}

#[cfg(test)]
mod test {
    use super::{format_mode, mk_triple};
    #[test]
    fn test_mk_triple() {
        assert_eq!(mk_triple(0o751, 0o400, 0o200, 0o100), "rwx");
        assert_eq!(mk_triple(0o751, 0o040, 0o020, 0o010), "r-x");
        assert_eq!(mk_triple(0o751, 0o004, 0o002, 0o001), "--x");
        assert_eq!(mk_triple(0o600, 0o004, 0o002, 0o001), "---");
    }

    #[test]
    fn test_format_mode() {
        assert_eq!(format_mode(0o755), "rwxr-xr-x");
        assert_eq!(format_mode(0o421), "r---w---x");
    }
}
