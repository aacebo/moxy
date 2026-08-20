use std::error::Error;

use moxy_fmt::{
    FmtConfig, FmtError, Formatter, Indent, Line, Mode, NewlineStyle, concat, fmt, group, if_break, indent, line, nil, text,
};
use moxy_token::{ToTokenStream, TokenTree};

#[test]
fn nodes_report_flat_widths_and_collapse_empty_concats() {
    assert_eq!(nil().flat_width(), Some(0));
    assert_eq!(text("abc").flat_width(), Some(3));
    assert_eq!(line(Line::Space).flat_width(), Some(1));
    assert_eq!(line(Line::Soft).flat_width(), Some(0));
    assert_eq!(line(Line::Hard).flat_width(), None);
    assert_eq!(concat([]), nil());
    assert_eq!(concat([nil(), text("one")]), text("one"));
    assert_eq!(group(indent(text("value"))).flat_width(), Some(5));
    assert_eq!(if_break(text("broken"), text("flat")).flat_width(), Some(4));
    assert_eq!(Line::Space.as_str(), "space");
    assert_eq!(Line::Soft.to_string(), "soft");
    assert_eq!(Line::Hard.as_str(), "hard");
}

#[test]
fn groups_choose_flat_or_broken_layout_at_the_width_boundary() {
    let document = group(concat([
        text("alpha"),
        line(Line::Space),
        if_break(text("BROKEN"), text("beta")),
    ]));
    assert_eq!(
        fmt!(&document, FmtConfig::default().with_max_width(10)).unwrap(),
        "alpha beta"
    );
    assert_eq!(
        fmt!(
            &document,
            FmtConfig::default().with_max_width(9).with_newline(NewlineStyle::Unix)
        )
        .unwrap(),
        "alpha\nBROKEN"
    );
}

#[test]
fn formatter_tracks_depth_columns_and_writes_all_public_line_modes() {
    let mut formatter = Formatter::new(
        FmtConfig::default()
            .with_indent(Indent::space(3))
            .with_newline(NewlineStyle::Unix),
    );
    assert_eq!(formatter.depth(), 0);
    assert_eq!(formatter.column(), 0);
    assert_eq!(formatter.config().indent, Indent::Space(3));
    formatter.write_node(&text("head"), Mode::Broken).unwrap();
    formatter.write_node(&line(Line::Space), Mode::Flat).unwrap();
    formatter.write_node(&text("tail\nlast"), Mode::Flat).unwrap();
    assert_eq!(formatter.column(), 4);
    formatter.write_line(Line::Soft, Mode::Flat).unwrap();
    formatter.write_newline().unwrap();
    assert_eq!(formatter.done(), "head tail\nlast\n");
}

#[test]
fn indentation_and_newline_configuration_cover_tabs_spaces_and_platform_modes() {
    assert_eq!(Indent::tab(2).width(), 2);
    assert_eq!(Indent::tab(2).spaces(), 8);
    assert_eq!(Indent::tab(2).to_string(), "\t\t");
    assert_eq!(Indent::space(3).width(), 3);
    assert_eq!(Indent::space(3).spaces(), 3);
    assert_eq!(Indent::space(3).to_string(), "   ");
    assert_eq!(NewlineStyle::Auto.as_str(), "auto");
    assert_eq!(NewlineStyle::Unix.as_str(), "unix");
    assert_eq!(NewlineStyle::Windows.as_str(), "windows");
    assert_eq!(NewlineStyle::Unix.to_string(), "\n");
    assert_eq!(NewlineStyle::Windows.to_string(), "\r\n");
    assert_eq!(NewlineStyle::Auto.to_string(), if cfg!(unix) { "\n" } else { "\r\n" });
}

#[test]
fn formatter_errors_emit_inspectable_compile_error_tokens() {
    let error = FmtError::from(std::fmt::Error);
    assert_eq!(error.to_string(), "an error occurred when formatting an argument");
    assert!(error.source().is_some());
    let tokens = error.to_compile_error();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].as_ident().unwrap().text(), "compile_error");
    assert_eq!(tokens[1].as_punct().unwrap().to_string(), "!");
    assert!(matches!(tokens[2], TokenTree::Group(_)));
    assert_eq!(error.to_token_stream().to_string(), tokens.to_string());
}
