use std::str::FromStr;

use moxy::Token;
use moxy::ast::{Parse, ParseError, Parser, Peek};
use moxy::token::{Delim, TokenStream, TokenTree};

#[test]
fn peek_and_failed_optional_parses_do_not_advance() {
    let tokens = TokenStream::from_str("fn value").unwrap();
    let parser = Parser::from_tokens(&tokens);
    let remaining = parser.remaining();

    assert!(parser.peek::<Token![fn]>());
    assert_eq!(parser.remaining(), remaining);

    assert!(parser.parse_if::<Token![struct]>().is_none());
    assert_eq!(parser.remaining(), remaining);

    let optional: Option<Token![struct]> = parser.parse().unwrap();
    assert!(optional.is_none());
    assert_eq!(parser.remaining(), remaining);

    let _: Token![fn] = parser.parse().unwrap();
    assert_eq!(parser.remaining(), remaining - 1);
}

#[test]
fn terminating_repetition_leaves_the_first_nonmatching_token() {
    let tokens = TokenStream::from_str("fn fn struct").unwrap();
    let parser = Parser::from_tokens(&tokens);
    let parsed = parser.parse_while::<Token![fn]>();

    assert_eq!(parsed.len(), 2);
    assert!(parser.peek::<Token![struct]>());

    let _: Token![struct] = parser.parse().unwrap();
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

    let inner_parser = Parser::from_tokens(&inner);
    let _: Token![fn] = inner_parser.parse().unwrap();
    assert!(inner_parser.is_empty());
}

#[test]
fn public_parse_rejects_trailing_tokens() {
    assert!(moxy::parse!("fn value" as Token![fn]).is_err());
}

#[test]
fn peek_suppresses_nested_trace_output() {
    const CHILD_ENV: &str = "MOXY_PEEK_TRACE_CHILD";

    if std::env::var_os(CHILD_ENV).is_some() {
        struct NestedParse;

        impl Parse for NestedParse {
            fn parse(parser: &Parser) -> Result<Self, ParseError> {
                let _: Token![fn] = parser.parse()?;
                Ok(Self)
            }
        }

        impl Peek for NestedParse {
            fn peek(parser: &Parser) -> bool {
                parser.parse::<Self>().is_ok()
            }
        }

        let tokens = TokenStream::from_str("fn").unwrap();
        let parser = Parser::from_tokens(&tokens).traceable();
        assert!(parser.peek::<NestedParse>());
        assert!(parser.peek::<Token![fn]>());
        return;
    }

    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "peek_suppresses_nested_trace_output", "--nocapture"])
        .env(CHILD_ENV, "1")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "child test failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("-> "), "peek leaked a trace entry:\n{stdout}");
    assert!(!stdout.contains("<- "), "peek leaked a trace result:\n{stdout}");
}
