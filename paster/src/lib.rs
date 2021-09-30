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
                .min_values(1),
        )
        .arg(
            Arg::with_name("serial")
                .short("s")
                .long("serial")
                .takes_value(false)
                .help("Concatenate lines of each file serially"),
        )
        .arg(
            Arg::with_name("delimiter")
                .value_name("DELIMITER")
                .short("d")
                .long("delimiter")
                .help("Delimiter [default value: \t]"),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let delimiters: Vec<_> = matches
        .value_of_lossy("delimiter")
        .unwrap_or(String::from("\t").into())
        .chars()
        .map(|c| c.to_string())
        .collect();
    let serial = matches.is_present("serial");

    Ok(Config {
        files,
        delimiters,
        serial,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn parse_delimiters(given: &str) -> MyResult<Vec<String>> {
    let mut delimiters = vec![];

    let mut chars = given.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            let n = chars
                .next()
                .ok_or::<Box<dyn Error>>("Lone backslash".into())?;
            match n {
                't' => delimiters.push("\t".to_string()),
                'n' => delimiters.push("\n".to_string()),
                '0' => delimiters.push("".to_string()),
                '\\' => delimiters.push("\\".to_string()),
                _ => return Err(format!("Unknown escape \"{}{}\"", ch, n).into()),
            }
        } else {
            delimiters.push(ch.to_string());
        }
    }

    Ok(delimiters)
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
