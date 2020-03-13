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
