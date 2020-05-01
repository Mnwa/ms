use crate::{from_ms, ms, ms_into_time, parse, DAY, HOUR, MINUTE, SECOND, WEEK, YEAR};
use std::string::ToString;

#[test]
fn ms_1d() {
    let value = ms("1d").unwrap();
    assert_eq!(value, 86_400_000)
}

#[test]
fn ms_1d_string() {
    let value = ms("1d".to_string()).unwrap();
    assert_eq!(value, 86_400_000)
}

#[test]
fn ms_2d() {
    let value = ms("2 days").unwrap();
    assert_eq!(value, 172_800_000)
}

#[test]
fn ms_2_5h() {
    let value = ms("2.5 hrs").unwrap();
    assert_eq!(value, 9_000_000)
}

#[test]
fn ms_pos_ms() {
    let value = ms("+100").unwrap();
    assert_eq!(value, 100)
}

#[test]
fn ms_neg_ms() {
    let value = ms("-100").unwrap();
    assert_eq!(value, -100)
}

#[test]
fn ms_invalid_postfix() {
    let value = ms("100 test").is_err();
    assert_eq!(value, true)
}

#[test]
fn ms_invalid_num() {
    let value = ms("test").is_err();
    assert_eq!(value, true)
}

#[test]
fn ms_invalid_num_2() {
    let value = ms("1-2").is_err();
    assert_eq!(value, true)
}

#[test]
fn ms_invalid_num_3() {
    let value = ms("1..2").is_err();
    assert_eq!(value, true)
}

#[test]
fn ms_empty_dot() {
    let value = ms("12.").unwrap();
    assert_eq!(value, 12)
}

#[test]
fn ms_macro_1d() {
    const VALUE: i64 = ms_expr!(i64, 1 d);
    assert_eq!(VALUE, 86_400_000)
}

#[test]
fn ms_macro_2d() {
    const VALUE: i64 = ms_expr!(i64, 2 days);
    assert_eq!(VALUE, 172_800_000)
}

#[test]
#[allow(clippy::float_cmp)]
fn ms_macro_2_5h() {
    const VALUE: f64 = ms_expr!(f64, 2.5 hrs);
    assert_eq!(VALUE, 9_000_000.)
}

#[test]
fn ms_macro_neg_ms() {
    const VALUE: i64 = ms_expr!(i64, -100);
    assert_eq!(VALUE, -100)
}

#[test]
fn ms_into_time_1d() {
    let value = ms_into_time("1d").unwrap();
    assert_eq!(value.as_millis(), 86_400_000)
}

#[test]
fn ms_into_time_2d() {
    let value = ms_into_time("2 days").unwrap();
    assert_eq!(value.as_millis(), 172_800_000)
}

#[test]
fn ms_into_time_2_5h() {
    let value = ms_into_time("2.5 hrs").unwrap();
    assert_eq!(value.as_millis(), 9_000_000)
}

#[test]
fn ms_into_time_neg_ms() {
    let value = ms_into_time("-100").is_err();
    assert_eq!(value, true)
}

#[test]
fn ms_into_time_invalid_postfix() {
    let value = ms_into_time("100 test").is_err();
    assert_eq!(value, true)
}

#[test]
fn parse_invalid_num() {
    let value = parse(b"test").is_err();
    assert_eq!(value, true)
}

#[test]
fn parse_invalid_num_2() {
    let value = parse(b"1-2").is_err();
    assert_eq!(value, true)
}

#[test]
fn parse_invalid_num_3() {
    let value = parse(b"1..2").is_err();
    assert_eq!(value, true)
}

#[test]
#[allow(clippy::float_cmp)]
fn parse_empty_dot() {
    let value = parse(b"12.").unwrap();
    assert_eq!(value, 12.)
}

#[test]
#[allow(clippy::float_cmp)]
fn parse_num() {
    let value = parse(b"12").unwrap();
    assert_eq!(value, 12.)
}

#[test]
#[allow(clippy::float_cmp)]
fn parse_dec_num() {
    let value = parse(b"12.5").unwrap();
    assert_eq!(value, 12.5)
}

#[test]
fn from_ms_base() {
    let value = from_ms(1, "ms").unwrap();
    assert_eq!(value, "1ms")
}

#[test]
fn from_ms_base_space() {
    let value = from_ms(1, " ms").unwrap();
    assert_eq!(value, "1 ms")
}

#[test]
fn from_ms_seconds() {
    let value = from_ms(10 * SECOND as i64, "seconds").unwrap();
    assert_eq!(value, "10seconds")
}

#[test]
fn from_ms_seconds_space() {
    let value = from_ms(10 * SECOND as i64, " seconds").unwrap();
    assert_eq!(value, "10 seconds")
}

#[test]
fn from_ms_minute() {
    let value = from_ms(MINUTE as i64, "minute").unwrap();
    assert_eq!(value, "1minute")
}

#[test]
fn from_ms_minute_space() {
    let value = from_ms(MINUTE as i64, " minute").unwrap();
    assert_eq!(value, "1 minute")
}

#[test]
fn from_ms_hours() {
    let value = from_ms(10 * HOUR as i64, "hours").unwrap();
    assert_eq!(value, "10hours")
}

#[test]
fn from_ms_hours_space() {
    let value = from_ms(10 * HOUR as i64, " hours").unwrap();
    assert_eq!(value, "10 hours")
}

#[test]
fn from_ms_day() {
    let value = from_ms(DAY as i64, "day").unwrap();
    assert_eq!(value, "1day")
}

#[test]
fn from_ms_day_space() {
    let value = from_ms(DAY as i64, " day").unwrap();
    assert_eq!(value, "1 day")
}

#[test]
fn from_ms_weeks() {
    let value = from_ms(10 * WEEK as i64, "weeks").unwrap();
    assert_eq!(value, "10weeks")
}

#[test]
fn from_ms_weeks_space() {
    let value = from_ms(10 * WEEK as i64, " weeks").unwrap();
    assert_eq!(value, "10 weeks")
}

#[test]
fn from_ms_year() {
    let value = from_ms(YEAR as i64, "year").unwrap();
    assert_eq!(value, "1year")
}

#[test]
fn from_ms_year_space() {
    let value = from_ms(YEAR as i64, " year").unwrap();
    assert_eq!(value, "1 year")
}
