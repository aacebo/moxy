use std::str::FromStr;

use moxy_token::{Span, TokenStream, TokenTree};

#[test]
fn proc_macro2_tokens_convert_outside_a_procedural_macro() {
    let source = "alpha + true (\"x\")";
    let bridged = TokenStream::from(proc_macro2::TokenStream::from_str(source).unwrap());
    let tokens = bridged.into_inner();

    assert_eq!(tokens.len(), 4);
    assert!(matches!(&tokens[0], TokenTree::Ident(_)));
    assert!(matches!(&tokens[1], TokenTree::Punct(_)));
    assert!(matches!(&tokens[2], TokenTree::Literal(_)));
    assert!(matches!(&tokens[3], TokenTree::Group(_)));

    for (token, expected) in tokens.iter().zip([0..5, 6..7, 8..12, 13..18]) {
        assert!(matches!(token.span(), Span::Fallback(_)));
        assert_eq!(token.span().byte_range(), expected);
    }

    let group = tokens[3].as_group().unwrap();
    assert_eq!(group.span.open().byte_range(), 13..14);
    assert_eq!(group.span.close().byte_range(), 17..18);
    assert_eq!(group.tokens[0].span().byte_range(), 14..17);
}

#[test]
fn proc_macro2_round_trip_preserves_token_text() {
    let source = "alpha + true (\"x\")";
    let owned = TokenStream::from(proc_macro2::TokenStream::from_str(source).unwrap());
    let bridged = proc_macro2::TokenStream::from(owned);

    assert_eq!(bridged.to_string(), source);
}
