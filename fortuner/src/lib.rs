use clap::{App, Arg};
use regex::{Regex, RegexBuilder};
use std::{
    collections::BTreeSet,
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
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
    let fortunes = read_fortunes(&files, &config.pattern)?;
    match config.pattern.is_some() {
        true => {
            for fortune in fortunes {
                // Print output
            }
        }
        _ => {
            if let Some(fortune) = pick_fortune(&fortunes, &config.seed) {
                println!("{}", fortune);
            }
        }
    };
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
        let metadata = fs::metadata(source).map_err(|e| format!("{}: {}", source, e))?;

        if metadata.is_dir() {
            for s in fs::read_dir(source)? {
                let s = s?;
                s.metadata().map_err(|e| format!("{}: {}", source, e))?;
                answer.insert(s.path());
            }
        } else {
            answer.insert(PathBuf::from(source));
        }
    }

    Ok(answer.into_iter().collect())
}

fn read_fortunes(paths: &[PathBuf], pattern: &Option<Regex>) -> MyResult<Vec<Fortune>> {
    let mut fortunes = vec![];

    for path in paths {
        let source = path.file_name().unwrap().to_string_lossy().to_string();

        let mut file = BufReader::new(File::open(path)?);
        let mut line = String::new();
        let mut texts = vec![];

        loop {
            let bytes = file.read_line(&mut line)?;
            if bytes == 0 {
                break;
            }

            let trim = line.trim();

            if trim == "%" {
                let text = texts.join("\n");
                if let Some(pat) = pattern {
                    if pat.is_match(&text) {
                        fortunes.push(Fortune {
                            source: source.clone(),
                            text,
                        });
                    }
                } else {
                    fortunes.push(Fortune {
                        source: source.clone(),
                        text,
                    });
                }

                texts.clear();
            } else {
                texts.push(trim.to_owned());
            }
            line.clear();
        }
    }

    Ok(fortunes)
}

#[derive(Debug, Clone)]
struct Fortune {
    source: String,
    text: String,
}

#[cfg(test)]
mod tests {
    use super::{find_files, parse_u64, read_fortunes};
    use regex::Regex;
    use std::path::PathBuf;

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

    #[test]
    fn test_read_fortunes() {
        // Parses all the fortunes without a filter
        let res = read_fortunes(&[PathBuf::from("./tests/inputs/fortunes")], &None);
        assert!(res.is_ok());

        if let Ok(fortunes) = res {
            // Correct number and sorting
            assert_eq!(fortunes.len(), 5433);
            assert_eq!(
                fortunes.iter().nth(0).unwrap().text,
                "You cannot achieve the impossible without \
    attempting the absurd."
            );
            assert_eq!(
                fortunes.last().unwrap().text,
                "There is no material safety data sheet for \
    astatine. If there were, it would just be the word \
    \"NO\" scrawled over and over in charred blood.\n\
    -- Randall Munroe, \"What If?\""
            );
        }

        // Filters for matching text
        let res = read_fortunes(
            &[PathBuf::from("./tests/inputs/fortunes")],
            &Some(Regex::new("Yogi Berra").unwrap()),
        );
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 2);
    }
}
