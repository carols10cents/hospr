use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("1.0")
        .author("Carol (Nichols || Goulding)")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("LINES")
                .long("lines")
                .help("Number of lines")
                .takes_value(true)
                .short("n")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("BYTES")
                .long("bytes")
                .help("Number of bytes")
                .takes_value(true)
                .short("c")
                .conflicts_with("LINES"),
        )
        .get_matches();

    let bytes = match matches.value_of("bytes") {
        Some(s) => Some(parse_positive_int(s).map_err(|e| format!("illegal byte count -- {}", e))?),
        None => None,
    };

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: parse_positive_int(matches.value_of("lines").unwrap())
            .map_err(|e| format!("illegal line count -- {}", e))?,
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
