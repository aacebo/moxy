#[derive(moxy::Meta)]
struct Inner {
    value: bool,
}

#[derive(moxy::Meta)]
struct Args {
    #[meta(rename = "enabled")]
    flag: bool,
    #[meta(default)]
    retry: bool,
    #[meta(default = true)]
    fallback: bool,
    inner: Inner,
}

fn main() {
    let meta: moxy::ast::Meta = moxy::parse!("build(enabled = true, inner(value = false))").unwrap();
    let args: Args = meta.parse().unwrap();
    assert!(args.flag);
    assert!(!args.retry);
    assert!(args.fallback);
    assert!(!args.inner.value);

    let missing: moxy::ast::Meta = moxy::parse!("build(enabled = true)").unwrap();
    assert!(missing.parse::<Args>().is_err());

    let duplicate: moxy::ast::Meta = moxy::parse!("build(enabled = true, enabled = false, inner(value = false))").unwrap();
    assert!(duplicate.parse::<Args>().is_err());

    let unknown: moxy::ast::Meta = moxy::parse!("build(enabled = true, inner(value = false), extra = true)").unwrap();
    assert!(unknown.parse::<Args>().is_err());
}
