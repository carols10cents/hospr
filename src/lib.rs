use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{Read, Seek, SeekFrom},
    str::FromStr,
};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    let multiple_files = config.files.len() > 1;
    for (file_num, filename) in config.files.iter().enumerate() {
        match File::open(&filename) {
            Ok(mut file) => {
                if multiple_files {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }

                let len = file.metadata()?.len() as i64;
                if let Some(requested_bytes) = config.bytes {
                    let num_bytes_to_print = std::cmp::min(len, requested_bytes) * -1;
                    file.seek(SeekFrom::End(num_bytes_to_print))?;
                    let bytes = file.bytes().collect::<Result<Vec<_>, _>>()?;
                    print!("{}", String::from_utf8_lossy(&bytes));
                } else {
                    unimplemented!();
                }
            }
            Err(err) => eprintln!("{}: {}", filename, err),
        };
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<i64>,
    quiet: bool,
}
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("tail")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust tail")
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .conflicts_with("lines")
                .help("Number of bytes"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Suppress headers"),
        )
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .min_values(1),
        )
        .get_matches();
    let lines = parse_int::<usize>(matches.value_of("lines"));
    if let Err(bad_lines) = lines {
        return Err(From::from(format!("illegal line count -- {}", bad_lines)));
    }
    let bytes = parse_int::<i64>(matches.value_of("bytes"));
    if let Err(bad_bytes) = bytes {
        return Err(From::from(format!("illegal byte count -- {}", bad_bytes)));
    }
    Ok(Config {
        lines: lines?.unwrap(),
        bytes: bytes?,
        files: matches.values_of_lossy("file").unwrap(),
        quiet: matches.is_present("quiet"),
    })
}

fn parse_int<T>(val: Option<&str>) -> MyResult<Option<T>>
where
    T: FromStr + num::Zero,
{
    match val {
        Some(v) => match v.trim().parse::<T>() {
            Ok(n) if !n.is_zero() => Ok(Some(n)),
            _ => Err(From::from(v)),
        },
        None => Ok(None),
    }
}

#[cfg(test)]
mod test {
    use super::{parse_int, MyResult};
    #[test]
    fn test_parse_int() {
        // No value is OK
        let res1 = parse_int::<u32>(None);
        assert!(res1.is_ok());
        assert!(res1.unwrap().is_none());
        // 3 is an OK integer
        let res2: MyResult<Option<u32>> = parse_int(Some("3"));
        assert!(res2.is_ok());
        assert_eq!(res2.unwrap(), Some(3u32));
        // 4 is an OK integer
        let res3 = parse_int::<i64>(Some("4"));
        assert!(res3.is_ok());
        assert_eq!(res3.unwrap(), Some(4i64));
        // Any string is an error
        let res4 = parse_int::<u32>(Some("foo"));
        assert!(res4.is_err());
        if let Err(e) = res4 {
            assert_eq!(e.to_string(), "foo".to_string());
        }
        // A zero is an error
        let res5 = parse_int::<u32>(Some("0"));
        assert!(res5.is_err());
        if let Err(e) = res5 {
            assert_eq!(e.to_string(), "0".to_string());
        }
    }
}
