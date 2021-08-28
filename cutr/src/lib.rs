use crate::Extract::*;
use clap::{App, Arg};
use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

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
                .required(true)
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("delimiter")
                .value_name("DELIMITER")
                .help("Field delimiter")
                .short("d")
                .long("delim")
                .default_value("\t"),
        )
        .arg(
            Arg::with_name("fields")
                .value_name("FIELDS")
                .help("Selected fields")
                .short("f")
                .long("fields")
                .conflicts_with_all(&["chars", "bytes"]),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("Selected bytes")
                .short("b")
                .long("bytes")
                .conflicts_with_all(&["fields", "chars"]),
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .help("Selected characters")
                .short("c")
                .long("chars")
                .conflicts_with_all(&["fields", "bytes"]),
        )
        .get_matches();

    let delimiter = matches.value_of("delimiter").unwrap_or("\t");
    let delim_bytes = delimiter.as_bytes();
    if delim_bytes.len() > 1 {
        return Err(From::from(format!(
            "--delim \"{}\" must be a single byte",
            delimiter
        )));
    }

    let fields = matches.value_of("fields").map(parse_pos).transpose()?;
    let bytes = matches.value_of("bytes").map(parse_pos).transpose()?;
    let chars = matches.value_of("chars").map(parse_pos).transpose()?;

    let extract = if let Some(field_pos) = fields {
        Fields(field_pos)
    } else if let Some(byte_pos) = bytes {
        Bytes(byte_pos)
    } else if let Some(char_pos) = chars {
        Chars(char_pos)
    } else {
        return Err(From::from("Must have --fields, --bytes, or --chars"));
    };

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        delimiter: delim_bytes[0],
        extract,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_file) => println!("Opened {}", filename),
        }
    }
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

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn extract_chars(line: &str, char_pos: &[usize]) -> String {
    unimplemented!();
}

fn extract_bytes(line: &str, byte_pos: &[usize]) -> String {
    unimplemented!();
)

#[cfg(test)]
mod tests {
    use super::parse_pos;

    #[test]
    fn test_extract_chars() {
        assert_eq!(extract_chars("", &[0]), "".to_string());
        assert_eq!(extract_chars("ábc", &[0]), "á".to_string());
        assert_eq!(extract_chars("ábc", &[0, 2]), "ác".to_string());
        assert_eq!(extract_chars("ábc", &[0, 1, 2]), "ábc".to_string());
        assert_eq!(extract_chars("ábc", &[2, 1]), "cb".to_string());
        assert_eq!(extract_chars("ábc", &[0, 1, 4]), "áb".to_string());
    }

    fn test_extract_bytes() {
    assert_eq!(extract_bytes("ábc", &[0]), "".to_string());
    assert_eq!(extract_bytes("ábc", &[0, 1]), "á".to_string());
    assert_eq!(extract_bytes("ábc", &[0, 1, 2]), "áb".to_string());
    assert_eq!(extract_bytes("ábc", &[0, 1, 2, 3]), "ábc".to_string());
    assert_eq!(extract_bytes("ábc", &[3, 2]), "cb".to_string());
    assert_eq!(extract_bytes("ábc", &[0, 1, 5]), "á".to_string());
    }

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
