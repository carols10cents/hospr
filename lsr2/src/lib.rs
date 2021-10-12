use clap::{App, Arg};
use std::{error::Error, fs, path::PathBuf};
use tabular::{Row, Table};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    long: bool,
    show_hidden: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("lsr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust ls")
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .help("Files and/or directories")
                .default_value(".")
                .multiple(true),
        )
        .arg(
            Arg::with_name("long")
                .takes_value(false)
                .help("Long listing")
                .short("l")
                .long("long"),
        )
        .arg(
            Arg::with_name("all")
                .takes_value(false)
                .help("Show all files")
                .short("a")
                .long("all"),
        )
        .get_matches();

    Ok(Config {
        paths: matches.values_of_lossy("paths").unwrap(),
        long: matches.is_present("long"),
        show_hidden: matches.is_present("all"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let paths = find_files(&config.paths, config.show_hidden)?;
    for path in paths {
        println!("{}", path.display());
    }
    Ok(())
}

fn find_files(paths: &[String], show_hidden: bool) -> MyResult<Vec<PathBuf>> {
    let mut files = vec![];

    for path in paths {
        let metadata = match fs::metadata(path) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("{}: {}", path, e);
                continue;
            }
        };
        if metadata.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let hidden = entry
                    .file_name()
                    .as_os_str()
                    .to_string_lossy()
                    .starts_with(".");

                if show_hidden || !hidden {
                    files.push(entry.path());
                }
            }
        } else {
            files.push(PathBuf::from(path));
        }
    }

    Ok(files)
}

fn format_output(paths: &[PathBuf]) -> MyResult<String> {
    // 1 2 3 4 5 6 7 8
    let fmt = "{:<}{:<} {:>} {:<} {:<} {:>} {:<} {:<}";
    let mut table = Table::new(fmt);
    for path in paths {
        table.add_row(
            Row::new()
                .with_cell("") // 1 "d" or "-"
                .with_cell("") // 2 permissions
                .with_cell("") // 3 number of links
                .with_cell("") // 4 user name
                .with_cell("") // 5 group name
                .with_cell("") // 6 size
                .with_cell("") // 7 modification
                .with_cell(""), // 8 path
        );
    }
    Ok(format!("{}", table))
}

/// Given a file mode in octal format like 0o751,
/// return a string like "rwxr-x--x"
pub fn format_mode(mode: u16) -> String {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::find_files;
    use std::collections::HashSet;

    #[test]
    fn test_find_files() {
        let res = find_files(&["tests/inputs".to_string()], false);
        assert!(res.is_ok());

        let paths = res.unwrap();
        assert_eq!(paths.len(), 4);

        let filenames: HashSet<String> = paths.iter().map(|f| f.display().to_string()).collect();
        let expected: HashSet<String> = [
            "tests/inputs/bustle.txt",
            "tests/inputs/dir",
            "tests/inputs/empty.txt",
            "tests/inputs/fox.txt",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect();
        assert_eq!(filenames, expected);
    }

    #[test]
    fn test_find_files_hidden() {
        let res = find_files(&["tests/inputs".to_string()], true);
        assert!(res.is_ok());

        let paths = res.unwrap();
        assert_eq!(paths.len(), 5);

        let filenames: HashSet<String> = paths.iter().map(|f| f.display().to_string()).collect();
        let expected: HashSet<String> = [
            "tests/inputs/.hidden",
            "tests/inputs/bustle.txt",
            "tests/inputs/dir",
            "tests/inputs/empty.txt",
            "tests/inputs/fox.txt",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect();
        assert_eq!(filenames, expected);
    }
}
