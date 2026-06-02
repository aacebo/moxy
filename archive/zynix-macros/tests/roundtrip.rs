use zynix::TokenStream;

fn roundtrip(src: &str) -> String {
    let pm2: proc_macro2::TokenStream = src.parse().unwrap();
    let ours: TokenStream = pm2.into();
    let back: proc_macro2::TokenStream = ours.into();
    back.to_string()
}

fn relex(src: &str) {
    let once = roundtrip(src);
    assert!(
        once.parse::<proc_macro2::TokenStream>().is_ok(),
        "round-trip of {src:?} did not re-lex: {once:?}"
    );
}

#[test]
fn raw_idents() {
    relex("r#foo");
    relex("r#fn");
}

#[test]
fn keywords() {
    relex("struct S;");
    relex("impl S { pub fn make() -> Self { Self } }");
    relex("let x = 3; match x { 3 => x, _ => 0 }");
}

#[test]
fn lifetimes() {
    relex("&'a [i32]");
    relex("fn takes<'a>(x: &'a str) {}");
    relex("'static");
}

#[test]
fn literals() {
    relex("1u8");
    relex("1.5f64");
    relex("0xffu16");
    relex("0b1010");
    relex("1_000usize");
    relex("b'x'");
    relex("'c'");
    relex("\"s\"");
    relex("b\"x\"");
    relex("c\"x\"");
    relex("r#\"raw\"#");
}

#[test]
fn groups_and_multitoken() {
    relex("(1 + (2 * 3))");
    relex("vec![1, 2, 3]");
    relex("1 == 1 && 2 <= 3");
}
