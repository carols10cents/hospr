use clap::{App, Arg};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp::Ordering::*,
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"^([+-])?(\d+)$").unwrap();
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: i64,
    bytes: Option<i64>,
    quiet: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("tailr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust tail")
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .conflicts_with("lines")
                .help("Number of bytes"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Suppress headers"),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_num)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_num)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        lines: lines.unwrap(),
        bytes,
        files: matches.values_of_lossy("files").unwrap(),
        quiet: matches.is_present("quiet"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match File::open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                print_lines(BufReader::new(file), config.lines)?;
            }
        }
    }
    Ok(())
}

fn print_lines(mut file: impl BufRead, num_lines: i64) -> MyResult<()> {
    match num_lines.cmp(&0) {
        Greater => {
            let mut line = String::new();
            let mut line_num = 0;
            loop {
                let bytes = file.read_line(&mut line)?;
                if bytes == 0 {
                    break;
                }
                line_num += 1;
                if line_num >= num_lines {
                    print!("{}", line);
                }
                line.clear();
            }
        }
        _ => {}
    };
    Ok(())
}

fn last_lines(mut file: impl BufRead, num_lines: usize) -> MyResult<Vec<String>> {
    unimplemented!();
}

fn parse_num(val: &str) -> MyResult<i64> {
    let (sign, num) = match NUM_RE.captures(val) {
        Some(caps) => (
            caps.get(1).map_or("", |c| c.as_str()),
            caps.get(2).unwrap().as_str(),
        ),
        _ => return Err(From::from(val)),
    };

    match num.parse() {
        Ok(n) => Ok(if sign == "+" { n } else { -n }),
        _ => Err(From::from(val)),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_num;

    #[test]
    fn test_parse_num() {
        let res0 = parse_num("3");
        assert!(res0.is_ok());
        assert_eq!(res0.unwrap(), -3);

        let res = parse_num("+3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 3);

        let res = parse_num("-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), -3);

        let res = parse_num("3.14");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "3.14");

        let res = parse_num("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo");
    }
}
