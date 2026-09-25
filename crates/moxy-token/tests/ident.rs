use moxy_token::ident;

#[test]
fn from_ident() {
    let ident = ident!(abc);
    assert_eq!(ident, "abc");
}

#[test]
fn from_literal_str() {
    let ident = ident!("a_test_name");
    assert_eq!(ident, "a_test_name");
}

#[test]
fn from_expr() {
    let is_one = false;
    let ident = ident!(if is_one { "one" } else { "two" });
    assert_eq!(ident, "two");
}

#[test]
fn from_raw() {
    let ident = ident!(r#dyn);
    assert!(ident.is_raw());
    assert_eq!(ident, "r#dyn");
}

#[test]
fn from_concat() {
    let ident = ident!(this, "_", "is", "_", a, "_", test);
    assert_eq!(ident, "this_is_a_test");
}
