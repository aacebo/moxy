use std::str::FromStr;

use moxy::Token;
use moxy::ast::{Parse, Parser};
use moxy::token::{Delim, TokenStream, TokenTree};

#[test]
fn peek_and_optional_parses_do_not_advance() {
    let tokens = TokenStream::from_str("fn value").unwrap();
    let parser = Parser::from_tokens(&tokens);
    let remaining = parser.remaining();

    assert!(<Token![fn]>::peek(parser.cursor()));
    assert_eq!(parser.remaining(), remaining);

    let optional = Option::<Token![struct]>::parse(&parser).unwrap();
    assert!(optional.is_none());
    assert_eq!(parser.remaining(), remaining);

    <Token![fn]>::parse(&parser).unwrap();
    assert_eq!(parser.remaining(), remaining - 1);

    let tokens = TokenStream::from_str("fn").unwrap();
    let parser = Parser::from_tokens(&tokens);
    assert!(<Token![struct]>::parse(&parser).is_err());
    assert!(!parser.is_empty());
}

#[test]
fn terminating_repetition_leaves_the_first_nonmatching_token() {
    let tokens = TokenStream::from_str("fn fn struct").unwrap();
    let parser = Parser::from_tokens(&tokens);
    let parsed = parser.parse_while::<Token![fn]>();

    assert_eq!(parsed.len(), 2);
    assert!(<Token![struct]>::peek(parser.cursor()));

    <Token![struct]>::parse(&parser).unwrap();
    assert!(parser.is_empty());
}

#[test]
fn cursor_access_is_bounds_safe_and_tracks_advancement() {
    let empty = TokenStream::new();
    let parser = Parser::from_tokens(&empty);
    assert!(parser.curr().is_none());
    assert!(parser.next().is_none());
    assert!(parser.prev().is_none());
    assert!(parser.nth(usize::MAX).is_none());
    assert!(parser.advance().is_none());

    let tokens = TokenStream::from_str("fn value").unwrap();
    let parser = Parser::from_tokens(&tokens);
    assert!(matches!(parser.curr(), Some(TokenTree::Keyword(_))));
    assert!(matches!(parser.next(), Some(TokenTree::Ident(_))));
    parser.advance();
    assert!(matches!(parser.prev(), Some(TokenTree::Keyword(_))));
    assert!(matches!(parser.curr(), Some(TokenTree::Ident(_))));
}

#[test]
fn group_parsing_creates_an_independent_nested_parser() {
    let tokens = TokenStream::from_str("(fn)").unwrap();
    let parser = Parser::from_tokens(&tokens);
    let inner = parser.parse_group(Delim::Paren).unwrap();
    assert!(parser.is_empty());

    <Token![fn]>::parse(&inner).unwrap();
    assert!(inner.is_empty());
}

#[test]
fn public_parse_rejects_trailing_tokens() {
    assert!(moxy::parse!("fn value" as Token![fn]).is_err());
}

#[test]
fn delimited_lists_that_stop_early_are_errors() {
    for (source, message) in [
        ("fn f(x: u8 y: u8) {}", "unexpected trailing input"),
        ("fn f(x: Vec<u8 u8>) {}", "unexpected trailing input"),
        ("fn f() { g(a b) }", "expected `,` punctuation"),
        ("fn f() { match x { 1 => a b } }", "expected `=>` punctuation"),
        ("fn f() { let s = S { a: 1 b: 2 }; }", "unexpected trailing input"),
        ("fn f() { [1 2] }", "unexpected trailing input"),
        ("fn f() { (1 2) }", "unexpected trailing input"),
        ("fn f() { x.y::<u8 u8>() }", "expected `>` punctuation"),
        ("impl A { fn f() {} @ }", "unexpected trailing input"),
        ("mod m { fn f() {} @ }", "unexpected trailing input"),
    ] {
        let error = moxy::parse!(source as moxy::ast::Item)
            .err()
            .unwrap_or_else(|| panic!("`{source}` parsed"));
        assert_eq!(error.message(), message, "`{source}`");
    }
}

#[test]
fn combined_errors_emit_one_compile_error_each() {
    let tokens = TokenStream::from_str("first second third").unwrap();
    let spans: Vec<moxy::token::Span> = tokens.iter().map(moxy::token::Spanner::span).collect();
    let error = moxy::ast::ParseError::new(spans[0], "first failed")
        .combine(moxy::ast::ParseError::new(spans[1], "second failed"))
        .combine(moxy::ast::ParseError::new(spans[2], "third failed"));

    assert_eq!(
        error.to_compile_error().to_string(),
        "compile_error ! (\"first failed\") ; compile_error ! (\"second failed\") ; compile_error ! (\"third failed\") ;"
    );
}

#[test]
fn spanned_errors_run_from_the_first_token_to_the_last() {
    let tokens = TokenStream::from_str("call(a, b) + Vec<u8>").unwrap();
    let error = moxy::ast::ParseError::new_spanned(&tokens, "unsupported");
    let emitted: Vec<TokenTree> = error.to_compile_error().iter().cloned().collect();
    let TokenTree::Ident(first) = tokens.iter().next().unwrap() else {
        panic!("expected an identifier");
    };
    let last = tokens.iter().last().unwrap().span();

    assert_eq!(error.span(), first.span());
    assert_eq!(emitted[0].span(), first.span());
    assert_eq!(emitted[1].span(), first.span());
    assert_eq!(emitted[3].span(), last);
    assert_eq!(error.to_compile_error().to_string(), "compile_error ! (\"unsupported\") ;");

    let tokens = TokenStream::from_str("(a, b)").unwrap();
    let TokenTree::Group(group) = tokens.iter().next().unwrap() else {
        panic!("expected a group");
    };
    let emitted: Vec<TokenTree> = moxy::ast::ParseError::new_spanned(&tokens, "unsupported")
        .to_compile_error()
        .iter()
        .cloned()
        .collect();
    assert_eq!(emitted[0].span(), group.span.open());
    assert_eq!(emitted[3].span(), group.span.close());
}

#[cfg(feature = "diagnostic")]
#[test]
fn diagnostics_from_spanned_errors_keep_both_ends() {
    let tokens = TokenStream::from_str("call(a, b) + Vec<u8>").unwrap();
    let error = moxy::ast::ParseError::new_spanned(&tokens, "unsupported");
    let diagnostic = moxy::diagnostic::Diagnostic::from(&error);

    assert_eq!(diagnostic.spans(), [error.span(), error.end_span()]);
    let emitted: Vec<TokenTree> = diagnostic.to_compile_error().iter().cloned().collect();
    assert_eq!(emitted[0].span(), error.span());
    assert_eq!(emitted[3].span(), error.end_span());
}

// Runs `parse` on a thread with the 2 MiB stack that spawned threads and
// tests get by default, which input at the depth limit must fit in a debug build
fn on_small_stack<T: Send + 'static>(parse: impl FnOnce() -> T + Send + 'static) -> T {
    std::thread::Builder::new()
        .stack_size(2 << 20)
        .spawn(parse)
        .unwrap()
        .join()
        .unwrap()
}

#[test]
fn input_nested_past_the_depth_limit_is_an_error() {
    let depth = moxy::token::MAX_DEPTH + 1;
    let message = format!("nesting is deeper than {} levels", moxy::token::MAX_DEPTH);

    for (source, kind) in [
        (format!("{}a", "!".repeat(depth)), "expression"),
        (format!("{}a{}", "(".repeat(depth), ")".repeat(depth)), "expression"),
        (format!("{}a{}", "{".repeat(depth), "}".repeat(depth)), "expression"),
        (format!("{}a{}", "f(".repeat(depth), ")".repeat(depth)), "expression"),
        (format!("{}a", "|x| ".repeat(depth)), "expression"),
        (format!("{}a", "return ".repeat(depth)), "expression"),
        (format!("{}a", "a = ".repeat(depth)), "expression"),
        (
            format!("{}a{}", "match a { _ => ".repeat(depth), " }".repeat(depth)),
            "expression",
        ),
        (format!("if a {{}}{} else {{}}", " else if a {}".repeat(depth)), "expression"),
        (format!("{}u8", "&".repeat(depth)), "type"),
        (format!("{}u8{}", "Vec<".repeat(depth), ">".repeat(depth)), "type"),
        (format!("{}a", "&".repeat(depth)), "pattern"),
        (format!("{}a{}", "(".repeat(depth), ")".repeat(depth)), "pattern"),
        (format!("{}{}", "mod m { ".repeat(depth), "}".repeat(depth)), "file"),
    ] {
        let label: String = source.chars().take(16).collect();
        let error = on_small_stack(move || {
            let error = match kind {
                "expression" => moxy::parse!(source as moxy::ast::Expr).err(),
                "type" => moxy::parse!(source as moxy::ast::Type).err(),
                "pattern" => moxy::parse!(source as moxy::ast::Pattern).err(),
                _ => moxy::parse!(source as moxy::ast::File).err(),
            };
            error.map(|error| error.message().to_string())
        });
        assert_eq!(error.as_deref(), Some(message.as_str()), "{kind} {label}");
    }

    let error = on_small_stack(move || {
        format!("{}{}", "[".repeat(depth), "]".repeat(depth))
            .parse::<moxy::token::TokenStream>()
            .unwrap_err()
            .to_string()
    });
    assert_eq!(error, message);
}

#[test]
fn input_nested_to_the_depth_limit_parses_on_a_small_stack() {
    let depth = moxy::token::MAX_DEPTH - 1;

    let parsed = on_small_stack(move || {
        let parens = format!("{}a{}", "(".repeat(depth), ")".repeat(depth));
        let arms = format!("{}a{}", "match a { _ => ".repeat(depth), " }".repeat(depth));
        let closures = format!("{}a", "|x| ".repeat(depth));
        let generics = format!("{}u8{}", "Vec<".repeat(depth), ">".repeat(depth));
        let patterns = format!("{}a{}", "(".repeat(depth), ")".repeat(depth));
        let modules = format!("{}{}", "mod m { ".repeat(depth), "}".repeat(depth));
        [
            moxy::parse!(parens as moxy::ast::Expr).is_ok(),
            moxy::parse!(arms as moxy::ast::Expr).is_ok(),
            moxy::parse!(closures as moxy::ast::Expr).is_ok(),
            moxy::parse!(generics as moxy::ast::Type).is_ok(),
            moxy::parse!(patterns as moxy::ast::Pattern).is_ok(),
            moxy::parse!(modules as moxy::ast::File).is_ok(),
        ]
    });

    assert_eq!(parsed, [true; 6]);
}

#[test]
fn an_error_inside_a_list_of_items_is_reported_where_it_is() {
    let source = "mod m {\n    fn f() {}\n    fn g() { let = 1; }\n}";
    let error = moxy::parse!(source as moxy::ast::File).err().unwrap();
    assert_eq!(error.span().start().line(), 2);
    assert_ne!(error.message(), "unexpected trailing input");

    let source = "fn f() {}\nstruct S { a: }";
    let error = moxy::parse!(source as moxy::ast::File).err().unwrap();
    assert_eq!(error.span().start().line(), 1);
    assert_ne!(error.message(), "unexpected trailing input");
}
