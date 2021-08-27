use crate::Extract::*;
use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<usize>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("cutr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cut")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("Selected bytes")
                .short("b")
                .long("bytes")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .help("Selected characters")
                .short("c")
                .long("chars")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("delimiter")
                .value_name("DELIMITER")
                .help("Field delimiter")
                .short("d")
                .long("delim")
                .default_value("\t")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("fields")
                .value_name("FIELDS")
                .help("Selected fields")
                .short("f")
                .long("fields")
                .takes_value(true),
        )
        .get_matches();

    panic!();
    // Ok(Config {
    //     files: matches.values_of_lossy("files").unwrap(),
    //     delimiter,
    //     extract,
    // })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", &config);
    Ok(())
}

fn parse_pos(range: &str) -> MyResult<PositionList> {
    let mut result = vec![];

    for value in range.split(',') {
        let mut parts = value.split('-').fuse();
        match (parts.next(), parts.next()) {
            (Some(first), Some(second)) => {
                let first_num: usize = first.parse().map_err::<Box<dyn Error>, _>(|_| {
                    format!("illegal list value: \"{}\"", first).into()
                })?;
                let second_num: usize = second.parse().map_err::<Box<dyn Error>, _>(|_| {
                    format!("illegal list value: \"{}\"", second).into()
                })?;

                if first_num >= second_num {
                    return Err(format!(
                        "First number in range ({}) must be lower than second number ({})",
                        first_num, second_num
                    )
                    .into());
                }

                for i in first_num..=second_num {
                    result.push(i - 1);
                }
            }
            (Some(first), None) => {
                let num: usize = first.parse().map_err::<Box<dyn Error>, _>(|_| {
                    format!("illegal list value: \"{}\"", first).into()
                })?;
                if num == 0 {
                    return Err("illegal list value: \"0\"".into());
                }
                result.push(num - 1)
            }
            _ => unreachable!(),
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::parse_pos;

    #[test]
    fn test_parse_pos() {
        assert!(parse_pos("").is_err());

        let res = parse_pos("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

        let res = parse_pos("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        let res = parse_pos("1,a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        let res = parse_pos("2-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );

        let res = parse_pos("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0]);

        let res = parse_pos("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0, 2]);

        let res = parse_pos("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0, 1, 2]);

        let res = parse_pos("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0, 6, 2, 3, 4]);
    }
}
