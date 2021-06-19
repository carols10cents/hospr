use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("me")
        .about("Rust head")
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
        .get_matches();

    let files = matches.values_of_lossy("file").unwrap();

    let bytes = parse_int(matches.value_of("bytes"));
    if let Err(bad_bytes) = bytes {
        return Err(From::from(format!("illegal byte count -- {}", bad_bytes)));
    }

    let lines =
        parse_int(matches.value_of("lines")).map_err(|e| format!("illegal line count -- {}", e))?;

    Ok(Config {
        lines: lines.unwrap(),
        bytes: bytes?,
        files,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match File::open(&filename) {
            Ok(file) => {
                let mut file = BufReader::new(file);
                let mut line = String::new();
                let mut line_num = 0;
                loop {
                    if line_num == config.lines {
                        break;
                    }
                    let bytes = file.read_line(&mut line)?;
                    if bytes == 0 {
                        break;
                    }
                    print!("{}", line);
                    line_num += 1;
                    line.clear();
                }
            }
            Err(err) => eprintln!("{}: {}", filename, err),
        };
    }
    Ok(())
}

fn parse_int(val: Option<&str>) -> MyResult<Option<usize>> {
    match val {
        Some(v) => match v.trim().parse::<core::num::NonZeroUsize>() {
            Ok(n) => Ok(Some(usize::from(n))),
            Err(_) => Err(From::from(v)),
        },
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int_none_is_fine() {
        let result = parse_int(None);
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_parse_int() {
        // No value is OK
        let res1 = parse_int(None);
        assert!(res1.is_ok());
        assert!(res1.unwrap().is_none());
        // 3 is an OK integer
        let res2 = parse_int(Some("3"));
        assert!(res2.is_ok());
        assert_eq!(res2.unwrap(), Some(3));
        // Any string is an error
        let res3 = parse_int(Some("foo"));
        assert!(res3.is_err());
        if let Err(e) = res3 {
            assert_eq!(e.to_string(), "foo".to_string());
        }
        // A zero is an error
        let res4 = parse_int(Some("0"));
        assert!(res4.is_err());
        if let Err(e) = res4 {
            assert_eq!(e.to_string(), "0".to_string());
        }
    }
}
