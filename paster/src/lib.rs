use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiters: Vec<String>,
    serial: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("paster")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust paste")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name("delimiters")
                .short("d")
                .long("delimiter")
                .value_name("DELIMITER")
                .help("Delimiter")
                .default_value("\\t")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("serial")
                .short("s")
                .long("serial")
                .help("Concatenate lines of each file serially")
                .takes_value(false),
        )
        .get_matches();

    let delimiters = matches
        .value_of("delimiters")
        .map(parse_delimiters)
        .transpose()?
        .unwrap();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        delimiters,
        serial: matches.is_present("serial"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn parse_delimiters(given: &str) -> MyResult<Vec<String>> {
    let mut chrs = given.chars();
    let mut results = vec![];
    while let Some(chr) = chrs.next() {
        if chr == '\\' {
            let esc = match chrs.next() {
                None => Err("Lone backslash".to_string()),
                Some('n') => Ok("\n".to_string()),
                Some('t') => Ok("\t".to_string()),
                Some('\\') => Ok("\\".to_string()),
                Some('0') => Ok("".to_string()),
                Some(c) => Err(format!("Unknown escape \"\\{}\"", c)),
            }?;
            results.push(esc);
        } else {
            results.push(chr.to_string());
        }
    }
    Ok(results)
}

#[cfg(test)]
mod test {
    use super::parse_delimiters;

    #[test]
    fn test_parse_delimiters() {
        // A single backslash is an error
        let res = parse_delimiters("\\");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Lone backslash");

        // Any backslash not followed by t, n, 0, or \\ is an error
        let res = parse_delimiters("\\x");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Unknown escape \"\\x\"");

        // A single character is OK
        let res = parse_delimiters(",");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), &[","]);

        // A tab character is OK
        let res = parse_delimiters("\\t");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), &["\t"]);

        // A newline character is OK
        let res = parse_delimiters("\\n");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), &["\n"]);

        // A literal backslash is OK
        let res = parse_delimiters("\\\\");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), &["\\"]);

        // The sequence \0 means the empty string
        let res = parse_delimiters("\\0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), &[""]);

        // Test all the things
        let res = parse_delimiters("\\t,\\n;\\\\\\0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), &["\t", ",", "\n", ";", "\\", ""]);
    }
}
