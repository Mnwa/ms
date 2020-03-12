#![doc = "Fast converter various time formats into milliseconds."]
#![doc(issue_tracker_base_url = "https://github.com/Mnwa/ms/issues/")]
#![doc(html_root_url = "https://docs.rs/ms-converter/")]

use std::fmt::Formatter;

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
pub fn ms(s: &str) -> Result<f64, Error> {
    let value_count = s
        .chars()
        .take_while(|c| c >= &'0' && c <= &'9' || c == &'.' || c == &'-')
        .count();

    let (value, postfix) = s.split_at(value_count);

    let value = value
        .parse::<f64>()
        .map_or(Err(Error::new("invalid value")), |v| Ok(v))?;
    let postfix = postfix.trim();

    match postfix {
        "years" | "year" | "yrs" | "yr" | "y" => Ok(value * YEAR),
        "weeks" | "week" | "w" => Ok(value * WEEK),
        "days" | "day" | "d" => Ok(value * DAY),
        "hours" | "hour" | "hrs" | "hr" | "h" => Ok(value * HOUR),
        "minutes" | "minute" | "mins" | "min" | "m" => Ok(value * MINUTE),
        "seconds" | "second" | "secs" | "sec" | "s" => Ok(value * SECOND),
        "milliseconds" | "millisecond" | "msecs" | "msec" | "ms" | "" => Ok(value),
        _ => Err(Error::new("invalid postfix")),
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
