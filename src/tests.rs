use crate::ms;

#[test]
fn ms_1d() {
    let value = ms("1d").unwrap();
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
    let value = ms("100 xs").is_err();
    assert_eq!(value, true)
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
