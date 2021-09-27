use chrono::{Datelike, Local, NaiveDate};
use clap::{App, Arg};
use std::error::Error;

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
