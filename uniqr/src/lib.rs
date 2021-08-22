use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniq")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust uniq")
        .arg(
            Arg::with_name("in_file")
                .value_name("INPUT")
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("OUTPUT")
                .help("Output file"),
        )
        .arg(
            Arg::with_name("count")
                .value_name("COUNT")
                .help("Show counts")
                .short("c")
                .long("count")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        in_file: matches.value_of("in_file").map(str::to_string).unwrap(),
        out_file: matches.value_of("out_file").map(String::from),
        count: matches.is_present("count"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut writer = output(&config.out_file)?;

    let mut line = String::new();
    let mut current_line: Option<String> = None;
    let mut current_line_count = 0;

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            if let Some(current) = &current_line {
                print_result(&mut writer, config.count, current_line_count, &current)?;
            }

            break;
        }

        let trimmed_line = line.trim();
        let trimmed_current = current_line.as_ref().map(|s| s.trim()).unwrap_or("");

        if trimmed_line == trimmed_current {
            current_line_count += 1;
        } else {
            if let Some(current) = &current_line {
                print_result(&mut writer, config.count, current_line_count, &current)?;
            }
            current_line = Some(line.clone());
            current_line_count = 1;
        }

        line.clear();
    }

    Ok(())
}

fn print_result(writer: &mut impl Write, count: bool, num: usize, line: &str) -> MyResult<()> {
    if count {
        Ok(write!(writer, "{:>4} {}", num, line)?)
    } else {
        Ok(write!(writer, "{}", line)?)
    }
}

fn output(filename: &Option<String>) -> MyResult<Box<dyn Write>> {
    if let Some(file) = filename {
        Ok(Box::new(File::create(file)?))
    } else {
        Ok(Box::new(io::stdout()))
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
