use chrono::{Datelike, Local, NaiveDate};
use clap::{App, Arg};
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("calr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cal")
        .arg(
            Arg::with_name("year")
                .short("y")
                .long("year")
                .takes_value(false)
                .help("Show whole current year"),
        )
        .arg(
            Arg::with_name("month")
                .short("m")
                .value_name("MONTH")
                .help("Month name or number (1-12)"),
        )
        .arg(
            Arg::with_name("year_value")
                .value_name("YEAR")
                .help("Year (1-9999)"),
        )
        .get_matches();

    let now = Local::now();

    let yv = matches.value_of("year_value");

    let month = match (matches.is_present("year"), matches.value_of("month"), yv) {
        (true, _, _) => None,
        (_, Some(m), _) => Some(parse_month(m)?),
        (_, None, Some(_)) => None,
        (_, None, None) => Some(now.month()),
    };

    let year = yv.map(|y| y.parse()).transpose()?.unwrap_or(now.year());

    Ok(Config { month, year })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

fn parse_month(m: &str) -> MyResult<u32> {
    m.parse::<u32>()
        .or_else(|_| {
            dbg!(NaiveDate::parse_from_str(&format!("{} 2021", m), "%B %Y")).map(|r| r.month())
        })
        .map_err(|_| format!("Invalid month \"{}\"", m).into())
        .and_then(|n| match n {
            1..=12 => Ok(n),
            _ => Err(format!("month \"{}\" not in the range 1..12", n).into()),
        })
}

fn parse_int<T: FromStr>(val: &str) -> MyResult<T> {
    val.parse()
        .map_err(|_| format!("Invalid integer \"{}\"", val).into())
}

fn parse_year(year: &str) -> MyResult<i32> {
    parse_int(year).and_then(|y| match y {
        1..=9999 => Ok(y),
        _ => Err(format!("year \"{}\" not in the range 1..9999", year).into()),
    })
}

#[cfg(test)]
mod tests {
    use super::{parse_int, parse_year};

    #[test]
    fn test_parse_int() {
        // Parse positive int as usize
        let res = parse_int::<usize>("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1usize);

        // Parse negative int as i32
        let res = parse_int::<i32>("-1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), -1i32);

        // Fail on a string
        let res = parse_int::<i64>("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid integer \"foo\"");
    }

    #[test]
    fn test_parse_year() {
        let res = parse_year("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1i32);

        let res = parse_year("9999");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 9999i32);

        let res = parse_year("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "year \"0\" not in the range 1..9999"
        );

        let res = parse_year("10000");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "year \"10000\" not in the range 1..9999"
        );

        let res = parse_year("foo");
        assert!(res.is_err());
    }
}
