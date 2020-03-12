#![feature(test)]
extern crate test;

use std::fmt::Formatter;
use std::str::FromStr;

const SECOND: f64 = 1000_f64;
const MINUTE: f64 = SECOND * 60_f64;
const HOUR: f64 = MINUTE * 60_f64;
const DAY: f64 = HOUR * 24_f64;
const WEEK: f64 = DAY * 7_f64;
const YEAR: f64 = DAY * 365.25_f64;

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
    let postfix = str.parse::<DatePostfixes>()?;

    Ok(match postfix {
        DatePostfixes::Year(v) => v * YEAR,
        DatePostfixes::Week(v) => v * WEEK,
        DatePostfixes::Day(v) => v * DAY,
        DatePostfixes::Hour(v) => v * HOUR,
        DatePostfixes::Minute(v) => v * MINUTE,
        DatePostfixes::Second(v) => v * SECOND,
        DatePostfixes::Millisecond(v) => v,
    })
}

#[derive(Debug)]
enum DatePostfixes {
    Year(f64),
    Week(f64),
    Day(f64),
    Hour(f64),
    Minute(f64),
    Second(f64),
    Millisecond(f64),
}

impl FromStr for DatePostfixes {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value_count = s
            .chars()
            .take_while(|c| c >= &'0' && c <= &'9' || c == &'.' || c == &'-')
            .count();
        let (value, postfix) = s.split_at(value_count);
        let value = value
            .parse::<f64>()
            .map_or(Err(Error::new("invalid value")), |v| Ok(v))?;

        match postfix.trim() {
            "years" | "year" | "yrs" | "yr" | "y" => Ok(DatePostfixes::Year(value)),
            "weeks" | "week" | "w" => Ok(DatePostfixes::Week(value)),
            "days" | "day" | "d" => Ok(DatePostfixes::Day(value)),
            "hours" | "hour" | "hrs" | "hr" | "h" => Ok(DatePostfixes::Hour(value)),
            "minutes" | "minute" | "mins" | "min" | "m" => Ok(DatePostfixes::Minute(value)),
            "seconds" | "second" | "secs" | "sec" | "s" => Ok(DatePostfixes::Second(value)),
            "milliseconds" | "millisecond" | "msecs" | "msec" | "ms" | "" => {
                Ok(DatePostfixes::Millisecond(value))
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
