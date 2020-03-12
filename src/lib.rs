#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use regex::{Captures, Regex};
use std::fmt::Formatter;
use std::str::FromStr;

const SECOND: f64 = 1000_f64;
const MINUTE: f64 = SECOND * 60_f64;
const HOUR: f64 = MINUTE * 60_f64;
const DAY: f64 = HOUR * 24_f64;
const WEEK: f64 = DAY * 7_f64;
const YEAR: f64 = DAY * 365.25_f64;

lazy_static! {
    static ref REG: Regex = Regex::new(r"^(-?(?:\d+)?\.?\d+) *(milliseconds?|msecs?|ms|seconds?|secs?|s|minutes?|mins?|m|hours?|hrs?|h|days?|d|weeks?|w|years?|yrs?|y)?$").unwrap();
}

/// Convert time string to a milliseconds
///
/// Example:
/// ```
/// use crate::ms_converter::ms;
///
/// let value = ms("1d").unwrap();
/// assert_eq!(value, 86400000.)
/// ```
pub fn ms(str: &str) -> Result<f64, Error> {
    let captures: Captures = REG
        .captures(str)
        .map_or(Err(Error::new("fail to parse input")), |captures| {
            Ok(captures)
        })?;

    let postfix = captures
        .get(2)
        .map_or("ms", |m| m.as_str())
        .parse::<DatePostfixes>()?;
    let value: f64 = captures
        .get(1)
        .map(|v| v.as_str())
        .and_then(|v| v.parse::<f64>().ok())
        .map_or(Err(Error::new("fail to parse value")), |v| Ok(v))?;

    Ok(match postfix {
        DatePostfixes::Year => value * YEAR,
        DatePostfixes::Week => value * WEEK,
        DatePostfixes::Day => value * DAY,
        DatePostfixes::Hour => value * HOUR,
        DatePostfixes::Minute => value * MINUTE,
        DatePostfixes::Second => value * SECOND,
        DatePostfixes::Millisecond => value,
    })
}

#[derive(Debug)]
enum DatePostfixes {
    Year,
    Week,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
}

impl FromStr for DatePostfixes {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "years" | "year" | "yrs" | "yr" | "y" => Ok(DatePostfixes::Year),
            "weeks" | "week" | "w" => Ok(DatePostfixes::Week),
            "days" | "day" | "d" => Ok(DatePostfixes::Day),
            "hours" | "hour" | "hrs" | "hr" | "h" => Ok(DatePostfixes::Hour),
            "minutes" | "minute" | "mins" | "min" | "m" => Ok(DatePostfixes::Minute),
            "seconds" | "second" | "secs" | "sec" | "s" => Ok(DatePostfixes::Second),
            "milliseconds" | "millisecond" | "msecs" | "msec" | "ms" => {
                Ok(DatePostfixes::Millisecond)
            }
            _ => Err(Error::new("invalid postfix")),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    message: &'static str,
}

impl Error {
    pub fn new(message: &'static str) -> Error {
        Error { message }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests;
