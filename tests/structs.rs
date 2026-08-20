use moxy::ast::Item;
use moxy::token::ToTokenStream;

#[test]
fn named_struct_fields_are_parsed_and_rendered() {
    let item: Item = moxy::parse!("pub struct Point{x:f64,y:f64}").unwrap();
    let structure = item.as_struct().unwrap();
    assert_eq!(structure.ident.text(), "Point");
    assert!(structure.vis.is_public());
    assert_eq!(structure.fields.as_named().unwrap().fields.inner.len(), 2);
    assert!(!item.to_token_stream().is_empty());
    assert_eq!(moxy::fmt!(&item).unwrap(), "pub struct Point {\n\tx: f64,\n\ty: f64,\n}");
}

#[test]
fn tuple_and_unit_structs_preserve_their_shapes() {
    for (source, expected, expected_len) in [
        ("pub struct Pair(pub String,usize);", "pub struct Pair(pub String, usize);", 2),
        ("pub struct Marker;", "pub struct Marker;", 0),
    ] {
        let item: Item = moxy::parse!(source).unwrap();
        let structure = item.as_struct().unwrap();
        let actual_len = structure.fields.as_unnamed().map_or(0, |fields| fields.fields.inner.len());
        assert_eq!(actual_len, expected_len);
        assert!(structure.vis.is_public());
        assert!(!item.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&item).unwrap(), expected);
    }
}

#[test]
fn generic_structs_expose_lifetimes_bounds_and_visibility() {
    let item: Item = moxy::parse!("pub struct Request<'a,T:Clone>{pub url:&'a str,body:T,status:u32}").unwrap();
    let structure = item.as_struct().unwrap();
    assert_eq!(structure.ident.text(), "Request");
    assert_eq!(structure.generics.params.len(), 2);
    assert_eq!(structure.fields.as_named().unwrap().fields.inner.len(), 3);
    assert!(!item.to_token_stream().is_empty());
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "pub struct Request<'a, T: Clone> {\n\tpub url: &'a str,\n\tbody: T,\n\tstatus: u32,\n}"
    );
}

#[cfg(feature = "serde")]
#[test]
fn struct_syntax_has_concrete_serde_output_and_exact_rust_output() {
    let item: Item = moxy::parse!("pub struct Record<T> { value: T }").unwrap();
    let structure = item.as_struct().unwrap();
    assert_eq!(structure.ident.text(), "Record");
    assert_eq!(structure.generics.params.len(), 1);
    let serialized = serde_json::to_value(&item).unwrap();
    assert!(serialized.get("Struct").is_some(), "missing Struct tag in {serialized}");
    assert_eq!(moxy::fmt!(&item).unwrap(), "pub struct Record<T> {\n\tvalue: T,\n}");
}

#[cfg(feature = "proc-macro2")]
#[test]
fn proc_macro_tokens_complete_a_struct_syntax_pipeline() {
    use std::str::FromStr;

    let proc_tokens = proc_macro2::TokenStream::from_str("struct Bridged { value: u32 }").unwrap();
    let owned = moxy::token::TokenStream::from(proc_tokens);
    let item: Item = moxy::parse!(owned).unwrap();
    let structure = item.as_struct().unwrap();
    assert_eq!(structure.ident.text(), "Bridged");
    assert_eq!(structure.fields.as_named().unwrap().fields.inner.len(), 1);
    assert_eq!(moxy::fmt!(&item).unwrap(), "struct Bridged {\n\tvalue: u32,\n}");
}
