use crate::{
    get_duration_by_postfix, get_max_possible_duration, ms, ms_into_time, parse, DAY, HOUR, MINUTE,
    SECOND, WEEK, YEAR,
};
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
fn get_duration_by_postfix_base() {
    let value = get_duration_by_postfix(1, "ms").unwrap();
    assert_eq!(value, "1ms")
}

#[test]
fn get_duration_by_postfix_base_space() {
    let value = get_duration_by_postfix(1, " ms").unwrap();
    assert_eq!(value, "1 ms")
}

#[test]
fn get_duration_by_postfix_seconds() {
    let value = get_duration_by_postfix(10 * SECOND as i64, "seconds").unwrap();
    assert_eq!(value, "10seconds")
}

#[test]
fn get_duration_by_postfix_seconds_space() {
    let value = get_duration_by_postfix(10 * SECOND as i64, " seconds").unwrap();
    assert_eq!(value, "10 seconds")
}

#[test]
fn get_duration_by_postfix_minute() {
    let value = get_duration_by_postfix(MINUTE as i64, "minute").unwrap();
    assert_eq!(value, "1minute")
}

#[test]
fn get_duration_by_postfix_minute_space() {
    let value = get_duration_by_postfix(MINUTE as i64, " minute").unwrap();
    assert_eq!(value, "1 minute")
}

#[test]
fn get_duration_by_postfix_hours() {
    let value = get_duration_by_postfix(10 * HOUR as i64, "hours").unwrap();
    assert_eq!(value, "10hours")
}

#[test]
fn get_duration_by_postfix_hours_space() {
    let value = get_duration_by_postfix(10 * HOUR as i64, " hours").unwrap();
    assert_eq!(value, "10 hours")
}

#[test]
fn get_duration_by_postfix_day() {
    let value = get_duration_by_postfix(DAY as i64, "day").unwrap();
    assert_eq!(value, "1day")
}

#[test]
fn get_duration_by_postfix_day_space() {
    let value = get_duration_by_postfix(DAY as i64, " day").unwrap();
    assert_eq!(value, "1 day")
}

#[test]
fn get_duration_by_postfix_weeks() {
    let value = get_duration_by_postfix(10 * WEEK as i64, "weeks").unwrap();
    assert_eq!(value, "10weeks")
}

#[test]
fn get_duration_by_postfix_weeks_space() {
    let value = get_duration_by_postfix(10 * WEEK as i64, " weeks").unwrap();
    assert_eq!(value, "10 weeks")
}

#[test]
fn get_duration_by_postfix_year() {
    let value = get_duration_by_postfix(YEAR as i64, "year").unwrap();
    assert_eq!(value, "1year")
}

#[test]
fn get_duration_by_postfix_year_space() {
    let value = get_duration_by_postfix(YEAR as i64, " year").unwrap();
    assert_eq!(value, "1 year")
}

#[test]
fn get_duration_by_postfix_year_neg() {
    let value = get_duration_by_postfix(-YEAR as i64, "year").unwrap();
    assert_eq!(value, "-1year")
}

#[test]
fn get_duration_by_postfix_year_neg_space() {
    let value = get_duration_by_postfix(-YEAR as i64, " year").unwrap();
    assert_eq!(value, "-1 year")
}

#[test]
fn get_max_possible_duration_milliseconds() {
    let value = get_max_possible_duration(11 as i64).unwrap();
    assert_eq!(value, "11ms")
}

#[test]
fn get_max_possible_duration_seconds() {
    let value = get_max_possible_duration(12 * SECOND as i64).unwrap();
    assert_eq!(value, "12s")
}

#[test]
fn get_max_possible_duration_minutes() {
    let value = get_max_possible_duration(32 * MINUTE as i64).unwrap();
    assert_eq!(value, "32m")
}

#[test]
fn get_max_possible_duration_hours() {
    let value = get_max_possible_duration(10 * HOUR as i64).unwrap();
    assert_eq!(value, "10h")
}

#[test]
fn get_max_possible_duration_day() {
    let value = get_max_possible_duration(DAY as i64).unwrap();
    assert_eq!(value, "1d")
}

#[test]
fn get_max_possible_duration_weeks() {
    let value = get_max_possible_duration(WEEK as i64).unwrap();
    assert_eq!(value, "7d")
}
#[test]
fn get_max_possible_duration_two_weeks() {
    let value = get_max_possible_duration(2 * WEEK as i64).unwrap();
    assert_eq!(value, "14d")
}

#[test]
fn get_max_possible_duration_weeks_neg() {
    let value = get_max_possible_duration(-WEEK as i64).unwrap();
    assert_eq!(value, "-7d")
}
