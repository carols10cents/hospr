use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .help("Show line count")
                .takes_value(false)
                .short("l")
                .long("lines"),
        )
        .arg(
            Arg::with_name("words")
                .value_name("WORDS")
                .help("Show word count")
                .takes_value(false)
                .short("w")
                .long("words"),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("Show byte count")
                .takes_value(false)
                .short("c")
                .long("bytes"),
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .help("Show character count")
                .takes_value(false)
                .short("m")
                .long("chars")
                .conflicts_with("bytes"),
        )
        .get_matches();
    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let mut chars = matches.is_present("chars");
    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
        chars = false;
    }
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{:>8}{:>8}{:>8} {}",
                        info.num_lines, info.num_words, info.num_bytes, filename
                    );
                }
            }
        }
    }
    Ok(())
}

// pub fn run(config: Config) -> MyResult<()> {
//     let (mut total_lines, mut total_words, mut total_bytes, mut total_chars) = (0, 0, 0, 0);
//
//     for filename in &config.files {
//         match open(filename) {
//             Err(err) => eprintln!("{}: {}", filename, err),
//             Ok(mut file) => {
//                 let mut lines = 0;
//                 let mut words = 0;
//                 let mut bytes = 0;
//                 let mut chars = 0;
//
//                 let mut line = String::new();
//                 loop {
//                     let num_bytes_read = file.read_line(&mut line)?;
//                     if num_bytes_read == 0 {
//                         break;
//                     }
//                     lines += 1;
//
//                     words += line.split_whitespace().count();
//                     bytes += line.as_bytes().len();
//                     chars += line.chars().count();
//
//                     line.clear();
//                 }
//
//                 if config.lines {
//                     print!("{:>8}", lines);
//                 }
//                 if config.words {
//                     print!("{:>8}", words);
//                 }
//                 if config.bytes {
//                     print!("{:>8}", bytes);
//                 }
//                 if config.chars {
//                     print!("{:>8}", chars);
//                 }
//                 if filename == "-" {
//                     println!();
//                 } else {
//                     println!(" {}", filename);
//                 }
//                 total_lines += lines;
//                 total_words += words;
//                 total_bytes += bytes;
//                 total_chars += chars;
//             }
//         }
//     }
//
//     if config.files.len() > 1 {
//         if config.lines {
//             print!("{:>8}", total_lines);
//         }
//         if config.words {
//             print!("{:>8}", total_words);
//         }
//         if config.bytes {
//             print!("{:>8}", total_bytes);
//         }
//         if config.chars {
//             print!("{:>8}", total_chars);
//         }
//         println!(" total");
//     }
//
//     Ok(())
// }

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;
    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
