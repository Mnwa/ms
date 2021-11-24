/*!
Fast abstraction for converting human-like times into milliseconds.

There are two ways to calculate milliseconds:
* In the runtime `crate::ms_converter::ms`
* In the compilation time `crate::ms_converter::ms_expr`

## Usage

### Running ms converter in Runtime:
```rust
use crate::ms_converter::ms;

let value = ms("1d").unwrap();
assert_eq!(value, 86400000)
```

### Convert ms in the compilation step:
```rust
use crate::ms_converter::ms_expr;

const VALUE: i64 = ms_expr!(i64, 1 d);
assert_eq!(VALUE, 86400000)
```

### Convert ms into `time.Duration`
```rust
use crate::ms_converter::ms_into_time;

let value = ms_into_time("1d").unwrap();
assert_eq!(value.as_millis(), 86400000)
```

### Convert milliseconds into human-like time string
```
use crate::ms_converter::{get_duration_by_postfix, DAY};

let value = get_duration_by_postfix(DAY as i64, " day").unwrap();
assert_eq!(value, "1 day")
```

### Convert milliseconds into human-like time string without postfix
```
use crate::ms_converter::{get_max_possible_duration, DAY};

let value = get_max_possible_duration(DAY as i64).unwrap();
assert_eq!(value, "1d")
```

### Convert milliseconds into long human-like time string without postfix
```
use crate::ms_converter::{get_max_possible_duration_long, WEEK};

let value = get_max_possible_duration_long(2 * WEEK as i64).unwrap();
assert_eq!(value, "14 days") // Max possible period is a day
```

## Supported time strings
* **Years:** `years`, `year`, `yrs`, `yr`, `y`
* **Weeks:** `weeks`, `week`, `w`
* **Days:** `days`, `day`, `d`
* **Hours:** `hours`, `hour`, `hrs`, `hr`, `h`
* **Minutes:** `minutes`, `minute`, `mins`, `min`, `m`
* **Seconds:** `seconds`, `second`, `secs`, `sec`, `s`
* **Milliseconds:** `milliseconds`, `millisecond`, `msecs`, `msec`, `ms` and empty postfix
*/

#![doc(issue_tracker_base_url = "https://github.com/Mnwa/ms/issues/")]
#![doc(html_root_url = "https://docs.rs/ms-converter/")]

use std::borrow::Cow;
use std::fmt::Formatter;
use std::format;
use std::ops::{Add, Mul, Sub};
use std::string::String;
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

/// Fast abstraction for converting human-like times into milliseconds.
/// `ms` function gets an str slice or String and returns how much milliseconds in your pattern.
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
pub fn ms<'a, T>(s: T) -> Result<i64, Error>
where
    T: Into<Cow<'a, str>>,
{
    let s = &*s.into();

    let (value, postfix): (&str, &str) = s
        .find(|c: char| !matches!(c, '0'..='9' | '.' | '-' | '+'))
        .map_or((s, ""), |vi| s.split_at(vi));

    let postfix = get_byte_postfix(postfix);

    parse(value.as_bytes())
        .and_then(move |value| Ok(get_modification(postfix)? * value))
        .map(|v| v.round() as i64)
}

/// Getting human-like time from milliseconds.
/// `get_duration_by_postfix` function gets a milliseconds count and str slice or String as postfix
/// and returns a string with your time.
///
/// ### Usage
/// ```
/// use crate::ms_converter::{get_duration_by_postfix, DAY};
///
/// let value = get_duration_by_postfix(1 * DAY as i64, "day").unwrap();
/// assert_eq!(value, "1day")
/// ```
///
/// You can add the space to start of you prefix to get space between date and postfix on return.
/// ```
/// use crate::ms_converter::{get_duration_by_postfix, DAY};
///
/// let value = get_duration_by_postfix(DAY as i64, " day").unwrap();
/// assert_eq!(value, "1 day")
/// ```
/// also you can a pass negative values
/// ```
/// use crate::ms_converter::{get_duration_by_postfix, DAY};
///
/// let value = get_duration_by_postfix(-DAY as i64, " day").unwrap();
/// assert_eq!(value, "-1 day")
/// ```

/// ### Supported postfixes
/// * **Years:** `years`, `year`, `yrs`, `yr`, `y`
/// * **Weeks:** `weeks`, `week`, `w`
/// * **Days:** `days`, `day`, `d`
/// * **Hours:** `hours`, `hour`, `hrs`, `hr`, `h`
/// * **Minutes:** `minutes`, `minute`, `mins`, `min`, `m`
/// * **Seconds:** `seconds`, `second`, `secs`, `sec`, `s`
/// * **Milliseconds:** `milliseconds`, `millisecond`, `msecs`, `msec`, `ms` and empty postfix
#[inline]
pub fn get_duration_by_postfix<'a, P>(milliseconds: i64, postfix: P) -> Result<String, Error>
where
    P: Into<Cow<'a, str>>,
{
    let postfix = &*postfix.into();
    let b_postfix = get_byte_postfix(postfix);
    let v = get_modification(b_postfix)?;
    Ok(format!(
        "{}{}",
        (milliseconds as f64 / v).round() as i64,
        postfix
    ))
}

/// Getting human-like time from milliseconds.
/// `get_max_possible_duration` function gets a milliseconds count and returns a max possible string with your time.
/// `get_max_possible_duration` **has some limitations** maximum of avalable postfixes is a day.
///
/// ### Usage
/// ```
/// use crate::ms_converter::{get_max_possible_duration, WEEK};
///
/// let value = get_max_possible_duration(2 * WEEK as i64).unwrap();
/// assert_eq!(value, "14d") // Max possible period is a day
/// ```
///
/// also you can a pass negative values
/// ```
/// use crate::ms_converter::{get_max_possible_duration, WEEK};
///
/// let value = get_max_possible_duration(-2 * WEEK as i64).unwrap();
/// assert_eq!(value, "-14d") // Max possible period is a day
/// ```
#[inline]
pub fn get_max_possible_duration(milliseconds: i64) -> Result<String, Error> {
    let postfix = match milliseconds.abs() {
        m if m >= DAY as i64 => "d",
        m if m >= HOUR as i64 => "h",
        m if m >= MINUTE as i64 => "m",
        m if m >= SECOND as i64 => "s",
        _ => "ms",
    };
    get_duration_by_postfix(milliseconds, postfix)
}

/// Getting human-like time from milliseconds.
/// `get_max_possible_duration_long` function gets a milliseconds count and returns a max possible string with your time.
/// `get_max_possible_duration_long` **has some limitations** maximum of avalable postfixes is a day.
///
/// ### Usage
/// ```
/// use crate::ms_converter::{get_max_possible_duration_long, WEEK};
///
/// let value = get_max_possible_duration_long(2 * WEEK as i64).unwrap();
/// assert_eq!(value, "14 days") // Max possible period is a day
/// ```
///
/// ```
/// use crate::ms_converter::{get_max_possible_duration_long, DAY};
///
/// let value = get_max_possible_duration_long(DAY as i64).unwrap();
/// assert_eq!(value, "1 day")
/// ```
///
/// also you can a pass negative values
/// ```
/// use crate::ms_converter::{get_max_possible_duration_long, WEEK};
///
/// let value = get_max_possible_duration_long(-2 * WEEK as i64).unwrap();
/// assert_eq!(value, "-14 days") // Max possible period is a day
/// ```
#[inline]
pub fn get_max_possible_duration_long(milliseconds: i64) -> Result<String, Error> {
    let postfix = match milliseconds.abs() {
        m if m >= DAY as i64 => check_postfix(m, DAY, " day", " days"),
        m if m >= HOUR as i64 => check_postfix(m, HOUR, " hour", " hours"),
        m if m >= MINUTE as i64 => check_postfix(m, MINUTE, " minute", " minutes"),
        m if m >= SECOND as i64 => check_postfix(m, SECOND, " second", " seconds"),
        m => check_postfix(m, 1f64, " millisecond", " milliseconds"),
    };
    get_duration_by_postfix(milliseconds, postfix)
}

#[inline(always)]
#[doc(hidden)]
fn check_postfix<'a>(
    milliseconds: i64,
    period: f64,
    postfix: &'a str,
    postfix_mul: &'a str,
) -> &'a str {
    if milliseconds as f64 >= 1.5 * period {
        return postfix_mul;
    }
    postfix
}

#[inline(always)]
#[doc(hidden)]
fn get_byte_postfix(postfix: &str) -> &[u8] {
    let b_postfix = postfix.as_bytes();
    match b_postfix.first() {
        Some(c) if c.is_ascii_whitespace() => &b_postfix[1..],
        _ => b_postfix,
    }
}

#[inline(always)]
#[doc(hidden)]
fn get_modification(postfix: &[u8]) -> Result<f64, Error> {
    match postfix.first() {
        Some(b'y') if matches!(postfix, b"years" | b"year" | b"yrs" | b"yr" | b"y") => Ok(YEAR),
        Some(b'w') if matches!(postfix, b"weeks" | b"week" | b"w") => Ok(WEEK),
        Some(b'd') if matches!(postfix, b"days" | b"day" | b"d") => Ok(DAY),
        Some(b'h') if matches!(postfix, b"hours" | b"hour" | b"hrs" | b"hr" | b"h") => Ok(HOUR),
        Some(b'm') if matches!(postfix, b"minutes" | b"minute" | b"mins" | b"min" | b"m") => {
            Ok(MINUTE)
        }
        None | Some(b'm')
            if matches!(
                postfix,
                b"milliseconds" | b"millisecond" | b"msecs" | b"msec" | b"ms" | b""
            ) =>
        {
            Ok(1f64)
        }
        Some(b's') if matches!(postfix, b"seconds" | b"second" | b"secs" | b"sec" | b"s") => {
            Ok(SECOND)
        }
        _ => Err(Error::new("invalid postfix")),
    }
}

#[inline(always)]
#[doc(hidden)]
fn parse(mut num: &[u8]) -> Result<f64, Error> {
    let sign = match num.first() {
        Some(b'-') => {
            num = &num[1..];
            -1_f64
        }
        Some(b'+') => {
            num = &num[1..];
            1_f64
        }
        _ => 1_f64,
    };
    let (mut ind, mut dist) = num
        .iter()
        .take_while(|b| b.is_ascii_digit())
        .map(|b| b.sub(b'0') as f64)
        .fold((0, 0_f64), |(ind, dist), b| {
            (ind.add(1), dist.mul_add(10_f64, b))
        });

    if matches!(num.get(ind), Some(b'.')) {
        ind = ind.add(1)
    }

    if ind < num.len() {
        let (pow, temp) = num[ind..]
            .iter()
            .take_while(|b| b.is_ascii_digit())
            .map(|b| b.sub(b'0') as f64)
            .fold((1, 0_f64), |(pow, temp), b| {
                (pow.add(1), temp.mul_add(10_f64, b))
            });

        ind = ind.add(pow as usize).sub(1);
        let pow = pow.mul(-1_i32).add(1);
        dist = dist.add(temp.mul((10_f64).powi(pow)));
    }

    if num.len() == ind {
        Ok(dist.copysign(sign))
    } else {
        Err(Error::new("invalid value"))
    }
}

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

/// Ms into time is the abstraction on `ms` function, which converts result into `time.Duration` type.
/// `ms_into_time` function gets an str slice or String and returns `time.Duration`.
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
pub fn ms_into_time<'a, T>(s: T) -> Result<Duration, Error>
where
    T: Into<Cow<'a, str>>,
{
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
