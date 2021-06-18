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
                .takes_value(true),
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

#[cfg(test)]
mod tests {
    use super::*;

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
