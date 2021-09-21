use clap::{App, Arg};
use regex::{Regex, RegexBuilder};
use std::error::Error;

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
            Arg::with_name("pattern")
                .short("m")
                .long("pattern")
                .takes_value(true)
                .value_name("PATTERN")
                .help("Search pattern"),
        )
        .arg(
            Arg::with_name("seed")
                .short("s")
                .long("seed")
                .takes_value(true)
                .value_name("SEED")
                .help("PRNG seed"),
        )
        .arg(
            Arg::with_name("sources")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    let pattern = match matches.value_of("pattern") {
        Some(pat) => Some(
            RegexBuilder::new(pat)
                .build()
                .map_err(|_| format!("Invalid --pattern \"{}\"", pat))?,
        ),
        None => None,
    };

    let seed = match matches.value_of("seed") {
        Some(s) => Some(parse_u64(s)?),
        None => None,
    };

    Ok(Config {
        sources: matches.values_of_lossy("sources").unwrap(),
        seed,
        pattern,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn parse_u64(val: &str) -> MyResult<u64> {
    Ok(val
        .parse()
        .map_err(|_| format!("\"{}\" not a valid integer", val))?)
}

#[cfg(test)]
mod tests {
    use super::parse_u64;

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
}
