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
        .arg(
            Arg::with_name("number_noblank")
                .help("number nonempty output lines, overrides -n")
                .takes_value(false)
                .long("number-nonblank")
                .short("b"),
        )
        .get_matches();

    let files = matches.values_of_lossy("file").unwrap();
    let number = matches.is_present("number");
    let number_noblank = matches.is_present("number_noblank");

    let output = files
        .iter()
        .map(|file| match fs::read_to_string(&file) {
            Ok(contents) => {
                if number_noblank {
                    let mut output = String::new();
                    let mut line_num = 1;
                    for line in contents.lines() {
                        if line.is_empty() {
                            output.push_str("\n");
                        } else {
                            output.push_str(&format!("{:>6}\t{}\n", line_num, line));
                            line_num += 1;
                        }
                    }
                    output = output.trim_end().to_string();
                    output
                } else if number {
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
