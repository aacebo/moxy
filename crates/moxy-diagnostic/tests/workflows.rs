use moxy_diagnostic::{Diagnostic, Level, SpanExt, error, help, note, warn};
use moxy_token::parser::ParseError;
use moxy_token::{Span, ToTokenStream, TokenStream, TokenTree};

#[test]
fn diagnostic_macros_cover_levels_spans_children_and_level_escalation() {
    let span = "value + other".parse::<TokenStream>().unwrap().span();
    let note = note!("context", span = span, [help!("inspect this")]);
    let warning = warn!("warning", [note!("nested note")]);
    let help = help!("top-level help", span = span);
    let error = error!("failed", span = span, [warning.clone(), note.clone(), help.clone()]);

    assert_eq!(note.level(), Level::Help);
    assert_eq!(warning.level(), Level::Warning);
    assert_eq!(help.level(), Level::Help);
    assert_eq!(error.level(), Level::Error);
    assert_eq!(error.message(), Some("failed"));
    assert_eq!(error.spans(), &[span]);
    assert_eq!(error.children().len(), 3);
    assert!(error.to_string().contains("[warning]:: warning"));
}

#[test]
fn every_macro_form_and_span_extension_constructs_inspectable_diagnostics() {
    let span = Span::call_site();
    let diagnostics = [
        note!("plain note"),
        note!("spanned note", span = span),
        note!("child note", [help!("child")]),
        warn!("plain warning"),
        warn!("spanned warning", span = span),
        warn!("child warning", span = span, [note!("child")]),
        error!("plain error"),
        error!("spanned error", span = span),
        error!("child error", [help!("child")]),
        help!("plain help"),
        help!("spanned help", span = span),
        help!("child help", span = span, [note!("child")]),
        span.error("extension error"),
        span.warn("extension warning"),
        span.note("extension note"),
        span.help("extension help"),
    ];

    assert_eq!(diagnostics.len(), 16);
    assert!(diagnostics.iter().all(|diagnostic| diagnostic.message().is_some()));
    assert!(diagnostics.iter().any(|diagnostic| diagnostic.children().len() == 1));
    assert!(diagnostics.iter().any(|diagnostic| diagnostic.spans() == [span]));
}

#[test]
fn compile_error_streams_are_structurally_parseable_and_preserve_the_message() {
    let diagnostic = Diagnostic::new()
        .level(Level::Error)
        .span(Span::call_site())
        .message("invalid generated syntax")
        .child(help!("try a valid expression"))
        .build();
    let tokens = diagnostic.to_compile_error();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].as_ident().unwrap().text(), "compile_error");
    assert_eq!(tokens[1].as_punct().unwrap().to_string(), "!");
    let group = match &tokens[2] {
        TokenTree::Group(group) => group,
        token => panic!("expected group, got {token:?}"),
    };
    assert!(
        group.stream()[0]
            .as_literal()
            .unwrap()
            .repr()
            .contains("invalid generated syntax")
    );
    assert_eq!(diagnostic.to_token_stream().to_string(), tokens.to_string());
    assert_eq!(diagnostic.clone().emit().to_string(), tokens.to_string());
}

#[test]
fn parse_errors_become_nested_error_diagnostics() {
    let child = ParseError::new(Span::def_site(), "expected expression");
    let error = ParseError::new(Span::call_site(), "invalid input").combine(child);
    let borrowed = Diagnostic::from(&error);
    let owned = Diagnostic::from(error);
    assert_eq!(borrowed.level(), Level::Error);
    assert_eq!(borrowed.message(), Some("invalid input"));
    assert_eq!(borrowed.children()[0].message(), Some("expected expression"));
    assert_eq!(borrowed, owned);
}

#[test]
fn levels_expose_order_predicates_numeric_values_and_display_names() {
    for (level, value, name) in [
        (Level::Unknown, 0, "??"),
        (Level::Note, 1, "note"),
        (Level::Help, 2, "help"),
        (Level::Warning, 3, "warning"),
        (Level::Error, 4, "error"),
    ] {
        assert_eq!(level.as_u8(), value);
        assert_eq!(level.as_str(), name);
        assert_eq!(level.to_string(), name);
    }
    assert!(Level::Unknown.is_unknown());
    assert!(Level::Note.is_note());
    assert!(Level::Help.is_help());
    assert!(Level::Warning.is_warning());
    assert!(Level::Error.is_error());
    assert!(Level::Error > Level::Warning);
}
