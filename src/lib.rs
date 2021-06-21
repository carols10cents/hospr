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
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match File::open(&filename) {
            Ok(file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let n = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..n]));
                } else {
                    let mut file = BufReader::new(file);
                    let mut line = String::new();
                    for line_num in 0.. {
                        if line_num == config.lines {
                            break;
                        }
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
            Err(err) => eprintln!("{}: {}", filename, err),
        };
    }
    Ok(())
}

fn parse_int(val: &str) -> MyResult<usize> {
    match val.trim().parse::<core::num::NonZeroUsize>() {
        Ok(n) => Ok(usize::from(n)),
        Err(_) => Err(From::from(val)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        // 3 is an OK integer
        let res2 = parse_int("3");
        assert!(res2.is_ok());
        assert_eq!(res2.unwrap(), 3);
        // Any string is an error
        let res3 = parse_int("foo");
        assert!(res3.is_err());
        if let Err(e) = res3 {
            assert_eq!(e.to_string(), "foo".to_string());
        }
        // A zero is an error
        let res4 = parse_int("0");
        assert!(res4.is_err());
        if let Err(e) = res4 {
            assert_eq!(e.to_string(), "0".to_string());
        }
    }
}
