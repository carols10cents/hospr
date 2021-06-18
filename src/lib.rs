use clap::{App, Arg};
use std::{error::Error, path::Path, fs};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        let contents = fs::read_to_string(filename)?;

        if !contents.is_empty() {
            let lines: Vec<_> = contents.lines().collect();
            println!("{}", file_lines(&lines));
        }
    }
    Ok(())
}

fn file_lines(lines: &[&str]) -> String {
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_lines_retuns_empty_string() {
        assert_eq!(file_lines(&[]), "");
    }

    #[test]
    fn one_line_retuns_itself() {
        assert_eq!(file_lines(&["hello how are you"]), "hello how are you");
    }

    #[test]
    fn one_line_and_one_blank_line_returns_itself() {
        assert_eq!(file_lines(&["hello how are you", ""]), "hello how are you\n");
    }
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
            Arg::with_name("number_noblank")
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
