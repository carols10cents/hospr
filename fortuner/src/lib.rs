use clap::{App, Arg};
use regex::{Regex, RegexBuilder};
use std::{
    collections::BTreeSet,
    error::Error,
    path::{Path, PathBuf},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    sources: Vec<String>,
    pattern: Option<Regex>,
    seed: Option<u64>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("fortuner")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust fortune")
        .arg(
            Arg::with_name("sources")
                .value_name("FILE")
                .multiple(true)
                .required(true)
                .help("Input files or directories"),
        )
        .arg(
            Arg::with_name("pattern")
                .value_name("PATTERN")
                .short("m")
                .long("pattern")
                .help("Pattern"),
        )
        .arg(
            Arg::with_name("insensitive")
                .short("i")
                .long("insensitive")
                .help("Case-insensitive pattern matching")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("seed")
                .value_name("SEED")
                .short("s")
                .long("seed")
                .help("Random seed"),
        )
        .get_matches();

    let pattern = matches
        .value_of("pattern")
        .map(|val| {
            RegexBuilder::new(val)
                .case_insensitive(matches.is_present("insensitive"))
                .build()
                .map_err(|_| format!("Invalid --pattern \"{}\"", val))
        })
        .transpose()?;

    Ok(Config {
        sources: matches.values_of_lossy("sources").unwrap(),
        seed: matches.value_of("seed").map(parse_u64).transpose()?,
        pattern,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let files = find_files(&config.sources)?;
    println!("{:#?}", files);
    Ok(())
}

fn parse_u64(val: &str) -> MyResult<u64> {
    Ok(val
        .parse()
        .map_err(|_| format!("\"{}\" not a valid integer", val))?)
}

fn find_files(sources: &[String]) -> MyResult<Vec<PathBuf>> {
    let mut answer = BTreeSet::new();

    for source in sources {
        let path = Path::new(source);

        if !path.exists() {
            return Err("nope".into());
        }

        if path.is_dir() {
            for s in path.read_dir()? {
                let p = s?.path();
                if !p.exists() {
                    return Err("nope".into());
                }
                answer.insert(p);
            }
        } else {
            answer.insert(path.to_path_buf());
        }
    }

    Ok(answer.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::{find_files, parse_u64};

    #[test]
    fn test_parse_u64() {
        let res = parse_u64("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "\"a\" not a valid integer");
        let res = parse_u64("0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0);
        let res = parse_u64("4");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 4);
    }

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let res = find_files(&["./tests/inputs/fortunes".to_string()]);
        assert!(res.is_ok());

        let files = res.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(
            files.get(0).unwrap().to_string_lossy(),
            "./tests/inputs/fortunes"
        );

        // Fails to find a bad file
        let res = find_files(&["/path/does/not/exist".to_string()]);
        assert!(res.is_err());

        // Finds all the input files, excludes ".dat"
        let res = find_files(&["./tests/inputs".to_string()]);
        assert!(res.is_ok());

        // Check number and order of files
        let files = res.unwrap();
        assert_eq!(files.len(), 5);
        let first = files.get(0).unwrap().display().to_string();
        assert!(first.contains("ascii-art"));
        let last = files.last().unwrap().display().to_string();
        assert!(last.contains("startrek"));
    }
}
