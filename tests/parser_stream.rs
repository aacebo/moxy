use std::str::FromStr;

use moxy::Token;
use moxy::token::TokenStream;

#[test]
fn peek_and_failed_optional_parses_do_not_advance() {
    let tokens = TokenStream::from_str("fn value").unwrap();
    let mut stream = tokens.parse();
    let remaining = stream.remaining();

    assert!(stream.peek::<Token![fn]>());
    assert_eq!(stream.remaining(), remaining);

    assert!(stream.parse_if::<Token![struct]>().is_none());
    assert_eq!(stream.remaining(), remaining);

    let optional: Option<Token![struct]> = stream.parse().unwrap();
    assert!(optional.is_none());
    assert_eq!(stream.remaining(), remaining);

    let _: Token![fn] = stream.parse().unwrap();
    assert_eq!(stream.remaining(), remaining - 1);
}

#[test]
fn terminating_repetition_leaves_the_first_nonmatching_token() {
    let tokens = TokenStream::from_str("fn fn struct").unwrap();
    let mut stream = tokens.parse();
    let parsed = stream.parse_while::<Token![fn]>();

    assert_eq!(parsed.len(), 2);
    assert!(stream.peek::<Token![struct]>());

    let _: Token![struct] = stream.parse().unwrap();
    assert!(stream.is_empty());
}

#[cfg(feature = "trace")]
#[test]
fn peek_suppresses_nested_trace_output() {
    const CHILD_ENV: &str = "MOXY_PEEK_TRACE_CHILD";

    if std::env::var_os(CHILD_ENV).is_some() {
        use moxy::token::Parse;
        use moxy::token::parser::{ParseError, ParseStream};

        struct NestedParse;

        impl Parse for NestedParse {
            fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
                let _: Token![fn] = stream.parse()?;
                Ok(Self)
            }
        }

        let tokens = TokenStream::from_str("fn").unwrap();
        let mut stream = tokens.parse();
        assert!(stream.peek::<NestedParse>());
        assert!(stream.peek::<Token![fn]>());
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
