use moxy::ast::Item;
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
