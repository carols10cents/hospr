use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

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
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .takes_value(true)
                .min_values(1)
                .required(true),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .help("Number of bytes")
                .conflicts_with("lines"),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .takes_value(false)
                .help("Suppress headers"),
        )
        .get_matches();

    let bytes = match matches.value_of("bytes") {
        None => None,
        Some(b) => Some(parse_num(b).map_err(|e| format!("illegal byte count -- {}", e))?),
    };

    let lines = matches.value_of("lines").unwrap();
    let lines = parse_num(lines).map_err(|e| format!("illegal line count -- {}", e))?;

    Ok(Config {
        lines,
        bytes,
        files: matches.values_of_lossy("file").unwrap(),
        quiet: matches.is_present("quiet"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn parse_num(val: &str) -> MyResult<i64> {
    let num = val.parse().map_err(|_| val)?;
    Ok(if !val.starts_with("+") && !val.starts_with("-") {
        num * -1
    } else {
        num
    })
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
