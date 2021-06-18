use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        let file = File::open(filename)?;
        let lines = io::BufReader::new(file).lines();

        for (line_num, line_result) in lines.enumerate() {
            let line = line_result?;

            if config.number_lines {
                println!("{:6}\t{}", line_num + 1, line);
            } else {
                println!("{}", line);
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Carol (Nichols || Goulding)")
        .about("Rust cat")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("number")
                .help("number all output lines")
                .takes_value(false)
                .long("number")
                .short("n"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .help("number nonempty output lines, overrides -n")
                .takes_value(false)
                .long("number-nonblank")
                .short("b"),
        )
        .get_matches();

    let files = matches.values_of_lossy("file").unwrap();
    for file in files
        .iter()
        .filter(|&name| name != "-" && !Path::new(name).exists())
    {
        return Err(From::from(format!("\"{}\" is not a valid file.", file)));
    }

    Ok(Config {
        files: files,
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}
