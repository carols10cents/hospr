use clap::{App, Arg};
use std::{
    cmp::Ordering,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file1: String,
    file2: String,
    suppress_col1: bool,
    suppress_col2: bool,
    suppress_col3: bool,
    insensitive: bool,
    delimiter: String,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("commr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust comm")
        .arg(
            Arg::with_name("file1")
                .value_name("FILE1")
                .help("Input file 1")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("file2")
                .value_name("FILE2")
                .help("Input file 2")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("suppress_col1")
                .short("1")
                .value_name("COL1")
                .takes_value(false)
                .help("Suppress printing of column 1"),
        )
        .arg(
            Arg::with_name("suppress_col2")
                .short("2")
                .value_name("COL2")
                .takes_value(false)
                .help("Suppress printing of column 2"),
        )
        .arg(
            Arg::with_name("suppress_col3")
                .short("3")
                .value_name("COL3")
                .takes_value(false)
                .help("Suppress printing of column 3"),
        )
        .arg(
            Arg::with_name("insensitive")
                .short("i")
                .value_name("INSENSITIVE")
                .takes_value(false)
                .help("Case insensitive comparison of lines"),
        )
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("output-delimiter")
                .value_name("DELIM")
                .help("Output delimiter")
                .takes_value(true),
        )
        .get_matches();

    Ok(Config {
        file1: matches.value_of("file1").unwrap().to_string(),
        file2: matches.value_of("file2").unwrap().to_string(),
        suppress_col1: matches.is_present("suppress_col1"),
        suppress_col2: matches.is_present("suppress_col2"),
        suppress_col3: matches.is_present("suppress_col3"),
        insensitive: matches.is_present("insensitive"),
        delimiter: matches.value_of("delimiter").unwrap_or("\t").to_string(),
    })
}

enum Cols {
    Col1(String),
    Col2(String),
    Col3(String),
}

use Cols::*;

pub fn run(config: Config) -> MyResult<()> {
    let filename1 = &config.file1;
    let filename2 = &config.file2;
    if filename1 == "-" && filename2 == "-" {
        return Err(From::from("Both input files cannot be STDIN (\"-\")"));
    }
    let file1 = open(filename1)?;
    let file2 = open(filename2)?;

    let mut file1_lines = file1.lines().filter_map(|i| i.ok()).map(|i| {
        if config.insensitive {
            i.to_lowercase()
        } else {
            i
        }
    });
    let mut file2_lines = file2.lines().filter_map(|i| i.ok()).map(|i| {
        if config.insensitive {
            i.to_lowercase()
        } else {
            i
        }
    });

    let mut f1_next = file1_lines.next();
    let mut f2_next = file2_lines.next();

    let print = |value| match value {
        Col1(s) => {
            if !config.suppress_col1 {
                println!("{}", s);
            }
        }
        Col2(s) => {
            if !config.suppress_col2 {
                let pre = if config.suppress_col1 { "" } else { "\t" };
                println!("{}{}", pre, s);
            }
        }
        Col3(s) => {
            if !config.suppress_col3 {
                let pre = if config.suppress_col1 && config.suppress_col2 {
                    ""
                } else if config.suppress_col1 || config.suppress_col2 {
                    "\t"
                } else {
                    "\t\t"
                };
                println!("{}{}", pre, s);
            }
        }
    };

    loop {
        match (&f1_next, &f2_next) {
            (Some(f1), Some(f2)) => match f1.cmp(&f2) {
                Ordering::Greater => {
                    print(Col2(f2.to_string()));
                    f2_next = file2_lines.next();
                }
                Ordering::Less => {
                    print(Col1(f1.to_string()));
                    f1_next = file1_lines.next();
                }
                Ordering::Equal => {
                    print(Col3(f1.to_string()));
                    f1_next = file1_lines.next();
                    f2_next = file2_lines.next();
                }
            },
            (Some(f1), None) => {
                print(Col1(f1.to_string()));
                f1_next = file1_lines.next();
            }
            (None, Some(f2)) => {
                print(Col2(f2.to_string()));
                f2_next = file2_lines.next();
            }
            (None, None) => break,
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| format!("{}: {}", filename, e))?,
        ))),
    }
}
