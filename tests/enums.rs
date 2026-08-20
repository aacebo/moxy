use moxy::ast::{Fields, Item};

#[test]
fn enum_variants_preserve_unit_tuple_named_and_discriminant_syntax() {
    let item: Item = moxy::parse!("pub enum Message<T>{Unit,Tuple(T),Named{value:T},Explicit=4}").unwrap();
    let enumeration = item.as_enum().unwrap();
    assert_eq!(enumeration.ident.text(), "Message");
    assert_eq!(enumeration.variants.inner.len(), 4);
    assert!(enumeration.variants.inner[0].fields.is_unit());
    assert!(matches!(enumeration.variants.inner[1].fields, Fields::Unnamed(_)));
    assert!(matches!(enumeration.variants.inner[2].fields, Fields::Named(_)));
    assert!(enumeration.variants.inner[3].discriminant.is_some());
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "pub enum Message<T> {\n\tUnit,\n\tTuple(T),\n\tNamed {\n\t\tvalue: T,\n\t},\n\tExplicit = 4,\n}"
    );
}

#[test]
fn compact_enum_syntax_renders_one_variant_per_line() {
    let item: Item = moxy::parse!("enum Color{Red,Green,Blue}").unwrap();
    let enumeration = item.as_enum().unwrap();
    assert_eq!(enumeration.variants.inner.len(), 3);
    assert!(enumeration.variants.inner.iter().all(|variant| variant.fields.is_unit()));
    assert_eq!(moxy::fmt!(&item).unwrap(), "enum Color {\n\tRed,\n\tGreen,\n\tBlue,\n}");
}
