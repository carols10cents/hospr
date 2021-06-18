use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Carol (Nichols || Goulding)")
        .about("Rust cat")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file")
                .multiple(true),
        )
        .arg(
            Arg::with_name("number")
                .help("number all output lines")
                .takes_value(false)
                .long("number")
                .short("n"),
        )
        .get_matches();

    let files = matches.values_of_lossy("file").unwrap();
    let number = matches.is_present("number");

    let output = files
        .iter()
        .map(|file| match fs::read_to_string(&file) {
            Ok(contents) => {
                if number {
                    contents
                        .lines()
                        .enumerate()
                        .map(|(line_num, line)| format!("{:>6}\t{}", line_num + 1, line))
                        .collect::<Vec<_>>()
                        .join("\n")
                } else {
                    contents
                }
            }
            Err(_) => {
                eprintln!("\"{}\" is not a valid file.", file);
                std::process::exit(1);
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    print!("{}", output);
}
