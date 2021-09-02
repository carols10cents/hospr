use clap::{App, Arg};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"^([+-])?(\d+)$").unwrap();
}

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
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_num)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_num)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        lines: lines.unwrap(),
        bytes,
        files: matches.values_of_lossy("files").unwrap(),
        quiet: matches.is_present("quiet"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let multiple_files = config.files.len() > 1;

    for filename in config.files {
        match File::open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let mut file = BufReader::new(file);
                if multiple_files {
                    println!("==> {} <==", filename);
                }
                if let Some(num_bytes) = config.bytes {
                    // Get the file size so we know if we've gone past the end
                    let total_len = file.seek(SeekFrom::End(0)).unwrap();

                    if num_bytes > 0 {
                        // First seek past the number of bytes to skip
                        let new_pos = file.seek(SeekFrom::Start(num_bytes as u64 - 1)).unwrap();

                        // As long as we aren't past the end
                        if new_pos < total_len {
                            // Print the rest
                            let mut line = vec![];
                            loop {
                                let bytes = file.read_until('\n' as u8, &mut line)?;
                                if bytes == 0 {
                                    break;
                                }
                                print!("{}", String::from_utf8_lossy(&line));
                                line.clear();
                            }
                        }
                    } else {
                        // if seeking this many bytes would take us past the beginning, start at
                        // the beginning instead.
                        if total_len as i64 + num_bytes < 0 {
                            file.seek(SeekFrom::Start(0)).unwrap();
                        } else {
                            file.seek(SeekFrom::End(num_bytes)).unwrap();
                        }
                        // Print the rest
                        let mut line = vec![];
                        loop {
                            let bytes = file.read_until('\n' as u8, &mut line)?;
                            if bytes == 0 {
                                break;
                            }
                            print!("{}", String::from_utf8_lossy(&line));
                            line.clear();
                        }
                    }
                } else {
                    if config.lines > 0 {
                        let mut line = String::new();

                        // First skip config.lines
                        for _ in 0..config.lines - 1 {
                            let bytes = file.read_line(&mut line)?;
                            if bytes == 0 {
                                break;
                            }
                            line.clear();
                        }

                        // Then print the rest
                        loop {
                            let bytes = file.read_line(&mut line)?;
                            if bytes == 0 {
                                break;
                            }
                            print!("{}", line);
                            line.clear();
                        }
                    } else {
                        if let Err(_) = file.seek(SeekFrom::End(-1)) {
                            // if the file is empty, go to the next file
                            continue;
                        }

                        // First seek back abs(config.lines) number of lines
                        let mut lines_found = 1; // lol magic number

                        loop {
                            // read one byte
                            let mut byte_buf: Vec<u8> = vec![0];
                            file.read_exact(&mut byte_buf).unwrap();
                            dbg!(byte_buf[0] as char);

                            // if we found a newline, count it
                            if byte_buf[0] == '\n' as u8 {
                                lines_found -= 1;
                                dbg!(lines_found, config.lines);
                                if lines_found == config.lines {
                                    break;
                                }
                            }

                            // go back the byte we just read and the byte before that
                            if let Err(_) = file.seek(SeekFrom::Current(-2)) {
                                // if we're at the beginning of the file, stop
                                file.seek(SeekFrom::Current(-1)).unwrap();
                                break;
                            }
                        }

                        // Then print
                        let mut line = String::new();
                        loop {
                            let bytes = file.read_line(&mut line)?;
                            if bytes == 0 {
                                break;
                            }
                            print!("{}", line);
                            line.clear();
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn parse_num(val: &str) -> MyResult<i64> {
    let (sign, num) = match NUM_RE.captures(val) {
        Some(caps) => (
            caps.get(1).map_or("", |c| c.as_str()),
            caps.get(2).unwrap().as_str(),
        ),
        _ => return Err(From::from(val)),
    };

    match num.parse() {
        Ok(n) => Ok(if sign == "+" { n } else { -n }),
        _ => Err(From::from(val)),
    }
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
