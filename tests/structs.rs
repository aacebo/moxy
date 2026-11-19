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

#[cfg(feature = "proc-macro2")]
#[test]
fn raw_identifiers_bridge_back_to_proc_macro_raw_identifiers() {
    let owned: moxy::token::TokenStream = "struct Bridged { r#type: u32 }".parse().unwrap();
    let bridged = proc_macro2::TokenStream::from(owned);
    let rendered = bridged.to_string();
    assert!(rendered.contains("r#type"), "{rendered}");
    assert_eq!(
        moxy::token::TokenStream::from(bridged).to_string(),
        "struct Bridged {r#type : u32}"
    );
}

#[test]
fn tuple_struct_where_clauses_follow_the_fields() {
    let item: Item = moxy::parse!("struct S<T>(T) where T: Clone;").unwrap();
    assert!(item.as_struct().unwrap().generics.where_clause.is_some());
    assert_eq!(item.to_token_stream().to_string(), "struct S < T > (T) where T : Clone ;");
    assert_eq!(moxy::fmt!(&item).unwrap(), "struct S<T>(T)\nwhere\nT: Clone;");

    let item: Item = moxy::parse!("struct S<T> where T: Clone { a: T }").unwrap();
    assert_eq!(item.to_token_stream().to_string(), "struct S < T > where T : Clone {a : T}");
}

#[test]
fn parentheses_after_pub_that_do_not_restrict_it_are_the_field_type() {
    for (source, restricted) in [
        ("struct S(pub (u8, u8));", false),
        ("struct S(pub (crate::A));", false),
        ("struct S(pub ());", false),
        ("struct S(pub (crate) u8);", true),
        ("struct S(pub (in a) u8);", true),
        ("struct S(pub (super) u8);", true),
    ] {
        let item: moxy::ast::Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(item.to_token_stream().to_string(), lexed.to_string());
        let moxy::ast::Fields::Unnamed(fields) = &item.as_struct().unwrap().fields else {
            panic!("`{source}` has no tuple fields");
        };
        let field = fields.fields.inner.first().unwrap();
        assert_eq!(
            !matches!(field.vis, moxy::ast::Visibility::Public { .. }),
            restricted,
            "`{source}`"
        );
    }
}

#[test]
fn a_field_takes_no_mut() {
    assert!(moxy::parse!("struct S { pub a: u8 }" as Item).is_ok());

    for source in ["struct S { mut a: u8 }", "struct S(mut u8);"] {
        assert!(moxy::parse!(source as Item).is_err(), "`{source}`");
    }
}

#[test]
fn a_restricted_visibility_holds_one_path() {
    let item: Item = moxy::parse!("struct S(pub(in a::b) u8);").unwrap();
    assert_eq!(item.to_token_stream().to_string(), "struct S (pub (in a :: b) u8) ;");
    assert!(moxy::parse!("struct S(pub (in a b) u8);" as Item).is_err());
}
