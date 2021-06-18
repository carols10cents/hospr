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
        .get_matches();

    let files = matches.values_of_lossy("file").unwrap();

    for file in files {
        match fs::read_to_string(&file) {
            Ok(contents) => println!("{}", contents),
            Err(_) => {
                eprintln!("\"{}\" is not a valid file.", file);
                std::process::exit(1);
            }
        }
    }
}
