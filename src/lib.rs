use chrono::{DateTime, Local};
use clap::{App, Arg};
use std::error::Error;
use std::fs::{self, Metadata};
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use users::{get_group_by_gid, get_user_by_uid};

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
        match fs::metadata(&path) {
            Ok(metadata) => {
                if metadata.is_file() {
                    results.push(FileInfo {
                        path: path.into(),
                        metadata,
                    });
                } else {
                    for dir_entry in fs::read_dir(path)? {
                        let inner_path = dir_entry?.path().display().to_string();
                        match fs::metadata(&inner_path) {
                            Ok(metadata) => {
                                results.push(FileInfo {
                                    path: inner_path.into(),
                                    metadata,
                                });
                            }
                            Err(e) => errors.push(format!("{}: {}", inner_path, e)),
                        }
                    }
                }
            }
            Err(e) => errors.push(format!("{}: {}", path, e)),
        }
    }
    Ok((results, errors))
}

fn format_output(entry: &FileInfo, config: &Config) -> MyResult<String> {
    if config.long {
        let metadata = &entry.metadata;

        let d = if metadata.is_dir() { "d" } else { "-" };

        let mode = metadata.permissions().mode() as u16;
        let nlink = metadata.nlink();

        let user_id = metadata.uid();
        let username = get_user_by_uid(user_id).unwrap();
        let username = username.name().to_str().unwrap();
        let short_name = &username[..8];

        let group_id = metadata.gid();
        let group = get_group_by_gid(group_id).unwrap();
        let group = group.name().to_str().unwrap();

        let len = metadata.len();

        let modified: DateTime<Local> = metadata.modified()?.into();
        let modified = modified.format("%b %e %y %H:%M");

        Ok(format!(
            "{}{} {:>2} {:<8} {:<8} {:>8} {} {}",
            d,
            format_mode(mode),
            nlink,
            short_name,
            group,
            len,
            modified,
            entry.path
        ))
    } else {
        Ok(format!("{}", entry.path))
    }
}

#[derive(Debug)]
pub struct FileInfo {
    pub path: String,
    pub metadata: Metadata,
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

/// Given an octal number like 0o500 and three mask values,
/// return a string like "r-x"
pub fn mk_triple(mode: u16, read: u16, write: u16, execute: u16) -> String {
    format!(
        "{}{}{}",
        if mode & read == 0 { "-" } else { "r" },
        if mode & write == 0 { "-" } else { "w" },
        if mode & execute == 0 { "-" } else { "x" },
    )
}

fn format_mode(mode: u16) -> String {
    format!(
        "{}{}{}",
        mk_triple(mode, 0o400, 0o200, 0o100),
        mk_triple(mode, 0o040, 0o020, 0o010),
        mk_triple(mode, 0o004, 0o002, 0o001),
    )
}

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
