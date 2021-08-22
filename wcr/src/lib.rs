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
                let mut lines = 0;
                let mut words = 0;
                let mut bytes = 0;
                let mut chars = 0;

                for line in file.lines() {
                    let line = line?;
                    lines += 1;
                    bytes += 1;
                    chars += 1;

                    words += line.split_whitespace().count();
                    bytes += line.as_bytes().len();
                    chars += line.chars().count();
                }

                if config.lines {
                    print!("{:>8}", lines);
                }
                if config.words {
                    print!("{:>8}", words);
                }
                if config.bytes {
                    print!("{:>8}", bytes);
                }
                if config.chars {
                    print!("{:>8}", chars);
                }
                if filename == "-" {
                    println!();
                } else {
                    println!(" {}", filename);
                }
            },
        }
    }
    Ok(())
}
