#![doc = "Fast converter various time formats into milliseconds."]
#![doc(issue_tracker_base_url = "https://github.com/Mnwa/ms/issues/")]
#![doc(html_root_url = "https://docs.rs/ms-converter/")]

use std::fmt::Formatter;
use std::time::Duration;

/// How many milliseconds in one second
pub const SECOND: f64 = 1000_f64;
/// How many milliseconds in one minute
pub const MINUTE: f64 = SECOND * 60_f64;
/// How many milliseconds in one hour
pub const HOUR: f64 = MINUTE * 60_f64;
/// How many milliseconds in one day
pub const DAY: f64 = HOUR * 24_f64;
/// How many milliseconds in one week
pub const WEEK: f64 = DAY * 7_f64;
/// How many milliseconds in one year
pub const YEAR: f64 = DAY * 365.25_f64;

/// ### Description
/// Fast abstraction for converting human-like times into milliseconds.
/// `ms` function gets an str slice and returns how much milliseconds in your pattern.
///
/// ### Usage
/// ```
/// use crate::ms_converter::ms;
///
/// let value = ms("1d").unwrap();
/// assert_eq!(value, 86400000)
/// ```
///
/// ### Supported time strings
/// * **Years:** `years`, `year`, `yrs`, `yr`, `y`
/// * **Weeks:** `weeks`, `week`, `w`
/// * **Days:** `days`, `day`, `d`
/// * **Hours:** `hours`, `hour`, `hrs`, `hr`, `h`
/// * **Minutes:** `minutes`, `minute`, `mins`, `min`, `m`
/// * **Seconds:** `seconds`, `second`, `secs`, `sec`, `s`
/// * **Milliseconds:** `milliseconds`, `millisecond`, `msecs`, `msec`, `ms` and empty postfix
#[inline(always)]
pub fn ms(s: &str) -> Result<i64, Error> {
    let (value, postfix) = s
        .find(|c: char| !matches!(c, '0'..='9' | '.' | '-'))
        .map_or((s, ""), move |vi| s.split_at(vi));

    let postfix = postfix.trim();
    value
        .parse::<f64>()
        .map_err(|_| Error::new("invalid value"))
        .and_then(move |value| match postfix {
            "years" | "year" | "yrs" | "yr" | "y" => Ok(value * YEAR),
            "weeks" | "week" | "w" => Ok(value * WEEK),
            "days" | "day" | "d" => Ok(value * DAY),
            "hours" | "hour" | "hrs" | "hr" | "h" => Ok(value * HOUR),
            "minutes" | "minute" | "mins" | "min" | "m" => Ok(value * MINUTE),
            "seconds" | "second" | "secs" | "sec" | "s" => Ok(value * SECOND),
            "milliseconds" | "millisecond" | "msecs" | "msec" | "ms" | "" => Ok(value),
            _ => return Err(Error::new("invalid postfix")),
        })
        .map(|v| v.round() as i64)
}

/// ### Description
/// Zero cost converter from human-like time into a number.
/// In the first argument, you need to pass type of your number (`i64`, `f64` and etc).
/// The second argument is human-time construction, like `1 day`, `2 h`.
/// The output will be a number with type what you set in the first argument.
///
/// **This macro will be precalculated in compilation time.** Also, you can use ms_expr with constants:
///
/// ```
/// use crate::ms_converter::ms_expr;
///
/// const VALUE: f64 = ms_expr!(f64, 2.5 hrs);
/// assert_eq!(VALUE, 9000000.)
/// ```
///
/// ### Usage
/// ```
/// use crate::ms_converter::ms_expr;
///
/// assert_eq!(ms_expr!(i64, 1 d), 86400000)
/// ```
#[macro_export]
macro_rules! ms_expr {
    ($type:ty, $x:literal $(milliseconds)?$(millisecond)?$(msecs)?$(msec)?$(ms)?) => {{
        let x: $type = $x;
        x
    }};
    ($type:ty, $x:literal $(seconds)?$(second)?$(secs)?$(sec)?$(s)?) => {{
        let x: $type = $x * ($crate::SECOND as $type);
        x
    }};
    ($type:ty, $x:literal $(minutes)?$(minute)?$(mins)?$(min)?$(m)?) => {{
        let x: $type = $x * ($crate::MINUTE as $type);
        x
    }};
    ($type:ty, $x:literal $(hours)?$(hour)?$(hrs)?$(hr)?$(h)?) => {{
        let x: $type = $x * ($crate::HOUR as $type);
        x
    }};
    ($type:ty, $x:literal $(days)?$(day)?$(d)?) => {{
        let x: $type = $x * ($crate::DAY as $type);
        x
    }};
    ($type:ty, $x:literal $(weeks)?$(week)?$(w)?) => {{
        let x: $type = $x * ($crate::WEEK as $type);
        x
    }};
    ($type:ty, $x:literal $(years)?$(year)?$(yrs)?$(yr)?$(y)?) => {{
        let x: $type = $x * ($crate::YEAR as $type);
        x
    }};
}

/// ### Description
/// Ms into time is the abstraction on `ms` function, which converts result into `time.Duration` type.
/// `ms_into_time` function gets an str slice and returns `time.Duration`.
/// `ms_into_time` **has some limitations**, it's not working with negative values:
/// ```
/// use crate::ms_converter::ms_into_time;
///
/// let value = ms_into_time("-1d").is_err();
/// assert_eq!(value, true)
/// ```
///
/// ### Usage
/// ```
/// use crate::ms_converter::ms_into_time;
///
/// let value = ms_into_time("1d").unwrap();
/// assert_eq!(value.as_millis(), 86400000)
/// ```
pub fn ms_into_time(s: &str) -> Result<Duration, Error> {
    let milliseconds = ms(s)?;
    if milliseconds < 0 {
        return Err(Error::new("time.Duration cannot work with negative values"));
    }
    Ok(Duration::from_millis(milliseconds as u64))
}

/// Error which return `ms_converter` functions in runtime, if something is going wrong.
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
