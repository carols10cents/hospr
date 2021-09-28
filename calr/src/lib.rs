use chrono::NaiveDate;
use chrono::{Datelike, Local, Month};
use clap::{App, Arg};
use colorize::AnsiColor;
use num_traits::FromPrimitive;
use std::error::Error;
use std::str::FromStr;

const MONTH_NAMES: [&str; 12] = [
    "january",
    "february",
    "march",
    "april",
    "may",
    "june",
    "july",
    "august",
    "september",
    "october",
    "november",
    "december",
];

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
            Arg::with_name("month")
                .value_name("MONTH")
                .short("m")
                .help("Month name or number (1-12)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("show_current_year")
                .short("y")
                .long("year")
                .help("Show whole current year")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("year")
                .value_name("YEAR")
                .help("Year (1-9999)"),
        )
        .get_matches();

    let mut month = matches.value_of("month").map(parse_month).transpose()?;
    let mut year = matches.value_of("year").map(parse_year).transpose()?;

    let today = Local::today();
    if matches.is_present("show_current_year") {
        month = None;
        year = Some(today.year());
    } else if month.is_none() && year.is_none() {
        month = Some(today.month());
        year = Some(today.year());
    }

    Ok(Config {
        month,
        year: year.unwrap_or_else(|| today.year()),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let today = Local::today().naive_local();
    match config.month {
        Some(m) => {
            for line in format_month(config.year, m, true, today) {
                println!("{}", line);
            }
        }
        None => {
            print!("{}", format!("{:^61}", config.year).trim_end());

            let mut months = (1..=12u32).into_iter();

            loop {
                let month_left = if let Some(n) = months.next() {
                    n
                } else {
                    break;
                };
                let month_center = months.next().unwrap();
                let month_right = months.next().unwrap();

                println!();

                let month_left = format_month(config.year, month_left, false, today).into_iter();
                let month_center =
                    format_month(config.year, month_center, false, today).into_iter();
                let month_right = format_month(config.year, month_right, false, today).into_iter();

                for ((left, right), center) in month_left.zip(month_center).zip(month_right) {
                    println!("{}{}{}", left, right, center);
                }
            }
        }
    }
    Ok(())
}

fn parse_month(month: &str) -> MyResult<u32> {
    match parse_int(month) {
        Ok(num) => {
            if (1..=12).contains(&num) {
                Ok(num)
            } else {
                Err(format!("month \"{}\" not in the range 1..12", month).into())
            }
        }
        _ => {
            let lower = &month.to_lowercase();
            let matches: Vec<_> = MONTH_NAMES
                .iter()
                .enumerate()
                .filter_map(|(i, name)| {
                    if name.starts_with(lower) {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .collect();

            if matches.len() == 1 {
                Ok(matches[0] as u32)
            } else {
                Err(From::from(format!("Invalid month \"{}\"", month)))
            }
        }
    }
}

fn parse_int<T: FromStr>(val: &str) -> MyResult<T> {
    val.trim()
        .parse()
        .map_err(|_| format!("Invalid integer \"{}\"", val).into())
}

fn parse_year(year: &str) -> MyResult<i32> {
    parse_int(year).and_then(|num| {
        if (1..=9999).contains(&num) {
            Ok(num)
        } else {
            Err(format!("year \"{}\" not in the range 1..9999", year).into())
        }
    })
}

fn format_month(year: i32, month: u32, print_year: bool, today: NaiveDate) -> Vec<String> {
    let mut output = vec![];

    let mut days = NaiveDate::from_ymd(year, month, 1).iter_days();

    let start_date = days.next().unwrap();

    output.push(format!(
        "{:^20}  ",
        if print_year {
            format!(
                "{} {}",
                Month::from_u32(start_date.month()).unwrap().name(),
                start_date.year()
            )
        } else {
            Month::from_u32(start_date.month())
                .unwrap()
                .name()
                .to_string()
        }
    ));

    output.push("Su Mo Tu We Th Fr Sa  ".into());

    let mut week = String::new();

    // Initial padding
    week.push_str(&" ".repeat(3 * start_date.weekday().num_days_from_sunday() as usize));
    week.push_str(&day_display(start_date, today));

    while let Some(day) = days.next() {
        if day.month() != month {
            break;
        }

        if day.weekday().num_days_from_sunday() == 0 {
            week.push_str(" ");
            output.push(week.clone());
            week.clear();
        }

        week.push_str(&day_display(day, today));
    }

    output.push(format!("{:<22}", week));

    while output.len() < 8 {
        output.push(" ".repeat(22));
    }

    output
}

fn day_display(day: NaiveDate, today: NaiveDate) -> String {
    let mut day_display = format!("{:>2}", day.day());

    if day == today {
        day_display = day_display.reverse();
    }

    format!("{} ", day_display)
}

fn last_day_in_month(year: i32, month: u32) -> NaiveDate {
    // The first day of the next month...
    let (y, m) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    NaiveDate::from_ymd(y, m, 1).pred()
}

#[cfg(test)]
mod tests {
    use super::{format_month, last_day_in_month, parse_int, parse_month, parse_year};
    use chrono::{Local, NaiveDate};

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

    #[test]
    fn test_parse_month() {
        let res = parse_month("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("12");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 12u32);

        let res = parse_month("jan");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1u32);

        let res = parse_month("0");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"0\" not in the range 1..12"
        );

        let res = parse_month("13");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "month \"13\" not in the range 1..12"
        );

        let res = parse_month("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Invalid month \"foo\"");
    }

    #[test]
    fn test_format_month() {
        let today = Local::today().naive_local();
        let april = vec![
            "     April 2020       ",
            "Su Mo Tu We Th Fr Sa  ",
            "          1  2  3  4  ",
            " 5  6  7  8  9 10 11  ",
            "12 13 14 15 16 17 18  ",
            "19 20 21 22 23 24 25  ",
            "26 27 28 29 30        ",
            "                      ",
        ];
        assert_eq!(format_month(2020, 4, true, today), april);

        let may = vec![
            "      May 2020        ",
            "Su Mo Tu We Th Fr Sa  ",
            "                1  2  ",
            " 3  4  5  6  7  8  9  ",
            "10 11 12 13 14 15 16  ",
            "17 18 19 20 21 22 23  ",
            "24 25 26 27 28 29 30  ",
            "31                    ",
        ];
        assert_eq!(format_month(2020, 5, true, today), may);

        let april_hl = vec![
            "     April 2021       ",
            "Su Mo Tu We Th Fr Sa  ",
            "             1  2  3  ",
            " 4  5  6 \u{1b}[7m 7\u{1b}[0;39;49m  8  9 10  ",
            "11 12 13 14 15 16 17  ",
            "18 19 20 21 22 23 24  ",
            "25 26 27 28 29 30     ",
            "                      ",
        ];
        for line in &april_hl {
            println!("{}", line);
        }
        let today = NaiveDate::from_ymd(2021, 4, 7);
        assert_eq!(format_month(2021, 4, true, today), april_hl);
    }

    #[test]
    fn test_last_day_in_month() {
        assert_eq!(last_day_in_month(2020, 1), NaiveDate::from_ymd(2020, 1, 31));
        assert_eq!(last_day_in_month(2020, 2), NaiveDate::from_ymd(2020, 2, 29));
        assert_eq!(last_day_in_month(2020, 4), NaiveDate::from_ymd(2020, 4, 30));
    }
}
