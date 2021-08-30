use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file1: String,
    file2: String,
    suppress_col1: bool,
    suppress_col2: bool,
    suppress_col3: bool,
    insensitive: bool,
    delimiter: String,
}
