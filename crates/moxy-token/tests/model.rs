use std::str::FromStr;

use moxy_token::parser::ParseStream;
use moxy_token::punct::{Comma, Gt};
use moxy_token::span::RangeSpan;
use moxy_token::{Delim, Group, Ident, Lit, Punctuation, Spacing, Span, Spanner, ToTokenStream, TokenStream, TokenTree};

#[test]
fn delimiters_spacing_and_token_tree_accessors_cover_all_variants() {
    let delimiters = [
        (Delim::None, ' ', ' ', "none"),
        (Delim::Paren, '(', ')', "paren"),
        (Delim::Brace, '{', '}', "brace"),
        (Delim::Bracket, '[', ']', "bracket"),
    ];

    for (delim, open, close, name) in delimiters {
        assert_eq!(delim.open(), open);
        assert_eq!(delim.close(), close);
        assert_eq!(delim.as_str(), name);
        assert_eq!(Delim::from_char(open), if delim.is_none() { None } else { Some(delim) });
        assert_eq!(Delim::from_char(close), if delim.is_none() { None } else { Some(delim) });
    }
    assert!(Delim::None.is_none());
    assert!(Delim::Paren.is_paren());
    assert!(Delim::Brace.is_brace());
    assert!(Delim::Bracket.is_bracket());
    assert_eq!(Delim::from_open('('), Some(Delim::Paren));
    assert_eq!(Delim::from_close('}'), Some(Delim::Brace));
    assert_eq!(Delim::from_open('x'), None);
    assert_eq!(Delim::from_close('x'), None);

    assert!(Spacing::Alone.is_alone());
    assert!(Spacing::Joint.is_joint());
    assert_eq!(Spacing::Alone.to_string(), "alone");
    assert_eq!(Spacing::Joint.to_string(), "joint");

    let ident = TokenTree::from(Ident::new("name"));
    assert!(ident.is_ident());
    assert!(ident.is_token());
    assert_eq!(ident.as_ident().unwrap().text(), "name");
    assert_eq!(ident.text(), Some("name"));
    assert_eq!(ident.delim(), None);
    assert!(ident.as_group().is_none());
    assert!(ident.as_literal().is_none());

    let literal = TokenTree::from(Lit::u32_unsuffixed(7));
    assert!(literal.is_literal());
    assert_eq!(literal.as_literal().unwrap().as_u64(), Some(7));
    assert!(literal.as_ident().is_none());

    let group = TokenTree::from(Group::new(Delim::Bracket, TokenStream::from_str("a, b").unwrap()));
    assert!(group.is_group());
    assert!(!group.is_token());
    assert_eq!(group.delim(), Some(Delim::Bracket));
    assert_eq!(group.as_group().unwrap().stream().to_string(), "a , b");
}

#[test]
fn token_stream_collection_conversion_and_iteration_are_consistent() {
    let original = TokenStream::from_str("alpha + beta").unwrap();
    assert_eq!(original.len(), 3);
    assert!(!original.is_empty());
    assert_eq!(original.delim().span(), original.span());
    assert_eq!(original.clone().to_vec().len(), 3);
    assert_eq!(original.clone().into_inner().len(), 3);
    assert_eq!(original.clone().into_iter().count(), 3);
    assert_eq!(original.iter().count(), 3);

    let collected: TokenStream = original.clone().into_iter().collect();
    assert_eq!(collected, original);
    let borrowed = TokenStream::from(original.as_ref());
    assert_eq!(borrowed, original);
    let vector = Vec::<TokenTree>::from(original.clone());
    assert_eq!(TokenStream::from(vector), original);

    let mut extended = TokenStream::new();
    extended.extend_one(Ident::new("alpha").into());
    extended.extend([Punctuation::Plus(Default::default()).into(), Ident::new("beta").into()]);
    assert_eq!(extended.to_string(), "alpha + beta");
    assert_eq!(extended.span(), Spanner::span(&extended));
}

#[test]
fn parse_stream_navigation_and_group_parsing_cover_success_and_failure() {
    let tokens = TokenStream::from_str("a, b, [c, d] >> tail").unwrap();
    let mut stream = ParseStream::new(&tokens);
    assert_eq!(stream.remaining(), 7);
    assert_eq!(stream.curr().unwrap().text(), Some("a"));
    assert_eq!(stream.nth(2).unwrap().text(), Some("b"));
    let mut fork = stream.fork();
    assert_eq!(fork.parse::<Ident>().unwrap().text(), "a");
    assert!(fork.peek::<Comma>());
    fork.skip_if::<Comma>();
    assert_eq!(fork.parse::<Ident>().unwrap().text(), "b");
    stream.seek(&fork);
    assert_eq!(stream.prev().unwrap().text(), Some("b"));
    stream.skip_if::<Comma>();
    let inner = stream.parse_group(Delim::Bracket).unwrap();
    assert_eq!(inner.to_string(), "c , d");
    assert!(stream.parse_group(Delim::Paren).is_err());
    assert!(stream.peek::<Gt>());
    assert!(stream.eat_punct_head(">").is_some());
    assert!(stream.peek::<Gt>());
    assert!(stream.eat_punct_head(">").is_some());
    assert_eq!(stream.parse::<Ident>().unwrap().text(), "tail");
    assert!(stream.is_empty());

    let identifiers = TokenStream::from_str("a b c, d").unwrap();
    let mut stream = identifiers.parse();
    assert_eq!(stream.parse_while::<Ident>().len(), 3);
    stream.skip_until(|token| matches!(token, Some(TokenTree::Ident(_))));
    assert_eq!(stream.parse_until_empty::<Ident>().unwrap().len(), 1);
    assert!(stream.advance_by(1).is_none());
}

#[test]
fn identifiers_groups_and_ranges_preserve_public_contracts() {
    let mut ident = Ident::new("MiXeD");
    assert_eq!(ident.clone().to_lowercase().text(), "mixed");
    assert_eq!(ident.clone().to_uppercase().text(), "MIXED");
    assert_eq!(ident.clone().to_token_tree().text(), Some("MiXeD"));
    assert_eq!(ident.clone().into_token_tree().text(), Some("MiXeD"));
    ident.set_span(Span::mixed_site());
    assert_eq!(ident.span(), Span::mixed_site());

    let mut group = Group::new(Delim::Paren, "inside".to_token_stream());
    assert_eq!(group.delim(), Delim::Paren);
    assert_eq!(group.stream().to_string(), "inside");
    assert!(group.to_token_tree().is_group());
    assert!(group.clone().into_token_tree().is_group());
    group.set_span(Default::default());
    assert_eq!(group.span(), Default::default());

    let range = RangeSpan::new(Span::call_site(), Span::mixed_site());
    assert_eq!(range.start(), Span::call_site());
    assert_eq!(range.end(), Span::mixed_site());
    assert_eq!(range.span(), Span::call_site().join(Span::mixed_site()));
}

#[test]
fn tokenization_handles_comments_docs_operators_and_errors() {
    let tokens = TokenStream::from_str("/// docs\npub fn f<T>() where T: Clone { a >>= 1; b::c(); }").unwrap();
    let rendered = tokens.to_string();
    assert!(rendered.contains("doc"));
    assert!(rendered.contains(">>="));
    assert!(rendered.contains("::"));
    assert_eq!(
        TokenStream::from_str("/* nested /* comment */ done */ value")
            .unwrap()
            .to_string(),
        "value"
    );
    let error = TokenStream::from_str("\"unterminated").unwrap_err();
    assert_eq!(error.message(), "unexpected character '\"'");
    assert_eq!(
        error.to_compile_error().to_string(),
        "compile_error ! (\"unexpected character '\\\"'\")"
    );
    assert_eq!(TokenStream::from_str("@").unwrap().to_string(), "@");

    let string = String::from("value");
    assert_eq!(string.to_token_stream().to_string(), "\"value\"");
    let borrowed = String::from("borrowed");
    assert_eq!(borrowed.to_token_stream().to_string(), "\"borrowed\"");
    let optional = Some(Ident::new("optional"));
    assert_eq!(optional.to_token_stream().to_string(), "optional");
    assert_eq!(Vec::<Ident>::new().to_token_stream().to_string(), "");
}
