use moxy::ast::{Item, Meta};
use moxy::token::Spanner;

#[test]
fn outer_attributes_are_attached_to_the_declaration_and_rendered() {
    let item: Item = moxy::parse!("#[repr(C)] #[derive(Clone, Debug)] pub struct Header { value: u32 }").unwrap();
    let structure = item.as_struct().unwrap();
    assert_eq!(structure.attrs.len(), 2);
    assert_eq!(structure.ident.text(), "Header");
    assert!(!structure.span().is_empty());
    debug_assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "#[repr(C)]\n#[derive(Clone, Debug)]\npub struct Header {\n\tvalue: u32,\n}",
        "{item:#?}"
    );
}

#[test]
fn field_attributes_remain_with_the_field_syntax() {
    let item: Item = moxy::parse!("struct Packet { #[cfg(unix)] bytes: Vec<u8> }").unwrap();
    let structure = item.as_struct().unwrap();
    let fields = &structure.fields.as_named().unwrap().fields.inner;
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].attrs.len(), 1);
    assert_eq!(fields[0].ident.as_ref().unwrap().text(), "bytes");
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "struct Packet {\n\t#[cfg(unix)]\n\tbytes: Vec<u8>,\n}"
    );
}

#[test]
fn metadata_converts_builtin_scalar_values() {
    let value: Meta = moxy::parse!("value").unwrap();
    assert!(value.parse::<bool>().unwrap());

    let value: Meta = moxy::parse!("value = false").unwrap();
    assert!(!value.parse::<bool>().unwrap());

    let value: Meta = moxy::parse!("value = \"text\"").unwrap();
    assert_eq!(value.parse::<String>().unwrap(), "text");

    let value: Meta = moxy::parse!("value = 'x'").unwrap();
    assert_eq!(value.parse::<char>().unwrap(), 'x');

    let value: Meta = moxy::parse!("value = 1").unwrap();
    assert_eq!(value.parse::<u8>().unwrap(), 1);
    assert_eq!(value.parse::<u16>().unwrap(), 1);
    assert_eq!(value.parse::<u32>().unwrap(), 1);
    assert_eq!(value.parse::<u64>().unwrap(), 1);
    assert_eq!(value.parse::<u128>().unwrap(), 1);
    assert_eq!(value.parse::<usize>().unwrap(), 1);
    assert_eq!(value.parse::<i8>().unwrap(), 1);
    assert_eq!(value.parse::<i16>().unwrap(), 1);
    assert_eq!(value.parse::<i32>().unwrap(), 1);
    assert_eq!(value.parse::<i64>().unwrap(), 1);
    assert_eq!(value.parse::<i128>().unwrap(), 1);
    assert_eq!(value.parse::<isize>().unwrap(), 1);

    let value: Meta = moxy::parse!("value = -1").unwrap();
    assert_eq!(value.parse::<i8>().unwrap(), -1);
    assert_eq!(value.parse::<i16>().unwrap(), -1);
    assert_eq!(value.parse::<i32>().unwrap(), -1);
    assert_eq!(value.parse::<i64>().unwrap(), -1);
    assert_eq!(value.parse::<i128>().unwrap(), -1);
    assert_eq!(value.parse::<isize>().unwrap(), -1);

    let value: Meta = moxy::parse!("value = 1.5").unwrap();
    assert_eq!(value.parse::<f32>().unwrap(), 1.5);
    assert_eq!(value.parse::<f64>().unwrap(), 1.5);

    let value: Meta = moxy::parse!("value = -1.5").unwrap();
    assert_eq!(value.parse::<f32>().unwrap(), -1.5);
    assert_eq!(value.parse::<f64>().unwrap(), -1.5);
}

#[test]
fn metadata_rejects_scalar_overflow_and_invalid_literal_shapes() {
    let value: Meta = moxy::parse!("value = 256").unwrap();
    assert!(value.parse::<u8>().is_err());

    let value: Meta = moxy::parse!("value = -129").unwrap();
    assert!(value.parse::<i8>().is_err());

    let value: Meta = moxy::parse!("value = -1").unwrap();
    assert!(value.parse::<u8>().is_err());

    let value: Meta = moxy::parse!("value = 1").unwrap();
    assert!(value.parse::<String>().is_err());
    assert!(value.parse::<char>().is_err());
    assert!(value.parse::<f64>().is_err());

    let value: Meta = moxy::parse!("value(1)").unwrap();
    assert!(value.parse::<u8>().is_err());

    let value: Meta = moxy::parse!("value = 1 + 2").unwrap();
    assert!(value.parse::<u8>().is_err());

    let value: Meta = moxy::parse!("value = true && false").unwrap();
    assert!(value.parse::<bool>().is_err());
}
