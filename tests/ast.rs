use std::str::FromStr;

use moxy::Token;
use moxy::ast::{Item, Parse, Parser, Punctuated};
use moxy::token::{Ident, Span, TokenStream};

fn parse_token<T: Parse>(source: &str) -> Result<T, moxy::ast::ParseError> {
    let tokens = TokenStream::from_str(source).unwrap();
    let parser = Parser::from_tokens(&tokens);
    let value = T::parse(&parser)?;
    assert!(parser.is_empty(), "unparsed tokens remain in `{source}`");
    Ok(value)
}

#[test]
fn punctuated_parsing_and_mutation_cover_list_shapes() {
    type Idents = Punctuated<Ident, Token![,]>;

    let tokens = TokenStream::from_str("first, second,").unwrap();
    let parser = Parser::from_tokens(&tokens);
    let mut terminated = Idents::parse_terminated(&parser).unwrap();
    assert!(parser.is_empty());
    assert_eq!(terminated.len(), 2);
    assert!(terminated.is_trailing());
    assert_eq!(terminated.iter().map(Ident::text).collect::<Vec<_>>(), ["first", "second"]);
    assert_eq!(terminated.pop_punct().unwrap().to_string(), ",");
    assert!(!terminated.is_trailing());
    terminated.push_punct(Default::default());
    terminated.push_value(Ident::new("third"));
    terminated.insert(1, Ident::new("middle"));
    assert_eq!(
        terminated.iter().map(Ident::text).collect::<Vec<_>>(),
        ["first", "middle", "second", "third"]
    );
    assert_eq!(terminated.pop().unwrap().into_value().text(), "third");

    let tokens = TokenStream::from_str("left, right").unwrap();
    let parser = Parser::from_tokens(&tokens);
    let separated = Idents::parse_separated_nonempty(&parser).unwrap();
    assert!(parser.is_empty());
    assert_eq!(separated.len(), 2);
    assert!(!separated.is_trailing());

    let empty = TokenStream::new();
    let parser = Parser::from_tokens(&empty);
    assert!(Idents::parse_terminated(&parser).unwrap().is_empty());
}

#[test]
fn item_parser_accepts_each_top_level_item_variant() {
    let cases: &[(&str, fn(&Item) -> bool)] = &[
        ("use path::name;", Item::is_use),
        ("extern crate core;", Item::is_extern_crate),
        ("mod inner {}", Item::is_mod),
        ("fn function() {}", Item::is_fn),
        ("struct Unit;", Item::is_struct),
        ("enum Choice { One }", Item::is_enum),
        ("union Bits { value: u32 }", Item::is_union),
        ("trait Trait {}", Item::is_trait),
        ("trait Alias = Send;", Item::is_trait_alias),
        ("impl Trait for Unit {}", Item::is_impl),
        ("type Alias = u32;", Item::is_type_alias),
        ("const VALUE: u32 = 1;", Item::is_const),
        ("static VALUE: u32 = 1;", Item::is_static),
        ("call!();", Item::is_macro),
        ("macro_rules! rules { () => {} }", Item::is_macro2),
        ("extern \"C\" {}", Item::is_foreign_mod),
    ];

    for (source, predicate) in cases {
        let item = parse_token::<Item>(source).unwrap_or_else(|error| panic!("{source}: {error}"));
        assert!(predicate(&item), "wrong item variant for `{source}`");
    }
}

#[test]
fn lexical_spans_track_source_ranges_and_support_span_operations() {
    let tokens = TokenStream::from_str("first second\nthird").unwrap();
    let first = tokens[0].span();
    let second = tokens[1].span();
    let third = tokens[2].span();

    assert_eq!(first.byte_range(), 0..5);
    assert_eq!(second.byte_range(), 6..12);
    assert_eq!(third.byte_range(), 13..18);
    assert!(first.contains(0));
    assert!(!first.contains(5));
    assert!(first.is_subset(&first.join(second)));
    assert_eq!(first.join(second).byte_range(), 0..12);
    let (head, tail) = third.split(2);
    assert_eq!(head.len() + tail.len(), third.len());
    assert!(Span::call_site().is_empty());
}
