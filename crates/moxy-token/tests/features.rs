#[cfg(feature = "proc-macro2")]
#[test]
fn proc_macro2_roundtrips_groups_punctuation_keywords_and_literals() {
    use std::str::FromStr;

    use moxy_token::TokenStream;

    for (source, expected) in [
        (
            "name 42u64 3.5f32 'x' b\"bytes\" c\"ffi\"",
            "name 42u64 3.5f32 'x' b\"bytes\" c\"ffi\"",
        ),
        (
            "pub fn f<T>(x: T) -> Option<T> { Some(x) }",
            "pub fn f < T > (x : T) -> Option < T > { Some (x) }",
        ),
        ("a::b::<C>() >>= 2", "a :: b :: < C > () >>= 2"),
        ("(alpha, [beta], { gamma })", "(alpha , [beta] , { gamma })"),
    ] {
        let proc_tokens = proc_macro2::TokenStream::from_str(source).unwrap();
        let owned = TokenStream::from(proc_tokens);
        let roundtrip = proc_macro2::TokenStream::from(owned);
        assert_eq!(roundtrip.to_string(), expected);
    }
}

#[cfg(feature = "proc-macro2")]
#[test]
#[ignore]
fn proc_macro2_boolean_bridge_preserves_boolean_values_and_output() {
    use std::str::FromStr;

    use moxy_token::{Lit, ToTokens, TokenStream, TokenTree};

    let owned = TokenStream::from(proc_macro2::TokenStream::from_str("true false").unwrap());
    assert!(matches!(&owned[0], TokenTree::Literal(Lit::Bool(value)) if value.value()));
    assert!(matches!(&owned[1], TokenTree::Literal(Lit::Bool(value)) if !value.value()));
    assert_eq!(proc_macro2::TokenStream::from(owned).to_string(), "true false");

    let mut output = proc_macro2::TokenStream::new();
    TokenTree::Literal(Lit::Bool(moxy_token::LitBool::new(true, Default::default()))).to_tokens(&mut output);
    assert_eq!(output.to_string(), "true");
}

#[cfg(feature = "serde")]
#[test]
fn serde_serializes_the_complete_public_token_model() {
    use std::str::FromStr;

    use moxy_token::{Delim, Group, Ident, Lit, TokenStream, TokenTree};

    let values = [
        serde_json::to_value(Ident::new("name")).unwrap(),
        serde_json::to_value(Lit::u16_suffixed(16)).unwrap(),
        serde_json::to_value(TokenTree::from(Lit::string("text"))).unwrap(),
        serde_json::to_value(Group::new(Delim::Paren, TokenStream::from_str("a + b").unwrap())).unwrap(),
        serde_json::to_value(TokenStream::from_str("pub fn f() {}").unwrap()).unwrap(),
    ];

    assert_eq!(values[0], serde_json::json!("name"));
    assert_eq!(values[1], serde_json::json!("16u16"));
    assert_eq!(values[2], serde_json::json!("\"text\""));
    assert_eq!(values[3]["delim"], serde_json::json!("paren"));
    assert!(values[4].as_array().unwrap().len() >= 4);
}
