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

#[test]
fn deep_trees_of_every_kind_parse_and_traverse_or_report_the_depth() {
    let message = format!("nesting is deeper than {} levels", moxy::token::MAX_DEPTH);
    let r = |text: &str, count: usize| text.repeat(count);

    for n in [40, 100, 143, 400] {
        let expressions = [
            format!("{}a{}", r("S { a: ", n), r(" }", n)),
            format!("{}1{}", r("|| -> T { ", n), r(" }", n)),
            format!("{}a", r("a + ", 15 * n)),
            format!("a{}", r(".b()", 15 * n)),
            format!("a{}", r("?", 15 * n)),
            format!("{}a{}{}", r("(", n), r(")", n), r(".b()", 2 * n)),
            format!("{}x{}", r("{ ", n), r(" }.a()", n)),
        ];
        let types = [
            format!("{}T", r("Fn() -> ", n)),
            format!("{}T", r("impl Fn() -> ", n)),
            format!("{}u8{}", r("A<B = ", n), r(">", n)),
            format!("{}C{}", r("A<B: ", n), r(">", n)),
            format!("{}u8{}", r("fn(", n), r(")", n)),
            format!("{}u8{}", r("[u8; { let x: ", n), r("; 1 }]", n)),
            format!("{}u8{}", r("Vec<", n), r(">", n)),
        ];
        let patterns = [format!("{}x{}", r("S { a: ", n), r(" }", n))];
        let files = [
            format!("{}{}", r("impl A { fn f() { ", n), r("} }", n)),
            format!("{}{}", r("trait A { fn f() { ", n), r("} }", n)),
            format!("{}{}", r("fn f() { ", n), r("}", n)),
            format!("use {}a;", r("a::", 4 * n)),
        ];

        let errors = expressions
            .into_iter()
            .map(traverse_on_small_stack::<moxy::ast::Expr>)
            .chain(types.into_iter().map(traverse_on_small_stack::<moxy::ast::Type>))
            .chain(patterns.into_iter().map(traverse_on_small_stack::<moxy::ast::Pattern>))
            .chain(files.into_iter().map(traverse_on_small_stack::<moxy::ast::File>));

        for error in errors.flatten() {
            assert_eq!(error, message, "at {n} levels");
        }
    }

    // Real chains stay well inside the limit
    assert!(traverse_on_small_stack::<moxy::ast::Expr>(format!("a{}", r(".b()", 200))).is_none());
}
