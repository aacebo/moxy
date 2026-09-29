#[derive(moxy::Meta, Debug, PartialEq, Eq)]
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

#[derive(moxy::Meta, Debug, PartialEq, Eq)]
enum Volume {
    Low,
    High,
    #[meta(rename = "dB")]
    Decibels(bool),
    Nested(Inner),
    Custom {
        #[meta(default)]
        muted: bool,
    },
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

    let high: moxy::ast::Meta = moxy::parse!("volume(high)").unwrap();
    assert_eq!(high.parse::<Volume>().unwrap(), Volume::High);

    let decibels: moxy::ast::Meta = moxy::parse!("volume(dB = true)").unwrap();
    assert_eq!(decibels.parse::<Volume>().unwrap(), Volume::Decibels(true));

    let nested: moxy::ast::Meta = moxy::parse!("volume(nested(value = false))").unwrap();
    assert_eq!(nested.parse::<Volume>().unwrap(), Volume::Nested(Inner { value: false }));

    let custom: moxy::ast::Meta = moxy::parse!("volume(custom())").unwrap();
    assert_eq!(custom.parse::<Volume>().unwrap(), Volume::Custom { muted: false });

    let multiple: moxy::ast::Meta = moxy::parse!("volume(low, high)").unwrap();
    assert!(multiple.parse::<Volume>().is_err());

    let unknown_variant: moxy::ast::Meta = moxy::parse!("volume(other)").unwrap();
    assert!(unknown_variant.parse::<Volume>().is_err());

    let unit_value: moxy::ast::Meta = moxy::parse!("volume(high = true)").unwrap();
    assert!(unit_value.parse::<Volume>().is_err());

    let missing_value: moxy::ast::Meta = moxy::parse!("volume(dB)").unwrap();
    assert!(missing_value.parse::<Volume>().is_err());
}
