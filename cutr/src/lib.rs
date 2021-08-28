use crate::Extract::*;
use clap::{App, Arg};
use regex::Regex;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<usize>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("cutr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cut")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("Selected bytes")
                .short("b")
                .long("bytes")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .help("Selected characters")
                .short("c")
                .long("chars")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("delimiter")
                .value_name("DELIMITER")
                .help("Field delimiter")
                .short("d")
                .long("delim")
                .default_value("\t")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("fields")
                .value_name("FIELDS")
                .help("Selected fields")
                .short("f")
                .long("fields")
                .conflicts_with("bytes")
                .conflicts_with("chars")
                .takes_value(true),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();

    let delimiter_orig = matches.value_of("delimiter").unwrap(); // safe because of the default
    let mut delimiter_iter = delimiter_orig.bytes();

    let delimiter = match delimiter_iter.next() {
        Some(d) => d,
        None => return Err("--delim must be at least one byte".into()),
    };

    if delimiter_iter.next().is_some() {
        return Err(format!("--delim \"{}\" must be a single byte", delimiter_orig).into());
    }

    let extract = match (
        matches.value_of("bytes"),
        matches.value_of("chars"),
        matches.value_of("fields"),
    ) {
        (Some(bytes), None, None) => Bytes(parse_pos(bytes)?),
        (None, Some(chars), None) => Chars(parse_pos(chars)?),
        (None, None, Some(fields)) => Fields(parse_pos(fields)?),
        (None, None, None) => return Err("Must have --fields, --bytes, or --chars".into()),
        _ => unreachable!(),
    };

    Ok(Config {
        files,
        delimiter,
        extract,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", &config);
    Ok(())
}

fn parse_pos(range: &str) -> MyResult<PositionList> {
    let mut fields = vec![];
    let range_re = Regex::new(r"(\d+)-(\d+)").unwrap();
    for val in range.split(',') {
        if let Some(cap) = range_re.captures(val) {
            let n1: usize = cap[1].parse()?;
            let n2: usize = cap[2].parse()?;
            if n1 < n2 {
                for n in n1..=n2 {
                    fields.push(n);
                }
            } else {
                return Err(From::from(format!(
                    "First number in range ({}) \
must be lower than second number ({})",
                    n1, n2
                )));
            }
        } else {
            match val.parse() {
                Ok(n) if n > 0 => fields.push(n),
                _ => return Err(From::from(format!("illegal list value: \"{}\"", val))),
            }
        }
    }
    // Subtract one for field indexes
    Ok(fields.into_iter().map(|i| i - 1).collect())
}

#[cfg(test)]
mod tests {
    use super::parse_pos;

    #[test]
    fn test_parse_pos() {
        assert!(parse_pos("").is_err());

        let res = parse_pos("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

        let res = parse_pos("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        let res = parse_pos("1,a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        let res = parse_pos("2-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );

        let res = parse_pos("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0]);

        let res = parse_pos("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0, 2]);

        let res = parse_pos("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0, 1, 2]);

        let res = parse_pos("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0, 6, 2, 3, 4]);
    }
}
