use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
    str::FromStr,
};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<i64>,
    quiet: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("tailr")
        .version("0.1.0")
        .author("me")
        .about("Rust tail")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("Number of bytes")
                .long("bytes")
                .short("c")
                .takes_value(true)
                .conflicts_with("lines"),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .help("Number of lines")
                .long("lines")
                .short("n")
                .takes_value(true)
                .default_value("10"),
        )
        .arg(
            Arg::with_name("quiet")
                .help("don't print headers")
                .takes_value(false)
                .long("quiet")
                .short("q"),
        )
        .get_matches();

    let files = matches.values_of_lossy("file").unwrap();

    let bytes = match matches.value_of("bytes") {
        Some(b) => Some(parse_int(b).map_err(|e| format!("illegal byte count -- {}", e))?),
        None => None,
    };

    let lines = matches
        .value_of("lines")
        .expect("lines has a default value");
    let lines = parse_int(lines).map_err(|e| format!("illegal line count -- {}", e))?;

    Ok(Config {
        lines,
        bytes,
        files,
        quiet: matches.is_present("quiet"),
    })
}

fn parse_int<T>(val: &str) -> MyResult<T>
where
    T: FromStr + num::Zero,
{
    match val.trim().parse::<T>() {
        Ok(n) if !n.is_zero() => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[cfg(test)]
mod test {
    use super::{parse_int, MyResult};
    #[test]
    fn test_parse_int() {
        // 3 is an OK integer
        let res2: MyResult<u32> = parse_int("3");
        assert!(res2.is_ok());
        assert_eq!(res2.unwrap(), 3u32);
        // 4 is an OK integer
        let res3 = parse_int::<i64>("4");
        assert!(res3.is_ok());
        assert_eq!(res3.unwrap(), 4i64);
        // Any string is an error
        let res4 = parse_int::<u32>("foo");
        assert!(res4.is_err());
        if let Err(e) = res4 {
            assert_eq!(e.to_string(), "foo".to_string());
        }
        // A zero is an error
        let res5 = parse_int::<u32>("0");
        assert!(res5.is_err());
        if let Err(e) = res5 {
            assert_eq!(e.to_string(), "0".to_string());
        }
    }
}
