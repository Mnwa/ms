#![doc = "Fast converter various time formats into milliseconds."]
#![doc(issue_tracker_base_url = "https://github.com/Mnwa/ms/issues/")]
#![doc(html_root_url = "https://docs.rs/ms-converter/")]

use std::fmt::Formatter;

pub const SECOND: f64 = 1000_f64;
pub const MINUTE: f64 = SECOND * 60_f64;
pub const HOUR: f64 = MINUTE * 60_f64;
pub const DAY: f64 = HOUR * 24_f64;
pub const WEEK: f64 = DAY * 7_f64;
pub const YEAR: f64 = DAY * 365.25_f64;

/// Fast abstraction for converting human-like times into milliseconds.
/// `ms` function gets an str slice and returns how much milliseconds in your pattern.
///
/// Example:
/// ```
/// use crate::ms_converter::ms;
///
/// let value = ms("1d").unwrap();
/// assert_eq!(value, 86400000)
/// ```
#[inline]
pub fn ms(s: &str) -> Result<i64, Error> {
    let value_count = s
        .chars()
        .take_while(|c| c >= &'0' && c <= &'9' || c == &'.' || c == &'-')
        .count();

    let (value, postfix) = s.split_at(value_count);

    let value = value
        .parse::<f64>()
        .map_or(Err(Error::new("invalid value")), |v| Ok(v))?;
    let postfix = postfix.trim();

    let ret = match postfix {
        "years" | "year" | "yrs" | "yr" | "y" => value * YEAR,
        "weeks" | "week" | "w" => value * WEEK,
        "days" | "day" | "d" => value * DAY,
        "hours" | "hour" | "hrs" | "hr" | "h" => value * HOUR,
        "minutes" | "minute" | "mins" | "min" | "m" => value * MINUTE,
        "seconds" | "second" | "secs" | "sec" | "s" => value * SECOND,
        "milliseconds" | "millisecond" | "msecs" | "msec" | "ms" | "" => value,
        _ => return Err(Error::new("invalid postfix")),
    };

    Ok(ret.round() as i64)
}

/// Zero cost converter from human-like time into a number.
/// In the first argument, you need to pass type of your number (`i64`, `f64` and etc).
/// The second argument is human-time construction, like `1 day`, `2 h`.
/// The output will be a number with type what you set in the first argument.
///
/// This macro will be precalculated in compilation time. Also, you can use ms_expr with constants:
///
/// ```
/// use crate::ms_converter::ms_expr;
///
/// const VALUE: f64 = ms_expr!(f64, 2.5 hrs);
/// assert_eq!(VALUE, 9000000.)
/// ```
///
/// Example:
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
