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

// Nodes that parse a whole form before they reject it name the form they found
#[test]
fn a_rejected_form_is_named() {
    use moxy::ast::{FromMeta, ItemTrait, Meta, PatOr};

    let or = moxy::parse!("a @ b" as PatOr).err().unwrap();
    let alias = moxy::parse!("trait A = B;" as ItemTrait).err().unwrap();
    let meta: Meta = moxy::parse!("a = 1").unwrap();
    let flag = bool::from_meta(&meta).err().unwrap();

    assert_eq!(or.message(), "expected or-pattern, found `a`");
    assert_eq!(alias.message(), "expected trait, found trait alias");
    assert_eq!(flag.message(), "expected boolean literal, found `1`");
}

// An error at the end of a group points at the delimiter that closes it, an
// empty group included, and the error inside attribute arguments names it
#[test]
fn an_error_at_the_end_of_a_group_points_at_its_closing_delimiter() {
    use moxy::ast::Meta;

    let closing = |source: &str| {
        let error = moxy::parse!(source as moxy::ast::File).err().unwrap();
        (error.message().to_string(), error.span().start().column())
    };
    let meta: Meta = moxy::parse!("a(b =)").unwrap();
    let nested = meta.for_each(|_| Ok(())).err().unwrap();

    assert_eq!(closing("struct S { a: }"), ("expected type, found `}`".to_owned(), 14));
    assert_eq!(closing("type T = [];"), ("expected type, found `]`".to_owned(), 10));
    assert_eq!(nested.message(), "expected expression, found `)`");
}

// A group without delimiters, which a stream from the compiler can hold, is
// quoted by its first token
#[test]
fn an_error_quotes_a_group_without_delimiters_by_its_first_token() {
    use moxy::token::Group;

    let inner: TokenStream = "; ;".parse().unwrap();
    let tokens = TokenStream::from_iter([TokenTree::Group(Group::new(Delim::None, inner))]);
    let error = moxy::parse!(tokens as moxy::token::Ident).err().unwrap();
    assert_eq!(error.message(), "expected identifier, found `; ...`");

    let tokens = TokenStream::from_iter([TokenTree::Group(Group::new(Delim::None, TokenStream::new()))]);
    let empty = moxy::parse!(tokens as moxy::token::Ident).err().unwrap();
    let inner: TokenStream = "a".parse().unwrap();
    let tokens = TokenStream::from_iter([TokenTree::Group(Group::new(Delim::None, inner))]);
    let parser = moxy::ast::Parser::from_tokens(&tokens);
    let group = parser.parse_group(Delim::None).unwrap();
    group.advance();
    let end = group.expected("identifier");
    let none = TokenStream::new();

    assert_eq!(empty.message(), "expected identifier, found empty group");
    assert_eq!(end.message(), "expected identifier, found end of group");
    assert_eq!(
        moxy::ast::Parser::from_tokens(&none)
            .parse_group(Delim::None)
            .err()
            .unwrap()
            .message(),
        "expected group without delimiters, found end of input"
    );
}

// A message names what was expected and what was found, with code in
// backticks, in the lexer and the parser alike
#[test]
fn error_messages_name_what_was_expected_and_found() {
    for (source, message) in [
        ("\"abc", "unterminated string literal"),
        ("r\"abc", "unterminated raw string literal"),
        ("0x", "invalid literal"),
        ("(]", "expected `)`, found `]`"),
        ("fn f(", "unclosed delimiter `(`"),
        ("fn f() {})", "unexpected closing delimiter `)`"),
        ("fn f() { € }", "unexpected character `€`"),
        ("struct S<T", "expected `>`, found end of input"),
        ("fn f() -> {}", "expected type, found `{}`"),
        ("fn f() { a b }", "expected `;`, found `b`"),
        ("$", "unexpected `$`"),
        (
            "trait A { m!() }",
            "expected `;` after a macro item in parentheses or brackets, found `}`",
        ),
        ("fn f() { x. }", "expected identifier, found `}`"),
        ("struct S { a: }", "expected type, found `}`"),
        ("fn f() { let x = ; }", "expected expression, found `;`"),
        ("fn f() -> \"a`b\" {}", "expected type, found `` \"a`b\" ``"),
        ("impl &A for B {}", "expected trait path, found `&`"),
        ("fn f(&u8) {}", "expected parameter name, found `&`"),
        ("type T = [];", "expected type, found `]`"),
        ("fn f() { 1e`; }", "expected at least one digit in exponent, found `` ` ``"),
        ("fn f() { x.1u8; }", "expected tuple index, found `1u8`"),
        ("fn f(u8) {}", "expected parameter name, found `u8`"),
    ] {
        let error = moxy::parse!(source as moxy::ast::File)
            .err()
            .unwrap_or_else(|| panic!("`{source}` parsed"));
        assert_eq!(error.message(), message, "`{source}`");
        assert!(super::differential::follows_style(message), "`{message}`");
    }
}

#[test]
fn delimited_lists_that_stop_early_are_errors() {
    for (source, message) in [
        ("fn f(x: u8 y: u8) {}", "unexpected `y`"),
        ("fn f(x: Vec<u8 u8>) {}", "expected `>`, found `u8`"),
        ("fn f() { g(a b) }", "expected `,`, found `b`"),
        ("fn f() { match x { 1 => a b } }", "expected `,` after a match arm, found `b`"),
        ("fn f() { let s = S { a: 1 b: 2 }; }", "unexpected `b`"),
        ("fn f() { [1 2] }", "unexpected `2`"),
        ("fn f() { (1 2) }", "unexpected `2`"),
        ("fn f() { x.y::<u8 u8>() }", "expected `>`, found `u8`"),
        ("impl A { fn f() {} @ }", "unexpected `@`"),
        ("mod m { fn f() {} @ }", "unexpected `@`"),
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

// Runs `parse` on a thread with three quarters of the 2 MiB stack that spawned
// threads and tests get by default, so input at the depth limit fits in a debug
// build with a quarter to spare. An instrumented coverage build takes up to
// about 1.75 MiB, and gets the whole 2 MiB
fn on_small_stack<T: Send + 'static>(parse: impl FnOnce() -> T + Send + 'static) -> T {
    let stack = if cfg!(coverage) { 2 << 20 } else { 3 << 19 };

    std::thread::Builder::new()
        .stack_size(stack)
        .spawn(parse)
        .unwrap()
        .join()
        .unwrap()
}

// Every recursive construct of the grammar, as `(name, kind, before, open,
// center, close, after, unbounded)`: `open` and `close` nest around `center`
// inside `before` and `after`. An unbounded construct parses at any depth the
// lexer accepts: macro bodies and attribute arguments, which are tokens, and
// a chain, which holds its steps flat
const NESTED: &[(&str, char, &str, &str, &str, &str, &str, bool)] = &[
    ("neg", 'e', "", "-", "x", "", "", false),
    ("not", 'e', "", "!", "x", "", "", false),
    ("deref", 'e', "", "*", "x", "", "", false),
    ("ref", 'e', "", "& ", "x", "", "", false),
    ("ref mut", 'e', "", "&mut ", "x", "", "", false),
    ("raw ref", 'e', "", "&raw const ", "x", "", "", false),
    ("return", 'e', "", "return ", "x", "", "", false),
    ("break", 'e', "", "break ", "x", "", "", false),
    ("break label", 'e', "", "break 'a ", "x", "", "", false),
    ("yield", 'e', "", "yield ", "x", "", "", false),
    ("closure", 'e', "", "|| ", "x", "", "", false),
    ("closure param", 'e', "", "|x| ", "x", "", "", false),
    ("closure in an operand", 'e', "", "|| 0 + ", "x", "", "", false),
    ("move closure", 'e', "", "move || ", "x", "", "", false),
    ("async closure", 'e', "", "async || ", "x", "", "", false),
    ("static closure", 'e', "", "static || ", "x", "", "", false),
    ("assign", 'e', "", "a = ", "x", "", "", false),
    ("compound assign", 'e', "", "a += ", "x", "", "", false),
    ("range prefix", 'e', "", ".. ", "x", "", "", false),
    ("range inclusive prefix", 'e', "", "..= ", "x", "", "", false),
    ("assign range", 'e', "a", " = b..", "", "", "", false),
    ("binary unary", 'e', "", "a + -", "x", "", "", true),
    ("cast unary", 'e', "", "-", "x as T", "", "", false),
    ("turbofish", 'e', "a::<", "Vec<", "u8", ">", ">()", false),
    ("paren", 'e', "", "(", "x", ")", "", false),
    (
        "left operator ladder",
        'e',
        "",
        "(",
        "0",
        ")*0+0<<0&0^0|0==0&&0||0",
        "",
        false,
    ),
    ("left chain", 'e', "", "(", "0", ")*0+0", "", false),
    ("assigned chain", 'e', "", "(", "a", ")+0 = 0", "", false),
    ("compound assigned chain", 'e', "", "(", "a", ")+0 += 0", "", false),
    ("range start chain", 'e', "", "(", "0", ")+0..", "", false),
    ("range end chain", 'e', "", "(..0+", "0", ")", "", false),
    ("assignment after a range", 'e', "", "(", "a", ") = 0.. = 0", "", false),
    (
        "block-like statement chain",
        'b',
        "",
        "{ { ",
        "x",
        " }.a*0+0 = 0 }",
        "",
        false,
    ),
    ("let scrutinee chain", 'e', "", "if let a = (", "0", ")*0+0 {}", "", false),
    ("tuple", 'e', "", "(", "x", ",)", "", false),
    ("array", 'e', "", "[", "x", "]", "", false),
    ("repeat value", 'e', "", "[", "x", "; 1]", "", false),
    ("repeat length", 'e', "", "[x; ", "1", "]", "", false),
    ("block", 'e', "", "{ ", "x", " }", "", false),
    ("call", 'e', "", "f(", "x", ")", "", false),
    ("method", 'e', "", "a.b(", "x", ")", "", false),
    ("index", 'e', "", "a[", "x", "]", "", false),
    ("struct literal", 'e', "", "S { a: ", "x", " }", "", false),
    ("struct base", 'e', "", "S { ..", "x", " }", "", false),
    ("if body", 'e', "", "if a { ", "x", " }", "", false),
    ("else body", 'e', "", "if a {} else { ", "x", " }", "", false),
    ("if condition", 'e', "", "if ", "x", " {}", "", false),
    ("if let", 'e', "", "if let a = ", "x", " {}", "", false),
    ("while body", 'e', "", "while a { ", "x", " }", "", false),
    ("while condition", 'e', "", "while ", "x", " {}", "", false),
    ("loop", 'e', "", "loop { ", "x", " }", "", false),
    ("labeled loop", 'e', "", "'a: loop { ", "x", " }", "", false),
    ("for body", 'e', "", "for x in y { ", "x", " }", "", false),
    ("for iterator", 'e', "", "for x in ", "x", " {}", "", false),
    ("unsafe", 'e', "", "unsafe { ", "x", " }", "", false),
    ("async", 'e', "", "async { ", "x", " }", "", false),
    ("async move", 'e', "", "async move { ", "x", " }", "", false),
    ("const block", 'e', "", "const { ", "x", " }", "", false),
    ("try block", 'e', "", "try { ", "x", " }", "", false),
    ("labeled block", 'e', "", "'a: { ", "x", " }", "", false),
    ("match arm", 'e', "", "match a { _ => ", "x", " }", "", false),
    ("match arm block", 'e', "", "match a { _ => { ", "x", " } }", "", false),
    ("match scrutinee", 'e', "", "match ", "x", " {}", "", false),
    ("match guard", 'e', "", "match a { _ if ", "x", " => {} }", "", false),
    ("closure block", 'e', "", "|| { ", "x", " }", "", false),
    ("closure return type", 'e', "", "|| -> T { ", "x", " }", "", false),
    ("macro call", 'e', "", "m!(", "x", ")", "", true),
    ("macro braces", 'e', "", "m! { ", "x", " }", "", true),
    ("let in block", 'e', "", "{ let a = ", "x", "; a }", "", false),
    ("let else", 'e', "", "{ let a = b else { ", "x", " }; }", "", false),
    ("attribute arguments", 'e', "#[a", "(", "x", ")", "] y", true),
    ("const generic block", 'e', "", "a::<{ ", "1", " }>()", "", false),
    ("closure pattern", 'e', "|", "(", "x", ")", "| x", false),
    ("ref type", 't', "", "&", "u8", "", "", false),
    ("ref mut type", 't', "", "&mut ", "u8", "", "", false),
    ("lifetime ref type", 't', "", "&'a ", "u8", "", "", false),
    ("const pointer", 't', "", "*const ", "u8", "", "", false),
    ("mut pointer", 't', "", "*mut ", "u8", "", "", false),
    ("generics", 't', "", "Vec<", "u8", ">", "", false),
    ("fn return", 't', "", "fn() -> ", "u8", "", "", false),
    (
        "extern fn return",
        't',
        "",
        "unsafe extern \"C\" fn() -> ",
        "u8",
        "",
        "",
        false,
    ),
    ("for fn return", 't', "", "for<'a> fn() -> ", "u8", "", "", false),
    ("impl Fn", 't', "", "impl Fn() -> ", "u8", "", "", false),
    ("dyn Fn", 't', "", "dyn Fn() -> ", "u8", "", "", false),
    ("Fn sugar", 't', "", "Fn(u8) -> ", "u8", "", "", false),
    ("box dyn generics", 't', "", "Box<dyn A<", "u8", ">>", "", false),
    ("assoc type", 't', "", "A<B = ", "u8", ">", "", false),
    ("assoc bound", 't', "", "A<B: ", "C", ">", "", false),
    ("assoc binding arguments", 't', "X<", "B<", "C", "> = u8", ">", false),
    ("assoc bound arguments", 't', "X<", "B<", "C", ">: D", ">", false),
    ("qself", 't', "", "<", "T", " as A>::B", "", false),
    ("type turbofish", 't', "", "A::<", "u8", ">", "", false),
    ("impl generics", 't', "", "impl A<", "u8", ">", "", false),
    ("lifetime generics", 't', "", "A<'a, ", "u8", ">", "", false),
    ("paren type", 't', "", "(", "u8", ")", "", false),
    ("tuple type", 't', "", "(", "u8", ",)", "", false),
    ("slice", 't', "", "[", "u8", "]", "", false),
    ("array type", 't', "", "[", "u8", "; 1]", "", false),
    ("fn params", 't', "", "fn(", "u8", ")", "", false),
    ("Fn args", 't', "", "Fn(", "u8", ")", "", false),
    ("type macro", 't', "", "m!(", "u8", ")", "", true),
    ("array length", 't', "", "[u8; { let x: ", "u8", "; 1 }]", "", false),
    ("const argument", 't', "", "A<{ let x: ", "u8", "; 1 }>", "", false),
    ("ref pattern", 'p', "", "&", "x", "", "", false),
    ("ref mut pattern", 'p', "", "&mut ", "x", "", "", false),
    ("box pattern", 'p', "", "box ", "x", "", "", false),
    ("binding", 'p', "", "a @ ", "x", "", "", false),
    ("mut binding", 'p', "", "ref mut a @ ", "x", "", "", false),
    ("paren pattern", 'p', "", "(", "x", ")", "", false),
    ("tuple pattern", 'p', "", "(", "x", ",)", "", false),
    ("slice pattern", 'p', "", "[", "x", "]", "", false),
    ("tuple struct pattern", 'p', "", "S(", "x", ")", "", false),
    ("struct pattern", 'p', "", "S { a: ", "x", " }", "", false),
    ("const pattern", 'p', "", "const { ", "1", " }", "", false),
    ("pattern turbofish", 'p', "A::<", "Vec<", "u8", ">", ">", false),
    ("pattern macro", 'p', "", "m!(", "x", ")", "", true),
    ("range pattern", 'p', "", "(", "1..=2", ")", "", false),
    ("modules", 'f', "", "mod m { ", "", "}", "", false),
    ("functions", 'f', "", "fn f() { ", "", "}", "", false),
    ("impls", 'f', "", "impl A { fn f() { ", "", "} }", "", false),
    ("traits", 'f', "", "trait A { fn f() { ", "", "} }", "", false),
    ("const items", 'f', "", "const _: () = { ", "", " };", "", false),
    ("static items", 'f', "", "static S: () = { ", "", " };", "", false),
    ("enum discriminants", 'f', "", "enum E { A = { ", "", " 1 } }", "", false),
    ("struct array fields", 'f', "", "struct S([u8; { ", "", " 1 }]);", "", false),
    ("type aliases", 'f', "", "type A = [u8; { ", "", " 1 }];", "", false),
    ("impl consts", 'f', "", "impl A { const X: () = { ", "", " }; }", "", false),
    ("trait consts", 'f', "", "trait A { const X: () = { ", "", " }; }", "", false),
    (
        "extern statics",
        'f',
        "",
        "extern \"C\" { static X: [u8; { ",
        "",
        " 1 }]; }",
        "",
        false,
    ),
    ("let initializers", 'f', "", "fn f() { let a = { ", "", " }; }", "", false),
    ("use groups", 'f', "use ", "a::{", "b", "}", ";", false),
    ("item attributes", 'f', "#[a", "(", "x", ")", "] fn f() {}", true),
    ("where clauses", 'f', "fn f() where T: ", "A<", "B", ">", " {}", false),
    ("generic bounds", 'f', "fn f<T: ", "A<", "B", ">", ">() {}", false),
    ("generic defaults", 'f', "struct S<T = ", "A<", "B", ">", ">;", false),
    ("macro_rules", 'f', "macro_rules! m { ", "(", "", ")", " }", true),
    ("macro 2.0", 'f', "macro m { ", "(", "", ")", " }", true),
    ("item macro", 'f', "", "m! { ", "", " }", "", true),
    ("fn return impl", 'f', "fn f() -> ", "impl Fn() -> ", "u8", "", " {}", false),
    (
        "closure return impl",
        'e',
        "|| -> ",
        "impl Fn() -> ",
        "u8",
        "",
        " { x }",
        false,
    ),
    ("nested blocks", 'b', "", "{ ", "x", " }", "", false),
    ("ifs in a block", 'b', "{ ", "if a { ", "x", " }", " }", false),
    ("let chain", 'e', "", "if let a = b && (", "c", ") {}", "", false),
    ("while let", 'e', "", "while let a = (", "b", ") {}", "", false),
    ("labeled while", 'e', "", "'a: while a { ", "x", " }", "", false),
    ("labeled for", 'e', "", "'a: for x in y { ", "x", " }", "", false),
    ("let pattern", 'b', "{ let ", "(", "x", ")", " = y; }", false),
    ("arm pattern", 'e', "match a { ", "(", "x", ")", " => {} }", false),
    ("for pattern", 'e', "for ", "(", "x", ")", " in y {}", false),
    ("parameter pattern", 'f', "fn f(", "(", "x", ")", ": u8) {}", false),
    ("or-pattern", 'p', "", "(", "x", " | y)", "", false),
    ("qualified pattern", 'p', "", "<", "T", " as A>::B", "", false),
    ("qualified expression", 'e', "", "<", "T", " as A>::B", "", false),
    ("expression attributes", 'e', "", "#[a] (", "x", ")", "", false),
    ("statement attributes", 'b', "", "{ #[a] ", "x", " }", "", false),
    ("inner attributes", 'b', "", "{ #![a] ", "x", " }", "", false),
    (
        "higher-ranked where",
        'f',
        "fn f() where for<'a> T: ",
        "A<",
        "B",
        ">",
        " {}",
        false,
    ),
    (
        "higher-ranked bound",
        'f',
        "fn f<T: for<'a> ",
        "A<",
        "B",
        ">",
        ">() {}",
        false,
    ),
    ("parenthesized first bound", 't', "", "(A<", "u8", ">) + B", "", false),
    ("dyn parenthesized bound", 't', "", "dyn (A<", "u8", ">)", "", false),
    ("argument impl", 'f', "fn f(a: ", "impl A<", "u8", ">", ") {}", false),
    (
        "const generic default",
        'f',
        "struct S<const N: usize = ",
        "{ ",
        "1",
        " }",
        ">;",
        false,
    ),
    (
        "associated type bound",
        'f',
        "trait A { type T: ",
        "B<",
        "C",
        ">",
        "; }",
        false,
    ),
    (
        "impl associated type",
        'f',
        "impl A { type T = ",
        "Vec<",
        "u8",
        ">",
        "; }",
        false,
    ),
    ("statement macro", 'b', "", "{ m! { ", "x", " } }", "", true),
    ("higher-ranked closure", 'e', "", "for<'a> |x| ", "x", "", "", false),
    ("closure parameter type", 'e', "|x: ", "Vec<", "u8", ">", "| x", false),
    ("let type", 'b', "{ let x: ", "Vec<", "u8", ">", " = y; }", false),
    ("parenthesized method", 'e', "", "(", "x", ").a()", "", false),
    ("parenthesized cast", 'e', "", "(", "x", " as T) as T", "", false),
    (
        "block continued, cast, and added",
        'e',
        "",
        "{ ",
        "x",
        " }.a() as T + 1",
        "",
        false,
    ),
    (
        "block continued, cast, added, and assigned",
        'e',
        "",
        "{ ",
        "x",
        " }.a() as T + 1 = y",
        "",
        false,
    ),
    (
        "const block parameters",
        'f',
        "",
        "fn f(const { ",
        "",
        " }: u8) {}",
        "",
        false,
    ),
    (
        "const block closure parameters",
        'e',
        "",
        "|const { ",
        "1",
        " }: u8| x",
        "",
        false,
    ),
    ("range pattern bounds", 'p', "", "const { ", "1", " }..=2", "", false),
    (
        "anonymous parameter type",
        'f',
        "trait A { fn f(",
        "(",
        "u8",
        ")",
        "); }",
        false,
    ),
    (
        "anonymous reference parameter",
        'f',
        "trait A { fn f(",
        "&",
        "u8",
        "",
        "); }",
        false,
    ),
];

// A nested construct `n` levels deep
fn nested(case: usize, n: usize) -> String {
    let (_, _, before, open, center, close, after, _) = NESTED[case];
    format!("{before}{}{center}{}{after}", open.repeat(n), close.repeat(n))
}

// The deepest nesting of a construct that the lexer accepts, at most 50,000
// levels: as deep as a source, or a stream from the compiler or proc-macro2,
// can nest it
fn deepest_lexed(case: usize) -> usize {
    let lexes = |n: usize| nested(case, n).parse::<TokenStream>().is_ok();

    if lexes(50_000) {
        return 50_000;
    }

    let (mut low, mut high) = (1, 50_000);

    while high - low > 1 {
        let middle = (low + high) / 2;

        if lexes(middle) {
            low = middle;
        } else {
            high = middle;
        }
    }

    low
}

// Parses, skips, and peeks a source as `T` on a 2 MiB thread, and emits,
// clones, and drops a tree that parses; an overflow aborts the test
fn parse_skip_and_peek<T>(source: String) -> (Result<(), String>, bool, bool)
where
    T: Parse + moxy::token::ToTokenStream + Clone + 'static,
{
    on_small_stack(move || {
        let tokens: TokenStream = source.parse().unwrap();
        let parser = Parser::from_tokens(&tokens);
        let peeked = T::peek(parser.cursor());
        let skipped = T::skip(parser.cursor()).is_some_and(|cursor| cursor.is_empty());
        let parsed = match moxy::parse!(tokens as T) {
            Ok(node) => {
                let _ = node.to_token_stream();
                drop(node.clone());
                Ok(())
            }
            Err(error) => Err(error.message().to_string()),
        };
        (parsed, skipped, peeked)
    })
}

#[test]
fn every_construct_nested_to_the_lexer_limit_reports_the_depth() {
    use moxy::ast::{Expr, File, Pattern, StmtBlock, Type};

    let message = format!("nesting is deeper than {} levels", moxy::token::MAX_DEPTH);

    for (case, &(name, kind, .., unbounded)) in NESTED.iter().enumerate() {
        let run = match kind {
            'e' => parse_skip_and_peek::<Expr>,
            't' => parse_skip_and_peek::<Type>,
            'p' => parse_skip_and_peek::<Pattern>,
            'b' => parse_skip_and_peek::<StmtBlock>,
            _ => parse_skip_and_peek::<File>,
        };

        // Two levels are well formed
        assert_eq!(run(nested(case, 2)), (Ok(()), true, true), "{name} at 2 levels");

        // As deep as the lexer accepts, a construct that nests without groups
        // reports the parser's depth error and fails to skip; one that nests
        // through groups, which the lexer bounds, parses or reports that
        // error. An unbounded construct parses
        let n = deepest_lexed(case);
        let outcome = run(nested(case, n));
        let failed = (Err(message.clone()), false, true);

        if unbounded {
            assert_eq!(outcome, (Ok(()), true, true), "{name} at {n} levels");
        } else if n == 50_000 {
            assert_eq!(outcome, failed, "{name} at {n} levels");
        } else {
            assert!(
                outcome == failed || outcome == (Ok(()), true, true),
                "{name} at {n} levels: {outcome:?}"
            );
        }
    }
}

// The deepest nesting of a construct, at most as deep as the lexer accepts,
// for which `holds` is true, found by bisection
fn deepest_where(case: usize, holds: impl Fn(usize) -> bool) -> usize {
    let lexed = deepest_lexed(case);

    if holds(lexed) {
        return lexed;
    }

    let (mut low, mut high) = (1, lexed);

    while high - low > 1 {
        let middle = (low + high) / 2;

        if holds(middle) {
            low = middle;
        } else {
            high = middle;
        }
    }

    low
}

// Parses a source as `T` on a 2 MiB thread and runs every traversal on it
fn traverse_parsed<T>(source: String)
where
    T: Parse + moxy::token::ToTokenStream + moxy::token::Spanner + Clone + PartialEq + std::fmt::Debug,
    T: moxy::fmt::Format + serde::Serialize + Walked + 'static,
{
    on_small_stack(move || traverse(moxy::parse!(source as T).unwrap()));
}

#[test]
fn every_construct_skips_to_the_depth_that_it_parses_to() {
    use moxy::ast::{Expr, File, Pattern, StmtBlock, Type};

    let mismatched: Vec<_> = (0..NESTED.len())
        .filter_map(|case| {
            let (name, kind, ..) = NESTED[case];
            let run = |n: usize| match kind {
                'e' => parse_skip_and_peek::<Expr>(nested(case, n)),
                't' => parse_skip_and_peek::<Type>(nested(case, n)),
                'p' => parse_skip_and_peek::<Pattern>(nested(case, n)),
                'b' => parse_skip_and_peek::<StmtBlock>(nested(case, n)),
                _ => parse_skip_and_peek::<File>(nested(case, n)),
            };
            let parsed = deepest_where(case, |n| run(n).0.is_ok());
            let skipped = deepest_where(case, |n| run(n).1);

            // Every traversal runs on the deepest tree that parses
            let source = nested(case, parsed);
            match kind {
                'e' => traverse_parsed::<Expr>(source),
                't' => traverse_parsed::<Type>(source),
                'p' => traverse_parsed::<Pattern>(source),
                'b' => traverse_parsed::<StmtBlock>(source),
                _ => traverse_parsed::<File>(source),
            }

            (parsed != skipped).then_some((name, parsed, skipped))
        })
        .collect();
    assert!(
        mismatched.is_empty(),
        "(construct, deepest parse, deepest skip): {mismatched:?}"
    );
}

#[test]
fn the_lexer_and_the_parser_report_the_depth_apart() {
    let depth = moxy::token::MAX_DEPTH;
    let lexer = on_small_stack(move || {
        format!("{}{}", "[".repeat(depth + 1), "]".repeat(depth + 1))
            .parse::<TokenStream>()
            .unwrap_err()
            .to_string()
    });
    assert_eq!(lexer, format!("delimiters nest deeper than {depth} levels"));

    let parser = moxy::parse!((format!("{}a", "!".repeat(depth + 1))) as moxy::ast::Expr)
        .err()
        .unwrap();
    assert_eq!(parser.message(), format!("nesting is deeper than {depth} levels"));
}

#[test]
fn input_nested_to_the_depth_limit_parses_on_a_small_stack() {
    let depth = moxy::token::MAX_DEPTH - 1;

    let parsed = on_small_stack(move || {
        let parens = format!("{}a{}", "(".repeat(depth), ")".repeat(depth));
        // The arms of a match take a level of their own
        let arms = format!("{}a{}", "match a { _ => ".repeat(depth - 2), " }".repeat(depth - 2));
        // A closure parameter is a pattern, which keeps one level of room
        let closures = format!("{}a", "|x| ".repeat(depth - 1));
        let generics = format!("{}u8{}", "Vec<".repeat(depth), ">".repeat(depth));
        // A pattern keeps one level of room for the peeks that choose its
        // form
        let patterns = format!("{}a{}", "(".repeat(depth - 1), ")".repeat(depth - 1));
        // A module's item and its body each take a level
        let modules = format!("{}{}", "mod m { ".repeat(depth / 2), "}".repeat(depth / 2));
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
    assert_eq!(error.message(), "expected pattern, found `=`");

    let source = "fn f() {}\nstruct S { a: }";
    let error = moxy::parse!(source as moxy::ast::File).err().unwrap();
    assert_eq!(error.span().start().line(), 1);
    assert_eq!(error.message(), "expected type, found `}`");
}

// Runs `parse` on a worker thread and fails if it takes longer than a limit
// no linear parse comes near, so an exponential one fails instead of hanging
fn within_limit(label: &str, parse: impl FnOnce() -> bool + Send + 'static) {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .stack_size(64 << 20)
        .spawn(move || sender.send(parse()).unwrap())
        .unwrap();
    let parsed = receiver
        .recv_timeout(std::time::Duration::from_secs(10))
        .unwrap_or_else(|_| panic!("{label} took longer than 10 seconds"));
    assert!(parsed, "{label} did not parse");
}

#[test]
fn nested_parentheses_in_types_and_block_chains_parse_in_linear_time() {
    let depth = 64;
    let parens = |inner: &str| format!("{}{inner}{}", "(".repeat(depth), ")".repeat(depth));
    let tuples = format!("{}T{}", "(".repeat(depth), ",)".repeat(depth));
    let qualified = format!("A::<{}>::B", parens("T"));
    let chain = format!("{}x{}", "{ ".repeat(depth), " }.a()".repeat(depth));

    for (label, source) in [
        ("parameter type", format!("fn f(a: {}) {{}}", parens("T"))),
        ("tuple parameter type", format!("fn f(a: {tuples}) {{}}")),
        ("receiver type", format!("impl A {{ fn f(self: {}) {{}} }}", parens("T"))),
        ("trait function", format!("trait A {{ fn f(a: {}); }}", parens("T"))),
        ("foreign function", format!("extern \"C\" {{ fn f(a: {}); }}", parens("T"))),
        ("pattern path", format!("fn f() {{ let {qualified} = x; }}")),
        ("block chain", format!("fn f() {{ {chain}; }}")),
    ] {
        within_limit(label, move || moxy::parse!(source as moxy::ast::File).is_ok());
    }
}

#[test]
fn block_like_statements_at_the_depth_limit_skip_in_linear_time() {
    use moxy::ast::{Expr, StmtBlock};

    // A block-like statement that failed to skip at the limit was skipped
    // again as an operand at every enclosing level
    let depth = moxy::token::MAX_DEPTH;

    for open in ["if a { ", "if a {} else { ", "while a { ", "loop { ", "unsafe { ", "'a: { "] {
        let source = format!("{}x{}", open.repeat(depth), " }".repeat(depth));
        within_limit(open, move || {
            let tokens: TokenStream = source.parse().unwrap();
            Expr::skip(Parser::from_tokens(&tokens).cursor()).is_none()
        });
    }

    let source = format!("{{ {}x{} }}", "if a { ".repeat(depth - 1), " }".repeat(depth - 1));
    within_limit("ifs in a block", move || {
        let tokens: TokenStream = source.parse().unwrap();
        StmtBlock::skip(Parser::from_tokens(&tokens).cursor()).is_none()
    });
}

#[test]
fn parameters_with_const_block_patterns_parse_in_linear_time() {
    // Peeking each parameter for a variadic read its pattern, which held
    // functions with such parameters, so each level doubled the work: 14
    // levels took 10 ms
    let depth = 28;
    let source = format!("{}{}", "fn f(const { ".repeat(depth), " }: u8) {}".repeat(depth));
    within_limit("const block parameters", move || {
        moxy::parse!(source as moxy::ast::File).is_ok()
    });

    let source = format!("{}1{}", "|const { ".repeat(depth), " }: u8| x".repeat(depth));
    within_limit("const block closure parameters", move || {
        moxy::parse!(source as moxy::ast::Expr).is_ok()
    });
}

#[test]
fn many_let_statements_parse_in_linear_time() {
    // Peeking for an or-pattern read to the end of the block at every `let`
    let source = format!("fn f() {{ {} }}", "let a = 1; ".repeat(20_000));
    within_limit("let statements", move || moxy::parse!(source as moxy::ast::File).is_ok());

    let pattern: moxy::ast::Pattern = moxy::parse!("a | b").unwrap();
    assert!(matches!(pattern, moxy::ast::Pattern::Or(_)));
}

// Parses a source on a 2 MiB thread and, when it parses, emits, spans, clones,
// and drops the tree; deep input must give the depth error instead of
// overflowing the stack, which would abort the test
fn traverse_on_small_stack<T>(source: String) -> Option<String>
where
    T: moxy::ast::Parse + moxy::token::ToTokenStream + moxy::token::Spanner + Clone,
{
    on_small_stack(move || match moxy::parse!(source as T) {
        Ok(node) => {
            let _ = node.to_token_stream();
            let _ = node.span();
            drop(node.clone());
            drop(node);
            None
        }
        Err(error) => Some(error.message().to_string()),
    })
}

// Every traversal a user can run on a tree: emitting, spanning, cloning,
// comparing, debug printing, visiting, formatting, serializing, and dropping.
// AST nodes do not implement Hash.
fn traverse<T>(node: T)
where
    T: moxy::token::ToTokenStream + moxy::token::Spanner + Clone + PartialEq + std::fmt::Debug,
    T: moxy::fmt::Format + serde::Serialize + Walked,
{
    assert!(!node.nests_past_max_depth());
    let _ = node.to_token_stream();
    let _ = node.span();
    let mut copy = node.clone();
    assert!(copy == node);
    let _ = format!("{node:?}");
    node.walk();
    copy.walk_mut();
    moxy::fmt!(&node).unwrap();
    let _ = serde_json::to_string(&node).unwrap();
    drop(copy);
    drop(node);
}

struct Walk;

impl moxy::ast::VisitMut for Walk {}

// A tree that both visitors walk from its root, and that the depth check
// measures
trait Walked {
    fn walk(&self);
    fn walk_mut(&mut self);
    fn nests_past_max_depth(&self) -> bool;
}

macro_rules! walked {
    ($($ty:ty => $visit:ident, $visit_mut:ident;)*) => {
        $(impl Walked for $ty {
            fn walk(&self) {
                moxy::ast::Visit::$visit(&mut Walk, self);
            }

            fn walk_mut(&mut self) {
                moxy::ast::VisitMut::$visit_mut(&mut Walk, self);
            }

            fn nests_past_max_depth(&self) -> bool {
                <$ty>::nests_past_max_depth(self)
            }
        })*
    };
}

walked! {
    moxy::ast::Expr => visit_expr, visit_expr_mut;
    moxy::ast::Type => visit_type, visit_type_mut;
    moxy::ast::Pattern => visit_pattern, visit_pattern_mut;
    moxy::ast::StmtBlock => visit_stmt_block, visit_stmt_block_mut;
    moxy::ast::Stmt => visit_stmt, visit_stmt_mut;
}

impl Walked for moxy::ast::File {
    fn walk(&self) {
        self.items
            .iter()
            .for_each(|item| moxy::ast::Visit::visit_item(&mut Walk, item));
    }

    fn walk_mut(&mut self) {
        self.items
            .iter_mut()
            .for_each(|item| moxy::ast::VisitMut::visit_item_mut(&mut Walk, item));
    }

    fn nests_past_max_depth(&self) -> bool {
        moxy::ast::File::nests_past_max_depth(self)
    }
}

impl moxy::ast::Visit<'_> for Walk {}

// The deepest nesting of a kind that parses on a 2 MiB thread, found by
// bisection, with every traversal run on the tree at that depth; past it
// the parse reports the depth error
fn deepest<T>(make: fn(usize) -> String) -> usize
where
    T: moxy::ast::Parse + moxy::token::ToTokenStream + moxy::token::Spanner + Clone + PartialEq + std::fmt::Debug,
    T: moxy::fmt::Format + serde::Serialize + Walked + 'static,
{
    let message = format!("nesting is deeper than {} levels", moxy::token::MAX_DEPTH);
    let parses = move |n: usize| {
        let source = make(n);
        let message = message.clone();

        // Deeper than the lexer accepts: the parser never sees it
        if source.parse::<TokenStream>().is_err() {
            return false;
        }

        on_small_stack(move || match moxy::parse!(source as T) {
            Ok(node) => {
                traverse(node);
                true
            }
            Err(error) => {
                assert_eq!(error.message(), message, "at {n} levels");
                false
            }
        })
    };

    let (mut low, mut high) = (1, 400);
    assert!(!parses(high), "400 levels parsed");

    while high - low > 1 {
        let middle = (low + high) / 2;

        if parses(middle) {
            low = middle;
        } else {
            high = middle;
        }
    }

    low
}

#[test]
fn deep_trees_of_every_kind_parse_and_traverse_or_report_the_depth() {
    use moxy::ast::{Expr, File, Pattern, Type};

    use moxy::token::MAX_DEPTH;

    // Each kind with the deepest nesting that the depth count accepts, which
    // does not depend on the machine
    let kinds = [
        (
            "nested blocks",
            MAX_DEPTH - 2,
            deepest::<Expr>(|n| format!("{}x{}", "{ ".repeat(n), " }".repeat(n))),
        ),
        (
            "modules",
            MAX_DEPTH / 2,
            deepest::<File>(|n| format!("{}{}", "mod m { ".repeat(n), "}".repeat(n))),
        ),
        (
            "let initializers in blocks",
            (MAX_DEPTH - 1) / 3,
            deepest::<Expr>(|n| format!("{}1{}", "{ let a = ".repeat(n), "; a }".repeat(n))),
        ),
        (
            "const blocks in let patterns",
            (MAX_DEPTH - 2) / 3,
            deepest::<File>(|n| format!("fn f() {{ {}1{} }}", "let const { ".repeat(n), " } = 1;".repeat(n))),
        ),
        (
            "method call arguments",
            (MAX_DEPTH - 2) / 2,
            deepest::<Expr>(|n| format!("{}x{}", "a.b(".repeat(n), ")".repeat(n))),
        ),
        (
            "nested ifs in a block",
            MAX_DEPTH - 2,
            deepest::<moxy::ast::StmtBlock>(|n| format!("{{ {}x{} }}", "if a { ".repeat(n), " }".repeat(n))),
        ),
        (
            "operator ladders in parentheses",
            (MAX_DEPTH - 1) / 9,
            deepest::<Expr>(|n| format!("{}0{}", "(0||0&&0==0|0^0&0<<0+0*".repeat(n), ")".repeat(n))),
        ),
        (
            "left operator ladders in parentheses",
            (MAX_DEPTH - 1) / 9,
            deepest::<Expr>(|n| format!("{}0{}", "(".repeat(n), ")*0+0<<0&0^0|0==0&&0||0".repeat(n))),
        ),
        (
            "assignments after ranges in parentheses",
            (MAX_DEPTH - 1) / 2,
            deepest::<Expr>(|n| format!("{}a{}", "(".repeat(n), ") = 0.. = 0".repeat(n))),
        ),
        (
            "associated bindings nested through their arguments",
            MAX_DEPTH - 3,
            deepest::<Type>(|n| format!("X<{}C{}>", "B<".repeat(n), "> = u8".repeat(n))),
        ),
        (
            "associated type bounds nested through their arguments",
            MAX_DEPTH - 3,
            deepest::<Type>(|n| format!("X<{}C{}>", "B<".repeat(n), ">: D".repeat(n))),
        ),
        (
            "operator ladders in closures",
            (MAX_DEPTH - 1) / 9,
            deepest::<Expr>(|n| format!("{}0", "|| 0||0&&0==0|0^0&0<<0+0*".repeat(n))),
        ),
        (
            "match arms",
            MAX_DEPTH - 3,
            deepest::<Expr>(|n| format!("{}x{}", "match a { _ => ".repeat(n), " }".repeat(n))),
        ),
        (
            "match guards",
            (MAX_DEPTH - 2) / 2,
            deepest::<Expr>(|n| format!("{}x{}", "match a { _ if ".repeat(n), " => {} }".repeat(n))),
        ),
        (
            "parentheses",
            MAX_DEPTH - 1,
            deepest::<Expr>(|n| format!("{}a{}", "(".repeat(n), ")".repeat(n))),
        ),
        (
            "struct literals",
            (MAX_DEPTH - 1) / 2,
            deepest::<Expr>(|n| format!("{}a{}", "S { a: ".repeat(n), " }".repeat(n))),
        ),
        (
            "closures with a return type",
            (MAX_DEPTH - 1) / 3,
            deepest::<Expr>(|n| format!("{}1{}", "|| -> T { ".repeat(n), " }".repeat(n))),
        ),
        (
            "blocks with a method call",
            (MAX_DEPTH - 2) / 2,
            deepest::<Expr>(|n| format!("{}x{}", "{ ".repeat(n), " }.a()".repeat(n))),
        ),
        (
            "blocks continued by a method call and an operator",
            (MAX_DEPTH - 2) / 2,
            deepest::<Expr>(|n| format!("{}x{}", "{ ".repeat(n), " }.a() + 1".repeat(n))),
        ),
        (
            "matches continued by a method call",
            (MAX_DEPTH - 3) / 2,
            deepest::<Expr>(|n| format!("{}x{}", "match a { _ => ".repeat(n), " }.a()".repeat(n))),
        ),
        (
            "calls continued, cast, and added",
            (MAX_DEPTH - 1) / 3,
            deepest::<Expr>(|n| format!("{}x{}", "f(".repeat(n), ").a()? as T + 1".repeat(n))),
        ),
        (
            "parentheses continued and added",
            (MAX_DEPTH - 1) / 2,
            deepest::<Expr>(|n| format!("{}x{}", "(".repeat(n), ").a() + 1".repeat(n))),
        ),
        (
            "qualified types",
            (MAX_DEPTH - 1) / 2,
            deepest::<Type>(|n| format!("{}T{}", "<".repeat(n), " as A>::B".repeat(n))),
        ),
        (
            "function sugar",
            (MAX_DEPTH - 1) / 2,
            deepest::<Type>(|n| format!("{}T", "Fn() -> ".repeat(n))),
        ),
        (
            "impl function sugar",
            (MAX_DEPTH - 1) / 3,
            deepest::<Type>(|n| format!("{}T", "impl Fn() -> ".repeat(n))),
        ),
        (
            "associated types",
            (MAX_DEPTH - 1) / 3,
            deepest::<Type>(|n| format!("{}u8{}", "A<B = ".repeat(n), ">".repeat(n))),
        ),
        (
            "associated bounds",
            (MAX_DEPTH - 1) / 3,
            deepest::<Type>(|n| format!("{}C{}", "A<B: ".repeat(n), ">".repeat(n))),
        ),
        (
            "function pointers",
            (MAX_DEPTH - 1) / 2,
            deepest::<Type>(|n| format!("{}u8{}", "fn(".repeat(n), ")".repeat(n))),
        ),
        (
            "array lengths",
            (MAX_DEPTH - 1) / 4,
            deepest::<Type>(|n| format!("{}u8{}", "[u8; { let x: ".repeat(n), "; 1 }]".repeat(n))),
        ),
        (
            "generic arguments",
            MAX_DEPTH - 1,
            deepest::<Type>(|n| format!("{}u8{}", "Vec<".repeat(n), ">".repeat(n))),
        ),
        (
            "struct patterns",
            (MAX_DEPTH - 1) / 2,
            deepest::<Pattern>(|n| format!("{}x{}", "S { a: ".repeat(n), " }".repeat(n))),
        ),
        (
            "impls",
            (MAX_DEPTH + 1) / 5,
            deepest::<File>(|n| format!("{}{}", "impl A { fn f() { ".repeat(n), "} }".repeat(n))),
        ),
        (
            "traits",
            (MAX_DEPTH + 1) / 5,
            deepest::<File>(|n| format!("{}{}", "trait A { fn f() { ".repeat(n), "} }".repeat(n))),
        ),
        (
            "functions",
            MAX_DEPTH / 3,
            deepest::<File>(|n| format!("{}{}", "fn f() { ".repeat(n), "}".repeat(n))),
        ),
    ];

    let mismatched: Vec<_> = kinds.iter().filter(|(_, expected, reached)| reached != expected).collect();
    assert!(mismatched.is_empty(), "(kind, expected, reached): {mismatched:?}");
}

#[test]
fn a_pattern_at_the_depth_limit_reports_the_depth() {
    use moxy::token::MAX_DEPTH;

    // A pattern keeps two levels of room for the peeks that choose its form
    let message = format!("nesting is deeper than {MAX_DEPTH} levels");
    let source = |depth: usize| format!("{}1..=2{}", "(".repeat(depth), ")".repeat(depth));
    let results =
        [MAX_DEPTH - 2, MAX_DEPTH - 1, MAX_DEPTH].map(|depth| traverse_on_small_stack::<moxy::ast::Pattern>(source(depth)));
    assert_eq!(results, [None, Some(message.clone()), Some(message)]);
}

#[test]
fn nested_ranges_without_a_start_count_against_the_depth() {
    let message = format!("nesting is deeper than {} levels", moxy::token::MAX_DEPTH);
    let source = format!("{}a", ".. ".repeat(200_000));
    assert_eq!(traverse_on_small_stack::<moxy::ast::Expr>(source), Some(message));
    assert_eq!(
        traverse_on_small_stack::<moxy::ast::Expr>(format!("{}a", ".. ".repeat(100))),
        None
    );
}

#[test]
fn assignments_after_ranges_count_against_the_depth() {
    let message = format!("nesting is deeper than {} levels", moxy::token::MAX_DEPTH);
    let source = format!("a{}", " = b..".repeat(100_000));
    assert_eq!(traverse_on_small_stack::<moxy::ast::Expr>(source.clone()), Some(message));

    // Skipping holds each assignment's level as parsing does
    let skipped = on_small_stack(move || {
        let tokens: TokenStream = source.parse().unwrap();
        <moxy::ast::Expr as Parse>::skip(Parser::from_tokens(&tokens).cursor()).is_some()
    });
    assert!(!skipped);
    assert_eq!(
        traverse_on_small_stack::<moxy::ast::Expr>(format!("a{}", " = b..".repeat(50))),
        None
    );
}

#[test]
fn blocks_count_against_the_same_depth_as_other_nesting() {
    // Blocks counted only against the lexer's own limit, so blocks and
    // closures together nested past the stack
    let message = format!("nesting is deeper than {} levels", moxy::token::MAX_DEPTH);
    let source = format!("{}{}a{}", "if a { ".repeat(140), "|x| ".repeat(141), " }".repeat(140));
    assert_eq!(traverse_on_small_stack::<moxy::ast::Expr>(source), Some(message));

    let source = format!("{}{}a{}", "if a { ".repeat(60), "|x| ".repeat(60), " }".repeat(60));
    assert_eq!(traverse_on_small_stack::<moxy::ast::Expr>(source), None);
}

#[test]
fn patterns_with_blocks_in_their_bounds_and_paths_parse_in_linear_time() {
    // A range peek skipped a bound's block and each path form skipped the
    // path again, which doubled the work per level
    let depth = 32;
    let consts = format!("fn f() {{ {}1{} }}", "let const { ".repeat(depth), " } = 1;".repeat(depth));
    let turbofish = format!(
        "fn f() {{ let A::<{}0{}>::B = 1; }}",
        "{ let A::<".repeat(depth),
        ">::B = 1; 0 }".repeat(depth)
    );
    let ranges = format!(
        "fn f() {{ match x {{ {}0{} => 1 }} }}",
        "A::<{ match x { 1..=2 => ".repeat(8),
        " } }>::B..=C".repeat(8)
    );

    for (label, source) in [
        ("const patterns", consts),
        ("turbofish patterns", turbofish),
        ("ranges", ranges),
    ] {
        within_limit(label, move || moxy::parse!(source as moxy::ast::File).is_ok());
    }

    for source in [
        "1..=2",
        "-1..",
        "A::B..=C",
        "<T>::A..",
        "const { 1 }..=2",
        "a..=b",
        "..=b",
        "..b",
    ] {
        let pattern: moxy::ast::Pattern = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert!(matches!(pattern, moxy::ast::Pattern::Range(_)), "`{source}`");
    }

    for source in ["ref a..=b", "S(x)..=b", "m!().."] {
        assert!(moxy::parse!(source as moxy::ast::Pattern).is_err(), "`{source}`");
    }
}

#[test]
fn receivers_and_trait_aliases_with_blocks_in_their_types_parse_in_linear_time() {
    // The receiver peek skipped the receiver's type and the alias peek the
    // trait's generics before each parsed them again
    let depth = 18;
    let receivers = format!(
        "{}u8{}",
        "impl A { fn f(self: [u8; { ".repeat(depth),
        "; 1 }]) {} }".repeat(depth)
    );
    let aliases = format!("{}u8{}", "trait A<T = [u8; { ".repeat(depth), "; 1 }]> = B;".repeat(depth));

    for (label, source) in [("receivers", receivers), ("trait aliases", aliases)] {
        within_limit(label, move || moxy::parse!(source as moxy::ast::File).is_ok());
    }

    let item: moxy::ast::Item = moxy::parse!("trait A<T> = B<T> + C;").unwrap();
    assert!(item.is_trait_alias());
    let item: moxy::ast::Item = moxy::parse!("unsafe trait A<T>: B {}").unwrap();
    assert!(item.is_trait());
}

#[test]
fn chains_of_any_length_parse_and_traverse_on_a_small_stack() {
    use moxy::ast::{Expr, File, VisitMut};
    use moxy::token::{ToTokenStream, TokenStream};

    struct WalkMut;

    impl VisitMut for WalkMut {}

    // The steps of the one chain at the top of each tree
    fn steps(expression: &Expr) -> usize {
        match expression {
            Expr::BinaryChain(chain) => chain.operands.len() - 1,
            Expr::Postfix(chain) => chain.ops.len(),
            Expr::CastChain(chain) => chain.casts.len(),
            Expr::IfChain(chain) => chain.else_ifs.len(),
            _ => 0,
        }
    }

    let steps_wanted = 100_000;

    for (source, wanted) in [
        (format!("a{}", " + a".repeat(steps_wanted)), steps_wanted),
        (format!("a{}", ".b()".repeat(steps_wanted)), steps_wanted),
        (format!("a{}", "?".repeat(steps_wanted)), steps_wanted),
        (format!("a{}", " as T".repeat(steps_wanted)), steps_wanted),
        (
            format!("if a {{}}{} else {{}}", " else if a {}".repeat(steps_wanted)),
            steps_wanted,
        ),
        // A sum of method chains that end in casts
        (format!("a{}", " + a.b()? as T".repeat(steps_wanted / 4)), steps_wanted / 4),
    ] {
        let checked = on_small_stack(move || {
            let lexed: TokenStream = source.parse().unwrap();
            let mut node: Expr = moxy::parse!(source).unwrap();
            assert_eq!(steps(&node), wanted);
            assert_eq!(node.to_token_stream().to_string(), lexed.to_string());
            WalkMut.visit_expr_mut(&mut node);
            let formatted = moxy::fmt!(&node).unwrap();
            let again: Expr = moxy::parse!(formatted).unwrap();
            assert!(again == node);
            traverse(node);
            true
        });
        assert!(checked);
    }

    let source = format!("use {}a;", "a::".repeat(steps_wanted));
    let checked = on_small_stack(move || {
        let lexed: TokenStream = source.parse().unwrap();
        let node: File = moxy::parse!(source).unwrap();
        let path = node.items[0].as_use().unwrap().tree.as_path_chain().unwrap();
        assert_eq!(path.segments.len(), steps_wanted);
        assert_eq!(node.to_token_stream().to_string(), lexed.to_string());
        traverse(node);
        true
    });
    assert!(checked);
}

#[test]
fn a_chain_of_atoms_costs_no_level_of_its_own() {
    // `(1 + 1)` costs what its parentheses cost; a chain nests a level only
    // inside an operand of a weaker chain
    let parens = |n: usize| format!("{}1{}", "(1 + ".repeat(n), ")".repeat(n));
    let depth = moxy::token::MAX_DEPTH - 1;
    let message = format!("nesting is deeper than {} levels", moxy::token::MAX_DEPTH);
    let results = [depth, depth + 1].map(|n| traverse_on_small_stack::<moxy::ast::Expr>(parens(n)));
    assert_eq!(results, [None, Some(message)]);
}

// A proc-macro2 stream of `n` nested groups
#[cfg(feature = "proc-macro2")]
fn nested_proc_macro2_groups(n: usize) -> proc_macro2::TokenStream {
    use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};

    (0..n).fold(TokenStream::new(), |stream, _| {
        TokenTree::Group(Group::new(Delimiter::Brace, stream)).into()
    })
}

#[cfg(feature = "proc-macro2")]
#[test]
fn a_token_stream_nested_past_the_depth_limit_is_refused_when_converted() {
    use moxy::token::TryIntoTokenStream;

    // Streams from the compiler or proc-macro2 are not lexed, so converting
    // one counts its groups as the lexer does, and no deeper stream exists for
    // the traversals that recurse per group
    let depth = moxy::token::MAX_DEPTH;
    let (refused, parsed, converted) = on_small_stack(move || {
        let refused = nested_proc_macro2_groups(50_000)
            .try_into_token_stream()
            .unwrap_err()
            .to_string();
        let parsed = moxy::parse!((nested_proc_macro2_groups(depth + 1)) as moxy::ast::StmtBlock)
            .err()
            .map(|error| error.message().to_string());
        let converted = nested_proc_macro2_groups(depth).try_into_token_stream().is_ok();
        (refused, parsed, converted)
    });
    let message = format!("delimiters nest deeper than {depth} levels");
    assert_eq!(
        (refused.as_str(), parsed.as_deref(), converted),
        (message.as_str(), Some(message.as_str()), true)
    );
}

#[cfg(feature = "proc-macro2")]
#[test]
fn converting_tokens_past_the_depth_limit_without_an_error_panics() {
    let depth = moxy::token::MAX_DEPTH;
    let message = |convert: fn(proc_macro2::TokenStream)| {
        std::panic::catch_unwind(|| convert(nested_proc_macro2_groups(depth + 1)))
            .err()
            .and_then(|payload| payload.downcast::<String>().ok())
            .map(|message| *message)
    };
    let stream: fn(proc_macro2::TokenStream) = |tokens| drop(moxy::token::TokenStream::from(tokens));
    let group: fn(proc_macro2::TokenStream) = |tokens| {
        let Some(proc_macro2::TokenTree::Group(group)) = tokens.into_iter().next() else {
            unreachable!()
        };
        drop(moxy::token::Group::from(group));
    };
    let expected = format!(
        "delimiters nest deeper than {depth} levels; convert a token stream that holds the tokens with `try_into_token_stream` to get this as an error"
    );
    assert_eq!([message(stream), message(group)], [Some(expected.clone()), Some(expected)]);
}

#[test]
fn an_error_quotes_a_group_by_its_delimiters() {
    // A message that printed the whole group printed every macro body it met
    let error = moxy::parse!("{ a b c }" as moxy::token::Ident).err().unwrap();
    assert_eq!(error.message(), "expected identifier, found `{ ... }`");

    let error = moxy::parse!("()" as moxy::token::Ident).err().unwrap();
    assert_eq!(error.message(), "expected identifier, found `()`");
}

#[test]
fn parsing_a_token_stream_borrows_it() {
    use moxy::token::TryIntoTokenStream;

    // A borrowed stream reaches the parser as itself, not as a copy
    let tokens: moxy::token::TokenStream = "a + b".parse().unwrap();
    let address = (&tokens).with_token_stream(std::ptr::from_ref).unwrap();
    assert_eq!(address, std::ptr::from_ref(&tokens));

    let expression: moxy::ast::Expr = moxy::parse!(tokens).unwrap();
    assert!(expression.is_binary_chain());
}

// A tree built in code: `seed` wrapped `n` times in a copy of the form of
// `template`, whose child `place` returns, built without recursion
fn built<T: Parse + Clone>(template: &str, seed: &str, n: usize, place: fn(&mut T) -> &mut T) -> T {
    let template: T = moxy::parse!(template).unwrap();
    let seed: T = moxy::parse!(seed).unwrap();

    (0..n).fold(seed, |node, _| {
        let mut outer = template.clone();
        *place(&mut outer) = node;
        outer
    })
}

fn negated(expr: &mut moxy::ast::Expr) -> &mut moxy::ast::Expr {
    let moxy::ast::Expr::Unary(unary) = expr else { panic!() };
    &mut unary.expr
}

fn referenced(ty: &mut moxy::ast::Type) -> &mut moxy::ast::Type {
    let moxy::ast::Type::Reference(reference) = ty else { panic!() };
    &mut reference.elem
}

fn referenced_pattern(pattern: &mut moxy::ast::Pattern) -> &mut moxy::ast::Pattern {
    let moxy::ast::Pattern::Reference(reference) = pattern else {
        panic!()
    };
    &mut reference.pat
}

fn inner_block(block: &mut moxy::ast::StmtBlock) -> &mut moxy::ast::StmtBlock {
    let moxy::ast::Stmt::Block(inner) = &mut block.stmts.inner[0] else {
        panic!()
    };
    inner
}

#[test]
fn a_tree_built_past_the_depth_limit_is_reported() {
    use moxy::ast::{Expr, Pattern, StmtBlock, Type};

    let message = format!("the tree nests more than {} nodes deep", moxy::token::MAX_NESTED_NODES);
    let (reported, formatted) = on_small_stack(move || {
        let n = 100_000;
        let expr = built::<Expr>("-x", "x", n, negated);
        let ty = built::<Type>("&u8", "u8", n, referenced);
        let pattern = built::<Pattern>("&x", "x", n, referenced_pattern);
        let block = built::<StmtBlock>("{ {} }", "{ x }", n, inner_block);
        let reported = [
            expr.nests_past_max_depth(),
            ty.nests_past_max_depth(),
            pattern.nests_past_max_depth(),
            block.nests_past_max_depth(),
        ];
        let formatted = moxy::fmt!(&expr).map_err(|error| error.to_string());

        // Dropping them would recurse once per level
        std::mem::forget((expr, ty, pattern, block));
        (reported, formatted)
    });

    assert_eq!(reported, [true; 4]);
    assert_eq!(formatted, Err(message));
}

// A tree counts half a level for each expression, type, pattern, item, block,
// and token group nested in another, so it holds twice `MAX_DEPTH` of them at
// the limit, and every traversal of it runs on a small stack
#[test]
fn a_tree_built_to_the_depth_limit_runs_every_traversal() {
    use moxy::ast::{Expr, Pattern, StmtBlock, Type};

    let nodes = 2 * moxy::token::MAX_DEPTH;
    let limits = on_small_stack(move || {
        // The seed is one node, or two for a block that holds an expression
        let expr = |n| built::<Expr>("-x", "x", n, negated);
        let ty = |n| built::<Type>("&u8", "u8", n, referenced);
        let pattern = |n| built::<Pattern>("&x", "x", n, referenced_pattern);
        let block = |n| built::<StmtBlock>("{ {} }", "{ x }", n, inner_block);
        let limits = [
            (expr(nodes - 1).nests_past_max_depth(), expr(nodes).nests_past_max_depth()),
            (ty(nodes - 1).nests_past_max_depth(), ty(nodes).nests_past_max_depth()),
            (
                pattern(nodes - 1).nests_past_max_depth(),
                pattern(nodes).nests_past_max_depth(),
            ),
            (
                block(nodes - 2).nests_past_max_depth(),
                block(nodes - 1).nests_past_max_depth(),
            ),
        ];

        // Formatting counts the same nodes, so one past the limit fails
        let formatted = [
            moxy::fmt!(&expr(nodes)).is_err(),
            moxy::fmt!(&ty(nodes)).is_err(),
            moxy::fmt!(&pattern(nodes)).is_err(),
            moxy::fmt!(&block(nodes - 1)).is_err(),
        ];

        traverse(expr(nodes - 1));
        traverse(ty(nodes - 1));
        traverse(pattern(nodes - 1));
        traverse(block(nodes - 2));
        (limits, formatted)
    });

    assert_eq!(limits, ([(false, true); 4], [true; 4]));
}

#[test]
fn a_token_stream_built_past_the_depth_limit_is_reported() {
    use moxy::token::Group;

    let depth = moxy::token::MAX_DEPTH;
    let lexed: TokenStream = "[[x]]".parse().unwrap();
    let built = on_small_stack(move || {
        let nested = |n: usize| {
            (0..n).fold(TokenStream::new(), |stream, _| {
                TokenStream::from_iter([TokenTree::Group(Group::new(Delim::Bracket, stream))])
            })
        };

        [
            nested(depth).nests_deeper_than(depth),
            nested(depth + 1).nests_deeper_than(depth),
            nested(100_000).nests_deeper_than(depth),
        ]
    });

    assert_eq!((lexed.nests_deeper_than(1), lexed.nests_deeper_than(2)), (true, false));
    assert_eq!(built, [false, true, true]);
}

fn grouped(tree: &mut moxy::ast::UseTree) -> &mut moxy::ast::UseTree {
    let moxy::ast::UseTree::PathChain(chain) = tree else {
        panic!()
    };
    let moxy::ast::UseTree::Group(group) = &mut *chain.tree else {
        panic!()
    };
    group.items.inner.first_mut().unwrap()
}

fn bounded(path: &mut moxy::ast::Path) -> &mut moxy::ast::Path {
    let moxy::ast::PathArguments::AngleBracketed(args) = &mut path[0].args else {
        panic!()
    };
    let Some(moxy::ast::GenericArgument::Constraint(constraint)) = args.args.first_mut() else {
        panic!()
    };
    let Some(moxy::ast::TypeBound::Trait(bound)) = constraint.bounds.first_mut() else {
        panic!()
    };
    &mut bound.path
}

fn chained(tree: &mut moxy::ast::UseTree) -> &mut moxy::ast::UseTree {
    let moxy::ast::UseTree::PathChain(chain) = tree else {
        panic!()
    };
    &mut chain.tree
}

fn bound_args(args: &mut moxy::ast::PathArguments) -> &mut moxy::ast::PathArguments {
    let moxy::ast::PathArguments::AngleBracketed(angle) = args else {
        panic!()
    };
    let Some(moxy::ast::GenericArgument::AssocType(binding)) = angle.args.first_mut() else {
        panic!()
    };
    &mut binding.args
}

// A file of one use item whose tree is built `n` deep
fn built_use(n: usize, place: fn(&mut moxy::ast::UseTree) -> &mut moxy::ast::UseTree, template: &str) -> moxy::ast::File {
    let mut file: moxy::ast::File = moxy::parse!("use x;").unwrap();
    let moxy::ast::Item::Use(use_item) = &mut file.items[0] else {
        panic!()
    };
    use_item.tree = built(template, "b", n, place);
    file
}

// A type whose path is built `n` deep through associated type bounds, or
// through the arguments of associated bindings
fn built_bounds(n: usize) -> moxy::ast::Type {
    let mut ty: moxy::ast::Type = moxy::parse!("X").unwrap();
    let moxy::ast::Type::Path(path) = &mut ty else { panic!() };
    path.path = built("A<B: C>", "C", n, bounded);
    ty
}

fn built_bindings(n: usize) -> moxy::ast::Type {
    let mut ty: moxy::ast::Type = moxy::parse!("X").unwrap();
    let moxy::ast::Type::Path(path) = &mut ty else { panic!() };
    path.path[0].args = built("<B<C> = u8>", "<C>", n, bound_args);
    ty
}

// Use groups, use path chains, associated type bounds, and associated
// bindings nest without an expression, type, or pattern between their levels,
// so the check and the formatter count them
#[test]
fn use_trees_and_associated_arguments_built_deep_are_reported() {
    let reported = on_small_stack(move || {
        [400, 100_000].map(|n| {
            let groups = built_use(n, grouped, "a::{b}");
            let chains = built_use(n, chained, "a::b");
            let (bounds, bindings) = (built_bounds(n), built_bindings(n));
            let reported = [
                (groups.nests_past_max_depth(), moxy::fmt!(&groups).is_err()),
                (chains.nests_past_max_depth(), moxy::fmt!(&chains).is_err()),
                (bounds.nests_past_max_depth(), moxy::fmt!(&bounds).is_err()),
                (bindings.nests_past_max_depth(), moxy::fmt!(&bindings).is_err()),
            ];
            std::mem::forget((groups, chains, bounds, bindings));
            reported
        })
    });

    assert_eq!(reported, [[(true, true); 4]; 2]);
}

// The deepest use trees and associated arguments that the check accepts run
// every traversal on a small stack
#[test]
fn use_trees_and_associated_arguments_built_to_the_limit_run_every_traversal() {
    let limits = on_small_stack(move || {
        let deepest = |past: &dyn Fn(usize) -> bool| (1..).find(|&n| past(n + 1)).unwrap();
        let groups = deepest(&|n| built_use(n, grouped, "a::{b}").nests_past_max_depth());
        let chains = deepest(&|n| built_use(n, chained, "a::b").nests_past_max_depth());
        let bounds = deepest(&|n| built_bounds(n).nests_past_max_depth());
        let bindings = deepest(&|n| built_bindings(n).nests_past_max_depth());
        traverse(built_use(groups, grouped, "a::{b}"));
        traverse(built_use(chains, chained, "a::b"));
        traverse(built_bounds(bounds));
        traverse(built_bindings(bindings));
        [groups, chains, bounds, bindings]
    });

    // Of the 288 nodes, the item or the type takes one. A level of groups
    // takes two (its chain and its group), a chain one, a bound six (two for
    // the argument and four around its bounds), and a binding nested through
    // its arguments two, with three more for the innermost binding's `u8` (two
    // around a binding's value and one for the type)
    assert_eq!(limits, [143, 287, 47, 142]);
}

// The formatter checks a macro body's groups before it reads them, so a body
// built 100,000 groups deep is an error instead of a stack overflow
#[test]
fn a_macro_body_built_deep_formats_to_an_error() {
    use moxy::token::Group;

    let formatted = on_small_stack(|| {
        let mut expr: moxy::ast::Expr = moxy::parse!("m!()").unwrap();
        let moxy::ast::Expr::Macro(mac) = &mut expr else { panic!() };
        mac.mac.body.tokens = (0..100_000).fold(TokenStream::new(), |stream, _| {
            TokenStream::from_iter([TokenTree::Group(Group::new(Delim::Bracket, stream))])
        });
        let formatted = moxy::fmt!(&expr).map_err(|error| error.to_string());
        drop(expr);
        formatted
    });

    assert_eq!(
        formatted,
        Err(format!(
            "the tree nests more than {} nodes deep",
            moxy::token::MAX_NESTED_NODES
        ))
    );
}

// A macro body or attribute arguments, which a traversal walks as deep as
// their groups nest, take a level of the parser per group, so an expression
// nested to the limit around them parses only while the whole tree fits, and
// what parses passes the depth check and formats
#[test]
fn a_macro_body_inside_a_deep_expression_counts_its_groups() {
    use moxy::ast::Expr;

    let message = format!("nesting is deeper than {} levels", moxy::token::MAX_DEPTH);
    let outcomes = on_small_stack(|| {
        [
            format!("{}m!()*0{}", "(".repeat(143), ") = 0".repeat(143)),
            format!("{}m!((()))", "|| 0 + ".repeat(140)),
            format!("{}m!((()))", "|| 0 + ".repeat(142)),
            format!("{}#[a((b))] 0{}", "(".repeat(140), ") = 0".repeat(140)),
            format!("{}#[a((((b))))] x", "-".repeat(142)),
        ]
        .map(|source| match moxy::parse!(source as Expr) {
            Ok(expr) => {
                let outcome = Ok((expr.nests_past_max_depth(), moxy::fmt!(&expr).is_ok()));
                traverse(expr);
                outcome
            }
            Err(error) => Err(error.message().to_string()),
        })
    });

    assert_eq!(
        outcomes,
        [
            Ok((false, true)),
            Ok((false, true)),
            Err(message.clone()),
            Ok((false, true)),
            Err(message)
        ]
    );
}

// A built tree whose nodes and macro body groups fill the limit together runs
// every traversal on a small stack; one more group fails the check
#[test]
fn a_built_tree_and_its_macro_body_share_the_node_limit() {
    use moxy::ast::Expr;
    use moxy::token::Group;

    let tree = |groups: usize| {
        let mut seed: Expr = moxy::parse!("m!()").unwrap();
        let Expr::Macro(mac) = &mut seed else { panic!() };
        mac.mac.body.tokens = (0..groups).fold(TokenStream::new(), |stream, _| {
            TokenStream::from_iter([TokenTree::Group(Group::new(Delim::Bracket, stream))])
        });
        let template: Expr = moxy::parse!("-x").unwrap();
        (0..143).fold(seed, |expr, _| {
            let mut outer = template.clone();
            *negated(&mut outer) = expr;
            outer
        })
    };

    // 144 expressions take 144 nodes, and each group two of the 144 left
    let checked = on_small_stack(move || {
        let past = tree(73).nests_past_max_depth();
        let at = tree(72);
        let checked = (at.nests_past_max_depth(), past);
        traverse(at);
        checked
    });

    assert_eq!(checked, (false, true));
}

// Leaves that add nodes or token groups without parser levels of their own
const LEAVES: &[(char, &str)] = &[
    ('e', "x"),
    ('e', "m!([[[x]]])"),
    ('e', "#[a((b))] x"),
    ('e', "#[a((b))] x + 0"),
    ('e', "#[a((b))] .."),
    ('e', "#[a((b))] ..x"),
    ('e', "0*0+0<<0&0^0|0==0&&0||0"),
    ('e', "a = 0.. = 0.. = 0"),
    ('t', "u8"),
    ('t', "m!([[[u8]]])"),
    ('t', "A<B<C> = u8, D: E>"),
    ('p', "x"),
    ('p', "m!([[[x]]])"),
    ('p', "1..=2"),
];

// A leaf that only one kind reads, which tells the kind of the centre of an
// outer construct, as a pattern for "closure pattern"
const CENTRE_PROBES: &[(char, &str)] = &[('e', "1 + 1"), ('t', "&'a u8"), ('p', "ref x")];

// Text after each level of an outer expression that makes the level wrap more
// nodes, as an assignment wraps its left side
const SUFFIXES: &[&str] = &["", " = 0", "*0+0", " = 0.. = 0"];

// Every construct is an outer this many times, each with the next inner of
// its centre's kind in turn, so every construct is also an inner about as
// often
const INNERS_PER_OUTER: usize = 1;

// The construct that fills a tree to the node limit around or inside another:
// a closure and a binary chain per level, two nodes for each level the parser
// takes, the most a construct holds
const FILLER: &str = "closure in an operand";

// The constructs nested in one another: one nested `outer` levels, with
// `suffix` after each, around another nested `inner` levels around a leaf
fn composed(outer: usize, inner: usize, levels: (usize, usize), leaf: &str, suffix: &str) -> String {
    let (_, _, before, open, _, close, after, _) = NESTED[inner];
    let core = format!("{before}{}{leaf}{}{after}", open.repeat(levels.1), close.repeat(levels.1));
    let (_, _, before, open, _, close, after, _) = NESTED[outer];
    format!(
        "{before}{}{core}{}{after}",
        open.repeat(levels.0),
        format!("{close}{suffix}").repeat(levels.0)
    )
}

// The roots a pair of a kind is parsed as: an expression, outside the half
// nesting, also as a statement, where a block-like root takes its own level.
// The filler compositions parse statements of a block as well.
fn roots(kind: char, half: bool) -> &'static [char] {
    match kind {
        'e' if half => &['e'],
        'e' => &['e', 's'],
        't' => &['t'],
        _ => &['p'],
    }
}

// The kind of a construct's centre, which a leaf that only that kind reads
// fills, or `None` when no probe does
fn centre_kind(case: usize, parses: &dyn Fn(char, String) -> bool) -> Option<char> {
    let (_, kind, before, open, _, close, after, _) = NESTED[case];
    CENTRE_PROBES
        .iter()
        .find(|(_, probe)| parses(kind, format!("{before}{open}{probe}{close}{after}")))
        .map(|(centre, _)| *centre)
}

// Parses a source as `root` and, if it parses, checks its depth and formats
// it, every traversal running when `traverse_all` is set; `None` when it does
// not parse. It runs on the caller's stack.
fn parsed_tree_holds(root: char, source: String, traverse_all: bool) -> Option<bool> {
    fn holds<T>(node: T, traverse_all: bool) -> bool
    where
        T: moxy::token::ToTokenStream + moxy::token::Spanner + Clone + PartialEq + std::fmt::Debug,
        T: moxy::fmt::Format + serde::Serialize + Walked + 'static,
    {
        let holds = !node.nests_past_max_depth() && moxy::fmt!(&node).is_ok();

        if holds && traverse_all {
            traverse(node);
        }

        holds
    }

    match root {
        'e' => moxy::parse!(source as moxy::ast::Expr)
            .ok()
            .map(|node| holds(node, traverse_all)),
        's' => moxy::parse!(source as moxy::ast::Stmt)
            .ok()
            .map(|node| holds(node, traverse_all)),
        'b' => moxy::parse!((format!("{{ {source} }}")) as moxy::ast::StmtBlock)
            .ok()
            .map(|node| holds(node, traverse_all)),
        't' => moxy::parse!(source as moxy::ast::Type)
            .ok()
            .map(|node| holds(node, traverse_all)),
        _ => moxy::parse!(source as moxy::ast::Pattern)
            .ok()
            .map(|node| holds(node, traverse_all)),
    }
}

// The deepest `n` in `[low, high)` for which `parses` holds, by bisection,
// given that it holds at `low`
fn deepest_in(mut low: usize, mut high: usize, mut parses: impl FnMut(usize) -> bool) -> usize {
    while high - low > 1 {
        let middle = (low + high) / 2;

        if parses(middle) {
            low = middle;
        } else {
            high = middle;
        }
    }

    low
}

// Whatever parses passes the depth check and formats, for constructs nested
// in one another around leaves that add nodes without levels: the depth check
// and the parser count the same tree. An expression is parsed as an
// expression, a statement, and the statement of a block.
#[test]
fn composed_constructs_that_parse_pass_the_depth_check() {
    on_small_stack(compose);
}

// The compositions of `composed_constructs_that_parse_pass_the_depth_check`,
// run on one small stack, whose frames here take a few kilobytes
fn compose() {
    let failures = std::cell::RefCell::new(Vec::new());

    // Whether a source parses as `root`; a tree that parses and fails the
    // depth check or formatting is a failure
    let parses = |root: char, source: String| match parsed_tree_holds(root, source.clone(), false) {
        Some(false) => {
            failures.borrow_mut().push(format!("{root}: {source}"));
            true
        }
        outcome => outcome.is_some(),
    };
    let composable: Vec<usize> = (0..NESTED.len()).filter(|&case| !NESTED[case].7).collect();
    let limits: std::collections::HashMap<usize, usize> = composable
        .iter()
        .map(|&case| (case, deepest_in(1, 300, |n| parses(NESTED[case].1, nested(case, n)))))
        .collect();
    // Files and blocks hold their centre in an item or a statement, which the
    // leaves do not fill
    let centres: std::collections::HashMap<usize, char> = composable
        .iter()
        .filter(|&&case| matches!(NESTED[case].1, 'e' | 't' | 'p'))
        .filter_map(|&case| centre_kind(case, &parses).map(|centre| (case, centre)))
        .collect();
    let (mut checked, mut unfitting, mut dropped) = (0, 0, Vec::new());

    // Checks every tree that parses as `root` on the way to the deepest `n` in
    // `[low, high)` at which `source(n)` parses, which the caller has seen
    // parse at `low`, and runs every traversal on that deepest tree
    let sweep = |root: char, low: usize, high: usize, source: &dyn Fn(usize) -> String| {
        let deepest = deepest_in(low, high, |n| parses(root, source(n)));

        match parsed_tree_holds(root, source(deepest), true) {
            Some(true) => {}
            Some(false) => failures.borrow_mut().push(format!("{root}: {}", source(deepest))),
            None => failures
                .borrow_mut()
                .push(format!("{root} does not parse at the low bound: {}", source(deepest))),
        }
    };

    // Each type and pattern construct alone around every leaf of its centre's
    // kind, as deep as it parses: where the leaf's own nodes meet the limit.
    // Expression constructs meet it inside the filler below.
    for (&case, &centre) in centres.iter().filter(|(case, _)| NESTED[**case].1 != 'e') {
        let (_, kind, before, open, _, close, after, _) = NESTED[case];

        for &(_, leaf) in LEAVES.iter().filter(|(leaf_kind, _)| *leaf_kind == centre) {
            let source = |n: usize| format!("{before}{}{leaf}{}{after}", open.repeat(n), close.repeat(n));

            // A leaf that the centre does not take, as an assignment in the
            // end of a range
            if !parses(kind, source(1)) {
                unfitting += 1;
                continue;
            }

            checked += 1;
            sweep(kind, 1, limits[&case] + 2, &source);
        }
    }

    // Each expression construct one level inside the filler, around every leaf
    // of its centre's kind, and one level around it as a statement, with the
    // filler as deep as it parses: where an off-by-one in either count shows
    let filler = composable.iter().copied().find(|&case| NESTED[case].0 == FILLER).unwrap();

    for (&case, &centre) in centres.iter().filter(|(case, _)| NESTED[**case].1 == 'e') {
        for &(_, leaf) in LEAVES.iter().filter(|(leaf_kind, _)| *leaf_kind == centre) {
            if !parses('e', composed(filler, case, (1, 1), leaf, "")) {
                unfitting += 1;
                continue;
            }

            checked += 1;
            sweep('e', 1, limits[&filler] + 1, &|n| composed(filler, case, (n, 1), leaf, ""));
        }

        if centre != 'e' {
            continue;
        }

        // As a statement, where a block-like root takes its own level, and as
        // the statement of a block, with and without a binary chain before the
        // filler, so the nodes meet the limit at either parity
        let (_, _, before, open, _, close, after, _) = NESTED[case];
        let filler_open = NESTED[filler].3;

        for (root, prefix) in [('s', ""), ('s', "0 + "), ('b', ""), ('b', "0 + ")] {
            let source = |n: usize| format!("{before}{open}{prefix}{}x{close}{after}", filler_open.repeat(n));

            if !parses(root, source(1)) {
                unfitting += 1;
                continue;
            }

            checked += 1;
            sweep(root, 1, limits[&filler] + 1, &source);
        }
    }

    let mut turns = std::collections::HashMap::<char, usize>::new();

    // Pairs of constructs, the outer nested 1 level, half its limit, or one
    // short of it, with a suffix after each level, around an inner nested as
    // deep as it parses
    for &outer in &composable {
        let kind = NESTED[outer].1;
        let Some(&centre) = centres.get(&outer) else {
            continue;
        };
        let inners: Vec<usize> = composable
            .iter()
            .copied()
            .filter(|case| NESTED[*case].1 == centre && centres.contains_key(case))
            .collect();

        for _ in 0..INNERS_PER_OUTER {
            let turn = turns.entry(centre).or_default();
            let inner = inners[*turn % inners.len()];
            let leaves: Vec<&str> = LEAVES
                .iter()
                .filter(|(k, _)| *k == centres[&inner])
                .map(|(_, leaf)| *leaf)
                .collect();
            let leaf = leaves[*turn / inners.len() % leaves.len()];
            let chosen = if kind == 'e' { SUFFIXES[*turn % SUFFIXES.len()] } else { "" };
            *turn += 1;

            let half = limits[&outer] / 2;

            for target in [1, half, limits[&outer].saturating_sub(1).max(1)] {
                for &root in roots(kind, target == half && half > 1) {
                    let reads = |suffix: &str| parses(root, composed(outer, inner, (1, 0), leaf, suffix));

                    // A suffix the grammar rejects after the construct, as an
                    // assignment after a range or `*0+0` after a block-like
                    // statement, gives way to none
                    let suffix = if reads(chosen) {
                        chosen
                    } else if reads("") {
                        unfitting += 1;
                        ""
                    } else {
                        dropped.push(format!(
                            "{} in {} as {root}: does not parse around `{leaf}`",
                            NESTED[inner].0, NESTED[outer].0
                        ));
                        continue;
                    };

                    // The outer, as deep as it parses up to the target around
                    // the bare inner, then the inner as deep as it parses
                    let levels = deepest_in(1, target + 1, |n| parses(root, composed(outer, inner, (n, 0), leaf, suffix)));

                    checked += 1;
                    sweep(root, 0, limits[&inner] + 1, &|n| {
                        composed(outer, inner, (levels, n), leaf, suffix)
                    });
                }
            }
        }
    }

    eprintln!(
        "{checked} compositions, {unfitting} leaves or suffixes that do not fit, {} constructs with a centre",
        centres.len()
    );
    assert!(checked > 900, "{checked} compositions parsed");
    let failures = failures.into_inner();
    assert!(
        failures.is_empty(),
        "parsed but failed the depth check or formatting:\n{}",
        failures.join("\n")
    );
    // `static` begins an item at the start of a statement, so a static
    // closure is no statement
    assert!(
        dropped.iter().all(|drop| drop.contains("in static closure as ")),
        "dropped:\n{}",
        dropped.join("\n")
    );
    assert_eq!(dropped.len(), 2, "dropped:\n{}", dropped.join("\n"));
}

fn first_operand(expr: &mut moxy::ast::Expr) -> &mut moxy::ast::Expr {
    let moxy::ast::Expr::BinaryChain(chain) = expr else { panic!() };
    chain.operands.first_mut().unwrap()
}

fn tuple_element(expr: &mut moxy::ast::Expr) -> &mut moxy::ast::Expr {
    let moxy::ast::Expr::Tuple(tuple) = expr else { panic!() };
    tuple.elems.inner.first_mut().unwrap()
}

fn call_argument(expr: &mut moxy::ast::Expr) -> &mut moxy::ast::Expr {
    let moxy::ast::Expr::Postfix(postfix) = expr else { panic!() };
    let moxy::ast::Postfix::Call(call) = &mut postfix.ops[0] else {
        panic!()
    };
    call.args.inner.first_mut().unwrap()
}

fn tuple_struct_element(pattern: &mut moxy::ast::Pattern) -> &mut moxy::ast::Pattern {
    let moxy::ast::Pattern::TupleStruct(tuple) = pattern else {
        panic!()
    };
    tuple.elems.inner.first_mut().unwrap()
}

fn tuple_type_element(ty: &mut moxy::ast::Type) -> &mut moxy::ast::Type {
    let moxy::ast::Type::Tuple(tuple) = ty else { panic!() };
    tuple.elems.inner.first_mut().unwrap()
}

// Trees built to the node limit through nodes that hold their children in
// lists run every traversal on a small stack
#[test]
fn trees_of_list_nodes_built_to_the_limit_run_every_traversal() {
    use moxy::ast::{Expr, Pattern, Type};

    let n = moxy::token::MAX_NESTED_NODES - 1;

    on_small_stack(move || {
        traverse(built::<Expr>("x+0", "x", n, first_operand));
        traverse(built::<Expr>("(x,)", "x", n, tuple_element));
        traverse(built::<Expr>("f(x)", "x", n, call_argument));
        traverse(built::<Pattern>("S(x)", "x", n, tuple_struct_element));
        traverse(built::<Type>("(u8,)", "u8", n, tuple_type_element));
    });
}

// Attributes read before their expression or closure parameter has entered its
// level count from that level, and a block-like statement or match arm parsed
// as the root takes the level its block or match would give it, so each tree
// here parses one level short of the limit and is a depth error at it, where
// its nodes would pass 288.
//
// A range without a start takes no level, so `#[a] ..` under closures stops one
// level earlier than a bare `..`: the depth check counts the range as a node,
// and without the attribute's level its argument groups would take
// `(#[a((b))] ..)*0+0`, a composition leaf, past 288 nodes.
#[test]
fn attributes_and_block_like_roots_take_their_levels() {
    use moxy::ast::{Expr, Stmt};

    let message = format!("nesting is deeper than {} levels", moxy::token::MAX_DEPTH);

    // Each row one level short of the limit at `0` and at it at `1`
    let errors = on_small_stack(|| {
        [0, 1].map(|past| {
            let exprs = [
                format!("{}|| #[a((b))] x + 0", "|| 0 + ".repeat(141 + past)),
                format!(
                    "0 + {}|#[a{}b{}] x| x",
                    "|| 0 + ".repeat(99),
                    "(".repeat(44 + past),
                    ")".repeat(44 + past)
                ),
                format!("{}#[a] ..", "|| ".repeat(143 + past)),
            ]
            .map(|source| moxy::parse!(source as Expr).err().map(|error| error.message().to_string()));
            let stmts = [
                format!("if 0 + {}x {{}}", "|| 0 + ".repeat(142 + past)),
                format!("while 0 + {}x {{}}", "|| 0 + ".repeat(142 + past)),
                format!("match 0 + {}x {{}}", "|| 0 + ".repeat(142 + past)),
                format!("for a in 0 + {}x {{}}", "|| 0 + ".repeat(142 + past)),
                format!("{}if 0 + x {{}}{}", "unsafe { ".repeat(142 + past), " }".repeat(142 + past)),
            ]
            .map(|source| moxy::parse!(source as Stmt).err().map(|error| error.message().to_string()));
            (exprs, stmts)
        })
    });

    assert_eq!(
        errors,
        [
            ([(); 3].map(|_| None), [(); 5].map(|_| None)),
            ([(); 3].map(|_| Some(message.clone())), [(); 5].map(|_| Some(message.clone())))
        ]
    );
}

fn item_body(block: &mut moxy::ast::StmtBlock) -> &mut moxy::ast::StmtBlock {
    let moxy::ast::Stmt::Item(item) = &mut block.stmts.inner[0] else {
        panic!()
    };
    let moxy::ast::Item::Fn(function) = &mut **item else {
        panic!()
    };
    function.body.as_mut().unwrap()
}

fn impl_member_body(block: &mut moxy::ast::StmtBlock) -> &mut moxy::ast::StmtBlock {
    let moxy::ast::Stmt::Item(item) = &mut block.stmts.inner[0] else {
        panic!()
    };
    let moxy::ast::Item::Impl(item) = &mut **item else { panic!() };
    let moxy::ast::ImplItem::Fn(function) = &mut item.items.inner[0] else {
        panic!()
    };
    function.body.as_mut().unwrap()
}

fn trait_member_body(block: &mut moxy::ast::StmtBlock) -> &mut moxy::ast::StmtBlock {
    let moxy::ast::Stmt::Item(item) = &mut block.stmts.inner[0] else {
        panic!()
    };
    let moxy::ast::Item::Trait(item) = &mut **item else { panic!() };
    let moxy::ast::TraitItem::Fn(function) = &mut item.items.inner[0] else {
        panic!()
    };
    function.body.as_mut().unwrap()
}

fn module_content(item: &mut moxy::ast::Item) -> &mut moxy::ast::Item {
    let moxy::ast::Item::Mod(module) = item else { panic!() };
    &mut module.content.as_mut().unwrap().items.inner[0]
}

// A block holding modules nested `n` deep
fn built_modules(n: usize) -> moxy::ast::StmtBlock {
    let mut block: moxy::ast::StmtBlock = moxy::parse!("{ mod m {} }").unwrap();
    let moxy::ast::Stmt::Item(item) = &mut block.stmts.inner[0] else {
        panic!()
    };
    **item = built("mod m { mod m {} }", "mod m {}", n, module_content);
    block
}

fn arm_body(expr: &mut moxy::ast::Expr) -> &mut moxy::ast::Expr {
    let moxy::ast::Expr::Match(expr) = expr else { panic!() };
    &mut expr.arms.inner[0].body
}

fn field_value(expr: &mut moxy::ast::Expr) -> &mut moxy::ast::Expr {
    let moxy::ast::Expr::Struct(expr) = expr else { panic!() };
    &mut expr.body.inner.fields.first_mut().unwrap().expr
}

fn typed_closure_body(expr: &mut moxy::ast::Expr) -> &mut moxy::ast::Expr {
    let moxy::ast::Expr::Closure(closure) = expr else { panic!() };
    let moxy::ast::Expr::Block(block) = &mut *closure.body else {
        panic!()
    };
    let moxy::ast::Stmt::Expr(expr, _) = &mut block.block.stmts.inner[0] else {
        panic!()
    };
    expr
}

// Heavy forms, whose parse takes more levels than the nodes the depth check
// counts for them, built to the node limit run every traversal on a small
// stack
#[test]
fn heavy_forms_built_to_the_limit_run_every_traversal() {
    use moxy::ast::{Expr, StmtBlock};

    let limits = on_small_stack(|| {
        let deepest = |past: &dyn Fn(usize) -> bool| (1..).find(|&n| past(n + 1)).unwrap();
        let items = |n| built::<StmtBlock>("{ fn f() {} }", "{}", n, item_body);
        let members = |n| built::<StmtBlock>("{ impl A { fn f() {} } }", "{}", n, impl_member_body);
        let trait_members = |n| built::<StmtBlock>("{ trait A { fn f() {} } }", "{}", n, trait_member_body);
        let arms = |n| built::<Expr>("match x { _ => y }", "y", n, arm_body);
        let fields = |n| built::<Expr>("S { a: x }", "x", n, field_value);
        let closures = |n| built::<Expr>("|| -> u8 { x }", "x", n, typed_closure_body);
        let limits = [
            deepest(&|n| items(n).nests_past_max_depth()),
            deepest(&|n| members(n).nests_past_max_depth()),
            deepest(&|n| trait_members(n).nests_past_max_depth()),
            deepest(&|n| built_modules(n).nests_past_max_depth()),
            deepest(&|n| arms(n).nests_past_max_depth()),
            deepest(&|n| fields(n).nests_past_max_depth()),
            deepest(&|n| closures(n).nests_past_max_depth()),
        ];

        traverse(items(limits[0]));
        traverse(members(limits[1]));
        traverse(trait_members(limits[2]));
        traverse(built_modules(limits[3]));
        traverse(arms(limits[4]));
        traverse(fields(limits[5]));
        traverse(closures(limits[6]));
        limits
    });

    // A block and an item per level take two of the 288 nodes, a module, an
    // arm body, or a field value one, and a typed closure three (the closure,
    // its block expression, and its block). The modules sit in a block.
    assert_eq!(limits, [143, 143, 143, 286, 287, 287, 95]);
}

// A message names what follows where it can: no value after a meta's name, the
// value a `FromMeta` literal rejects, a path whole when it is short, and an
// empty group
#[test]
fn messages_name_what_follows() {
    use moxy::ast::{FromMeta, Meta, ParseError};
    use moxy::token::{Cursor, Group, LitBool, Scan};

    let unit: Meta = moxy::parse!("name").unwrap();
    let sum: Meta = moxy::parse!("a = 1 + 2").unwrap();
    let path_value: Meta = moxy::parse!("a = A::B").unwrap();
    let long_source = format!("a = 1 + \"{}\"", "x".repeat(128));
    let long_value: Meta = moxy::parse!(long_source).unwrap();
    let attr = moxy::parse!("#[a = 1 +]" as moxy::ast::Attribute).err().unwrap();
    let path: moxy::ast::Path = moxy::parse!("a::b").unwrap();
    let field: TokenStream = "a: b".parse().unwrap();
    let empty = TokenStream::from_iter([TokenTree::Group(Group::new(Delim::None, TokenStream::new()))]);
    let word = LitBool::scan(Cursor::new("x", 0)).err().unwrap();
    let raw = LitBool::scan(Cursor::new("r#true", 0)).err().unwrap();

    assert_eq!(
        u8::from_meta(&unit).err().unwrap().message(),
        "expected integer literal, found no value"
    );
    assert_eq!(
        u8::from_meta(&sum).err().unwrap().message(),
        "expected integer literal, found `1 + 2`"
    );
    assert_eq!(
        u8::from_meta(&path_value).err().unwrap().message(),
        "expected integer literal, found `A::B`"
    );
    assert_eq!(
        u8::from_meta(&long_value).err().unwrap().message(),
        "expected integer literal, found `1`"
    );
    assert_eq!(attr.message(), "expected expression, found `]`");
    assert_eq!(
        unit.for_each(|_| Ok(())).err().unwrap().message(),
        "expected delimited list, as in `name(...)`, found no value"
    );
    assert_eq!(
        ParseError::expected(&path, "identifier").message(),
        "expected identifier, found `a::b`"
    );
    assert_eq!(
        ParseError::expected(&empty, "identifier").message(),
        "expected identifier, found empty group"
    );
    assert_eq!(
        ParseError::expected(&field, "identifier").message(),
        "expected identifier, found `a`"
    );
    assert_eq!(word.to_string(), "expected boolean literal, found `x`");
    assert_eq!(raw.to_string(), "expected boolean literal, found `r#true`");

    for message in [
        "expected integer literal, found no value",
        "expected delimited list, as in `name(...)`, found no value",
        "expected identifier, found `a::b`",
    ] {
        assert!(super::differential::follows_style(message), "`{message}`");
    }

    for message in ["expected the end, found `a`", "unexpected `a`, expected an item, found `a`"] {
        assert!(!super::differential::follows_style(message), "`{message}`");
    }
}

// The end of a group built in code is its last token, where an error at the end
// of its tokens points
#[test]
fn a_built_group_closes_at_its_last_token() {
    use moxy::token::Group;

    let tokens: TokenStream = "a b".parse().unwrap();
    let last = tokens.iter().last().unwrap().span();
    let group = Group::new(Delim::Brace, tokens);
    let parser = moxy::ast::Parser::from_group(&group);
    parser.advance();
    parser.advance();

    assert_eq!(parser.expected("`c`").span().byte_range(), last.byte_range());
}
