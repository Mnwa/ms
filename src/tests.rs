use crate::{ms, ms_into_time, parse};

#[test]
fn ms_1d() {
    let value = ms("1d").unwrap();
    assert_eq!(value, 86400000)
}

#[test]
fn ms_1d_string() {
    let value = ms("1d".to_string()).unwrap();
    assert_eq!(value, 86400000)
}

#[test]
fn ms_2d() {
    let value = ms("2 days").unwrap();
    assert_eq!(value, 172800000)
}

#[test]
fn ms_2_5h() {
    let value = ms("2.5 hrs").unwrap();
    assert_eq!(value, 9000000)
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
    assert_eq!(VALUE, 86400000)
}

#[test]
fn ms_macro_2d() {
    const VALUE: i64 = ms_expr!(i64, 2 days);
    assert_eq!(VALUE, 172800000)
}

#[test]
fn ms_macro_2_5h() {
    const VALUE: f64 = ms_expr!(f64, 2.5 hrs);
    assert_eq!(VALUE, 9000000.)
}

#[test]
fn ms_macro_neg_ms() {
    const VALUE: i64 = ms_expr!(i64, -100);
    assert_eq!(VALUE, -100)
}

#[test]
fn ms_into_time_1d() {
    let value = ms_into_time("1d").unwrap();
    assert_eq!(value.as_millis(), 86400000)
}

#[test]
fn ms_into_time_2d() {
    let value = ms_into_time("2 days").unwrap();
    assert_eq!(value.as_millis(), 172800000)
}

#[test]
fn ms_into_time_2_5h() {
    let value = ms_into_time("2.5 hrs").unwrap();
    assert_eq!(value.as_millis(), 9000000)
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
    let value = parse("test").is_err();
    assert_eq!(value, true)
}

#[test]
fn parse_invalid_num_2() {
    let value = parse("1-2").is_err();
    assert_eq!(value, true)
}

#[test]
fn parse_invalid_num_3() {
    let value = parse("1..2").is_err();
    assert_eq!(value, true)
}

#[test]
fn parse_empty_dot() {
    let value = parse("12.").unwrap() as i64;
    assert_eq!(value, 12)
}

#[test]
fn parse_num() {
    let value = parse("12").unwrap() as i64;
    assert_eq!(value, 12)
}

#[test]
fn parse_dec_num() {
    let value = parse("12.5").unwrap().round() as i64;
    assert_eq!(value, 13)
}
