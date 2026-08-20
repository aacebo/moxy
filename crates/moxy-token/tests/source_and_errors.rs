use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use moxy_token::parser::ParseError;
use moxy_token::source::{Location, Source, SourceMap};
use moxy_token::span::{DelimSpan, RangeSpan};
use moxy_token::{Delim, Group, Ident, LexError, Span, ToTokenStream, TokenStream, TokenTree};

#[test]
fn source_maps_resolve_unicode_text_locations_paths_and_mutable_lookup() {
    let mut map = SourceMap::new();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
    let first_span = map.push("alpha\nβeta");
    assert_eq!(map.find(first_span).unwrap().text(), "alpha\nβeta");
    assert_eq!(map.find_path(first_span).as_deref(), Some("<unspecified>"));
    assert_eq!(map.find_index(first_span), Some(0));
    let second_span = map.push("second");
    assert_eq!(map.len(), 2);
    assert_eq!(map.files()[0].text(), "alpha\nβeta");
    assert_eq!(map.files()[1].text(), "second");
    assert_eq!(map.find(second_span).unwrap().text(), "second");
    assert_eq!(map.find_path(second_span).as_deref(), Some("<parsed string 1>"));
    assert_eq!(map.find_index(second_span), Some(1));
    assert_eq!(map.find_mut(second_span).unwrap().text(), "second");

    let source = &map.files()[0];
    assert_eq!(source.span().byte_range(), first_span.byte_range());
    assert_eq!(source.range(first_span), 0..11);
    assert_eq!(source.slice(first_span), "alpha\nβeta");
    assert_eq!(source.byte(first_span.byte_range().start), 0);
    assert_eq!(source.byte(first_span.byte_range().end), 11);
    assert_eq!(source.location(first_span.byte_range().start), Location::new(0, 0, 0));
    assert_eq!(source.location(first_span.byte_range().start + 6), Location::new(6, 1, 0));
    assert_eq!(
        (
            source.location(first_span.byte_range().start + 8).index(),
            source.location(first_span.byte_range().start + 8).line(),
            source.location(first_span.byte_range().start + 8).column()
        ),
        (8, 1, 2)
    );

    let default = Source::default();
    assert_eq!(default.text(), "");
    assert_eq!(default.span().byte_range(), 0..0);
}

#[test]
fn spans_delimiters_and_ranges_report_exact_boundaries_and_relations() {
    let tokens: TokenStream = "alpha beta gamma".parse().unwrap();
    let first = tokens[0].span();
    let middle = tokens[1].span();
    let last = tokens[2].span();
    let whole = first.join(last);
    assert_eq!(whole.start(), first.start());
    assert_eq!(whole.end(), last.end());
    assert_eq!(whole.byte_range(), first.byte_range().start..last.byte_range().end);
    assert_eq!(whole.len(), last.byte_range().end - first.byte_range().start);
    assert!(!whole.is_empty());
    assert!(whole.contains(middle.byte_range().start));
    assert!(middle.is_subset(&whole));
    assert!(!whole.is_subset(&middle));
    assert!(first < middle && middle < last);
    assert_eq!(Span::default(), Span::call_site());

    let delim = DelimSpan::new(first, last);
    assert_eq!(delim.open(), first);
    assert_eq!(delim.close(), last);
    assert_eq!(delim.span(), whole);
    assert_eq!(Span::from(delim), whole);
    #[cfg(feature = "serde")]
    assert_eq!(
        serde_json::to_value(delim).unwrap(),
        serde_json::json!({"open": {"start": first.byte_range().start, "end": first.byte_range().end}, "close": {"start": last.byte_range().start, "end": last.byte_range().end}})
    );

    let range = RangeSpan::new(first, last);
    assert_eq!(range.start(), first);
    assert_eq!(range.end(), last);
    assert_eq!(range.span(), whole);
    assert_eq!(Span::from(range), whole);

    let mut left = DefaultHasher::new();
    whole.hash(&mut left);
    let mut right = DefaultHasher::new();
    first.join(last).hash(&mut right);
    assert_eq!(left.finish(), right.finish());
}

#[test]
fn parse_and_lex_errors_preserve_messages_children_spans_and_compile_error_tokens() {
    let reject = LexError::new(Span::call_site());
    assert!(reject.is_reject());
    assert_eq!(reject.to_string(), "string could not be parsed");
    assert_eq!(reject.span(), Span::call_site());
    let diagnostic = LexError::new(Span::mixed_site()).message("invalid syntax");
    assert!(!diagnostic.is_reject());
    assert_eq!(diagnostic.to_string(), "invalid syntax");

    let parent = ParseError::new(Span::call_site(), "outer error")
        .combine(ParseError::new(Span::mixed_site(), "first cause"))
        .combine(ParseError::new(Span::def_site(), "second cause"));
    assert_eq!(parent.message(), "outer error");
    assert_eq!(parent.children().len(), 2);
    assert_eq!(parent.children()[0].message(), "first cause");
    assert_eq!(parent.children()[1].message(), "second cause");
    assert_eq!(parent.to_string(), "outer error\nfirst cause\nsecond cause");
    assert_eq!(
        parent.to_compile_error().to_string(),
        "compile_error ! (\"outer error\\nfirst cause\\nsecond cause\")"
    );
    assert_eq!(parent.to_token_stream(), parent.to_compile_error());

    let ok: Result<Ident, ParseError> = Ok(Ident::new("value"));
    assert_eq!(ok.to_token_stream().to_string(), "value");
    let error: Result<Ident, ParseError> = Err(parent);
    assert_eq!(
        error.to_token_stream().to_string(),
        "compile_error ! (\"outer error\\nfirst cause\\nsecond cause\")"
    );
}

#[test]
fn groups_and_token_trees_preserve_each_real_variant_and_delimiter_failure_message() {
    let mut group = Group::new(Delim::None, "inside".parse().unwrap());
    assert_eq!(group.to_string(), "inside");
    assert_eq!(group.stream().to_string(), "inside");
    assert_eq!(group.delim(), Delim::None);
    let delim_span = DelimSpan::new(Span::mixed_site(), Span::def_site());
    group.set_span(delim_span);
    assert_eq!(group.span(), delim_span);
    assert_eq!(group.to_token_tree().as_group().unwrap().stream().to_string(), "inside");
    assert_eq!(
        group.clone().into_token_tree().as_group().unwrap().stream().to_string(),
        "inside"
    );

    let keyword = TokenStream::from("pub".parse::<TokenStream>().unwrap().to_vec())
        .into_iter()
        .next()
        .unwrap();
    let punct = TokenStream::from("+".parse::<TokenStream>().unwrap().to_vec())
        .into_iter()
        .next()
        .unwrap();
    assert_eq!(keyword.text(), Some("pub"));
    assert!(keyword.is_keyword());
    assert_eq!(keyword.as_keyword().unwrap().as_str(), "pub");
    assert!(punct.is_punct());
    assert_eq!(punct.as_punct().unwrap().as_str(), "+");
    assert_eq!(keyword.clone().into_iter().collect::<Vec<TokenTree>>(), vec![keyword]);

    for (source, message) in [
        ("(", "unexpected character '('"),
        ("(]", "unexpected character '('"),
        ("[)", "unexpected character '['"),
        ("{]", "unexpected character '{'"),
    ] {
        let error = source.parse::<TokenStream>().unwrap_err();
        assert_eq!(error.message(), message);
    }
}
