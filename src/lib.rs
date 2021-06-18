use clap::{App, Arg};
use std::{error::Error, fs::File};

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
    for file in files.iter().filter(|&name| name != "-") {
        File::open(file).map_err::<String, _>(|e| From::from(format!("{}: {}", file, e)))?;
    }

    let bytes = matches.value_of("bytes");
    let bytes = match bytes {
        Some(b) => Some(reject_unparsable_or_zero("byte", b)?),
        None => None,
    };

    let lines = matches.value_of("lines").unwrap();
    let lines = reject_unparsable_or_zero("line", lines)?;

    Ok(Config {
        lines,
        bytes,
        files,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn reject_unparsable_or_zero(arg_name: &str, val: &str) -> MyResult<usize> {
    let parse_result = val.parse();
    match parse_result {
        Ok(0) | Err(_) => Err(From::from(format!("illegal {} count -- {}", arg_name, val))),
        Ok(num) => Ok(num),
    }
}

fn parse_int(val: Option<&str>) -> MyResult<Option<usize>> {
    match val {
        Some(v) => {
            let i = v.parse().map_err::<String, _>(|_| From::from(v))?;
            if i == 0 {
                Err(From::from(v))
            } else {
                Ok(Some(i))
            }
        }
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

    #[test]
    fn zero_is_invalid() {
        let arg_name = "foo";
        let val = "0";
        let result = reject_unparsable_or_zero(arg_name, val);
        assert_eq!(result.unwrap_err().to_string(), "illegal foo count -- 0");
    }

    #[test]
    fn nonnumber_is_invalid() {
        let arg_name = "foo";
        let val = "notanumber";
        let result = reject_unparsable_or_zero(arg_name, val);
        assert_eq!(
            result.unwrap_err().to_string(),
            "illegal foo count -- notanumber"
        );
    }

    #[test]
    fn number_is_valid() {
        let arg_name = "foo";
        let val = "1000";
        let result = reject_unparsable_or_zero(arg_name, val);
        assert_eq!(result.unwrap(), 1000);
    }
}
