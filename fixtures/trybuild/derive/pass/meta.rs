#[derive(moxy::FromMeta, Debug, PartialEq, Eq)]
struct Inner {
    value: bool,
}

#[derive(moxy::FromMeta)]
struct Args {
    #[meta(rename = "enabled")]
    flag: bool,
    #[meta(default)]
    retry: bool,
    #[meta(default = true)]
    fallback: bool,
    inner: Inner,
}

#[derive(Debug, PartialEq, Eq)]
struct Marker<'a>(std::marker::PhantomData<&'a ()>);

impl<'a> moxy::ast::FromMeta for Marker<'a> {
    fn from_meta(meta: &moxy::ast::Meta) -> Result<Self, moxy::ast::ParseError> {
        let _ = <bool as moxy::ast::FromMeta>::from_meta(meta)?;
        Ok(Self(std::marker::PhantomData))
    }
}

#[derive(moxy::FromMeta, Debug, PartialEq, Eq)]
struct Generic<'a, T: Clone = bool, const N: usize = 1>
where
    T: 'a,
{
    value: T,
    marker: Marker<'a>,
}

#[derive(moxy::FromMeta, Debug, PartialEq, Eq)]
enum GenericEnum<'a, T: Clone = bool, const N: usize = 1>
where
    T: 'a,
{
    Value {
        value: T,
        marker: Marker<'a>,
    },
}

#[derive(moxy::FromMeta, Debug)]
struct Messages {
    #[meta(rename = "named", message = "field {ident} at {path}")]
    value: bool,
}

#[derive(moxy::FromMeta, Debug)]
enum MessageVolume {
    #[meta(rename = "custom", message = "variant {ident} at {path}")]
    Custom(bool),
    Named {
        #[meta(message = "nested {ident} at {path}")]
        value: bool,
    },
}

#[derive(moxy::FromMeta, Debug, PartialEq, Eq)]
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

    let generic_meta: moxy::ast::Meta = moxy::parse!("generic(value = true, marker = true)").unwrap();
    let generic: Generic<'static, bool, 1> = generic_meta.parse().unwrap();
    assert_eq!(generic, Generic { value: true, marker: Marker(std::marker::PhantomData) });

    let generic_enum_meta: moxy::ast::Meta = moxy::parse!("generic_enum(value(value = true, marker = true))").unwrap();
    let generic_enum: GenericEnum<'static, bool, 1> = generic_enum_meta.parse().unwrap();
    assert_eq!(generic_enum, GenericEnum::Value { value: true, marker: Marker(std::marker::PhantomData) });

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
    assert_eq!(missing_value.parse::<Volume>().unwrap(), Volume::Decibels(true));

    let missing: moxy::ast::Meta = moxy::parse!("message()").unwrap();
    assert_eq!(missing.parse::<Messages>().unwrap_err().message(), "field named at named");

    let duplicate: moxy::ast::Meta = moxy::parse!("message(named = true, named = false)").unwrap();
    assert_eq!(duplicate.parse::<Messages>().unwrap_err().message(), "field named at named");

    let unit: moxy::ast::Meta = moxy::parse!("message_volume(custom)").unwrap();
    assert!(matches!(unit.parse::<MessageVolume>().unwrap(), MessageVolume::Custom(true)));

    let named: moxy::ast::Meta = moxy::parse!("message_volume(named())").unwrap();
    assert_eq!(named.parse::<MessageVolume>().unwrap_err().message(), "nested value at value");
}
