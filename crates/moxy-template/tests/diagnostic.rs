use moxy_diagnostic::{Level, SpanExt};
use moxy_token::Span;
use moxy_token::parse::ParseError;

#[test]
fn parse_error_becomes_error_diagnostic() {
    let diag = moxy_diagnostic::Diagnostic::from(ParseError::new(Span::default(), "expected `if`, `for`, or `match`"));

    assert_eq!(diag.level(), Level::Error);
    assert_eq!(diag.message(), Some("expected `if`, `for`, or `match`"));
}

#[test]
fn warn_diagnostic_renders() {
    let stream = Span::default().warn("deprecated template syntax").emit();
    let s = stream.to_string();

    assert!(s.contains("compile_error"), "expected compile_error in: {s}");
    assert!(s.contains("warning"), "expected level in: {s}");
    assert!(s.contains("deprecated template syntax"), "expected message in: {s}");
}

#[test]
fn note_diagnostic_renders() {
    let stream = Span::default().note("interpolation runs at render time").emit();
    let s = stream.to_string();

    assert!(s.contains("compile_error"), "expected compile_error in: {s}");
    assert!(s.contains("interpolation runs at render time"), "expected message in: {s}");
}
