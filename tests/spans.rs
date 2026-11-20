use moxy::ast::*;
use moxy::token::{Spanner, ToTokenStream};

// Collects every node whose span differs from the span of its emitted tokens
#[derive(Default)]
struct SpanCheck {
    mismatches: Vec<String>,
}

impl SpanCheck {
    fn check(&mut self, kind: &str, node: &(impl Spanner + ToTokenStream)) {
        let tokens = node.to_token_stream();

        if tokens.is_empty() {
            return;
        }

        let (actual, expected) = (node.span().byte_range(), tokens.span().byte_range());

        if actual != expected {
            self.mismatches
                .push(format!("{kind} `{tokens}`: {actual:?}, tokens at {expected:?}"));
        }
    }
}

impl<'ast> Visit<'ast> for SpanCheck {
    fn visit_expr(&mut self, node: &'ast Expr) {
        self.check("expression", node);
        walk_expr(self, node);
    }

    fn visit_pattern(&mut self, node: &'ast Pattern) {
        self.check("pattern", node);
        walk_pattern(self, node);
    }

    fn visit_type(&mut self, node: &'ast Type) {
        self.check("type", node);
        walk_type(self, node);
    }

    fn visit_item(&mut self, node: &'ast Item) {
        self.check("item", node);
        walk_item(self, node);
    }

    fn visit_stmt(&mut self, node: &'ast Stmt) {
        self.check("statement", node);
        walk_stmt(self, node);
    }

    fn visit_impl_item(&mut self, node: &'ast ImplItem) {
        self.check("impl item", node);
        walk_impl_item(self, node);
    }

    fn visit_trait_item(&mut self, node: &'ast TraitItem) {
        self.check("trait item", node);
        walk_trait_item(self, node);
    }

    fn visit_variant(&mut self, node: &'ast Variant) {
        self.check("variant", node);
        walk_variant(self, node);
    }

    fn visit_field_def(&mut self, node: &'ast Field) {
        self.check("field", node);
        walk_field_def(self, node);
    }

    fn visit_generic_param(&mut self, node: &'ast GenericParam) {
        self.check("generic parameter", node);
        walk_generic_param(self, node);
    }
    fn visit_path(&mut self, node: &'ast Path) {
        self.check("path", node);
        walk_path(self, node);
    }

    fn visit_path_segment(&mut self, node: &'ast PathSegment) {
        self.check("path segment", node);
        walk_path_segment(self, node);
    }

    fn visit_qself(&mut self, node: &'ast QSelf) {
        self.check("qualified self", node);
        walk_qself(self, node);
    }

    fn visit_generics(&mut self, node: &'ast Generics) {
        self.check("generics", node);
        walk_generics(self, node);
    }

    fn visit_where_clause(&mut self, node: &'ast WhereClause) {
        self.check("where clause", node);
        walk_where_clause(self, node);
    }

    fn visit_where_predicate(&mut self, node: &'ast WherePredicate) {
        self.check("where predicate", node);
        walk_where_predicate(self, node);
    }

    fn visit_signature(&mut self, node: &'ast Signature) {
        self.check("signature", node);

        if let Some(abi) = &node.abi {
            self.check("abi", abi);
        }

        walk_signature(self, node);
    }

    fn visit_use_tree(&mut self, node: &'ast UseTree) {
        self.check("use tree", node);
        walk_use_tree(self, node);
    }

    fn visit_use_path_chain(&mut self, node: &'ast UsePathChain) {
        self.check("use path", node);
        walk_use_path_chain(self, node);
    }

    fn visit_trait_bound(&mut self, node: &'ast TraitBound) {
        self.check("trait bound", node);
        walk_trait_bound(self, node);
    }

    fn visit_type_bound(&mut self, node: &'ast TypeBound) {
        self.check("type bound", node);
        walk_type_bound(self, node);
    }

    fn visit_generic_argument(&mut self, node: &'ast GenericArgument) {
        self.check("generic argument", node);
        walk_generic_argument(self, node);
    }

    fn visit_fn_param(&mut self, node: &'ast FnParam) {
        self.check("parameter", node);
        walk_fn_param(self, node);
    }

    fn visit_field_value(&mut self, node: &'ast FieldValue) {
        self.check("field value", node);
        walk_field_value(self, node);
    }

    fn visit_match_arm(&mut self, node: &'ast MatchArm) {
        self.check("match arm", node);
        walk_match_arm(self, node);
    }

    fn visit_attribute(&mut self, node: &'ast Attribute) {
        self.check("attribute", node);
        walk_attribute(self, node);
    }

    fn visit_receiver(&mut self, node: &'ast Receiver) {
        self.check("receiver", node);
        walk_receiver(self, node);
    }

    fn visit_foreign_item(&mut self, node: &'ast ForeignItem) {
        self.check("foreign item", node);
        walk_foreign_item(self, node);
    }

    fn visit_closure_param(&mut self, node: &'ast ClosureParam) {
        self.check("closure parameter", node);
        walk_closure_param(self, node);
    }

    fn visit_return_type(&mut self, node: &'ast ReturnType) {
        self.check("return type", node);
        walk_return_type(self, node);
    }

    fn visit_visibility(&mut self, node: &'ast Visibility) {
        self.check("visibility", node);
        walk_visibility(self, node);
    }

    fn visit_lifetime(&mut self, node: &'ast Lifetime) {
        self.check("lifetime", node);
        walk_lifetime(self, node);
    }

    fn visit_stmt_block(&mut self, node: &'ast StmtBlock) {
        self.check("block", node);
        walk_stmt_block(self, node);
    }

    fn visit_bare_fn_arg(&mut self, node: &'ast BareFnArg) {
        self.check("function pointer parameter", node);
        walk_bare_fn_arg(self, node);
    }
}

#[test]
fn every_node_spans_exactly_its_tokens() {
    let source = r#"
        use std::{fmt, io::Write as _};
        struct Unit;
        struct Tuple(pub u8, std::vec::Vec<u8>);
        pub(crate) struct Named<'a, T: Clone = u8, const N: usize = 3> where T: Copy { a: &'a T, b: [u8; N] }
        enum E<T> { A, B(T), C { x: u8 } = 1 }
        union U { a: u8, b: u16 }
        type Alias<T> = Vec<T>;
        type Callback = Box<dyn Fn::(u8) -> u8 + Send>;
        const C: u8 = 1 + 2;
        static S: &str = "s";
        trait Tr: Clone { const K: u8; type T; fn f(&self) -> u8 { 1 } fn g(); }
        impl<T> Tr for Named<'_, T> { const K: u8 = 1; type T = u8; fn f(&self) -> u8 { self.a.len() as u8 } }
        mod m { fn inner() {} }
        extern "C" { fn ext(x: i32) -> i32; }
        macro_rules! mac { () => {} }
        fn f(a: u8, (b, _): (u8, u8)) -> Option<u8> {
            let x = zz + (a * b) - -a;
            let Some(y) = f2(x) else { return None };
            let s = S { a: 1, ..Default::default() };
            let c = |v: u8| -> u8 { v + 1 };
            #[allow(unused)] let r = &mut x[0..2];
            if let 1..=9 | 20 = x && y > 0 { y.method::<u8>(1)?.field } else { loop { break 'l; } };
            match x { 0 => (), n @ 1..=9 => {} _ if a => continue, _ => {} }
            for i in 0.. { while i < 3 { i += 1; } }
            async { a.await };
            unsafe { *p };
            m!(a);
            vec![1].len();
            x as u64
        }
    "#;

    let file: File = moxy::parse!(source).unwrap();
    let mut check = SpanCheck::default();
    for item in &file.items {
        check.visit_item(item);
    }
    assert!(check.mismatches.is_empty(), "{}", check.mismatches.join("\n"));
}

#[test]
fn a_group_closes_at_its_closing_delimiter() {
    let expression: Expr = moxy::parse!("zz + (a * b)").unwrap();
    let paren = expression.as_binary_chain().unwrap().operands[1].as_paren().unwrap();
    let (open, close) = (paren.content.open().byte_range(), paren.content.close().byte_range());
    assert_eq!(close.start - open.start, "(a * b".len());
    assert_eq!(close.len(), 1);
}

// Every node of every differential corpus source that parses
#[test]
fn every_corpus_node_spans_exactly_its_tokens() {
    let mut check = SpanCheck::default();

    for line in include_str!("differential/corpus.txt").lines() {
        let Some((kind, source)) = line.split_once(": ").filter(|_| !line.starts_with('#')) else {
            continue;
        };
        let source = source.replace("\\n", "\n");
        let before = check.mismatches.len();

        match kind {
            "e" => moxy::parse!(source as Expr).map(|node| check.visit_expr(&node)),
            "b" => moxy::parse!(source as StmtBlock).map(|node| check.visit_stmt_block(&node)),
            "p" => moxy::parse!(source as Pattern).map(|node| check.visit_pattern(&node)),
            "t" => moxy::parse!(source as Type).map(|node| check.visit_type(&node)),
            "i" => moxy::parse!(source as Item).map(|node| check.visit_item(&node)),
            "f" => moxy::parse!(source as File).map(|node| node.items.iter().for_each(|item| check.visit_item(item))),
            _ => panic!("unknown kind `{kind}`"),
        }
        .ok();

        for mismatch in &mut check.mismatches[before..] {
            mismatch.push_str(&format!(" in {line}"));
        }
    }

    assert!(check.mismatches.is_empty(), "{}", check.mismatches.join("\n"));
}

#[test]
fn nested_function_sugar_spans_in_linear_time() {
    // A path, a list of one value, and parenthesized arguments each took the
    // span of their last part twice, which doubled the work per level
    let source = format!("{}T", "Fn(A) -> ".repeat(64));
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .stack_size(64 << 20)
        .spawn(move || {
            let ty: Type = moxy::parse!(source).unwrap();
            sender.send(ty.span().byte_range().len()).unwrap();
        })
        .unwrap();
    let len = receiver
        .recv_timeout(std::time::Duration::from_secs(10))
        .expect("span took longer than 10 seconds");
    assert_eq!(len, 64 * "Fn(A) -> ".len() + 1);
}

// Positions count bytes, as the lexer does, so a token after text of more than
// one byte a character stays in its own source and its column counts
// characters
#[test]
fn tokens_after_multibyte_text_locate_in_their_own_source() {
    use moxy::token::source::SourceMap;
    use moxy::token::{Span, TokenStream};

    let first: TokenStream = "\"ééééé\"\n é x".parse().unwrap();
    let second: TokenStream = "yy zz".parse().unwrap();
    let x = first.iter().last().unwrap().span();
    let text = SourceMap::with(|map| {
        let Span::Fallback(span) = x else { panic!() };
        map.find(span).map(|source| source.slice(span))
    });

    assert_eq!(second.to_string(), "yy zz");
    assert_eq!(text.as_deref(), Some("x"));
    assert_eq!((x.start().index(), x.start().line(), x.start().column()), (17, 1, 3));
}

// The span of the last token of `source`, lexed now
fn last_span(source: &str) -> moxy::token::Span {
    let tokens: moxy::token::TokenStream = source.parse().unwrap();
    tokens.iter().last().unwrap().span()
}

// Whether the source map holds the source of `span`
fn has_source(span: moxy::token::Span) -> bool {
    let moxy::token::Span::Fallback(span) = span else { panic!() };
    moxy::token::source::SourceMap::with(|map| map.find(span).is_some())
}

#[test]
fn a_source_scope_releases_what_was_lexed_inside_it() {
    use moxy::token::source::SourceMap;

    let before = SourceMap::with(|map| map.len());
    let mut inside = Vec::new();

    for _ in 0..1_000 {
        let _scope = SourceMap::scope();
        let _: File = moxy::parse!("fn f() { g(1) }").unwrap();
        let _: File = moxy::parse!("struct S;").unwrap();
        inside.push(SourceMap::with(|map| map.len()));
    }

    assert_eq!(SourceMap::with(|map| map.len()), before);
    assert_eq!(inside, vec![before + 2; 1_000]);
}

// A span whose source was released finds no source, even once later sources
// are lexed, and reads as line 0 and column 0 instead of panicking
#[test]
fn a_released_span_finds_no_source() {
    use moxy::token::Location;
    use moxy::token::source::SourceMap;

    let kept = last_span("a\n  b");
    let released = {
        let _scope = SourceMap::scope();
        last_span("c\n  d")
    };
    let later = last_span("e\n  f");

    assert!(has_source(kept) && has_source(later) && !has_source(released));
    assert_eq!(released.start(), Location::new(0, 0, 0));
    assert_eq!(released.end(), Location::new(0, 0, 0));
    assert_eq!((kept.start().line(), kept.start().column()), (1, 2));
    assert_eq!((later.start().line(), later.start().column()), (1, 2));
}

#[test]
fn source_scopes_nest() {
    use moxy::token::source::SourceMap;

    let before = SourceMap::with(|map| map.len());
    let outer = SourceMap::scope();
    let first = last_span("a");
    let inner = SourceMap::scope();
    let second = last_span("b");
    drop(inner);

    let after_inner = (SourceMap::with(|map| map.len()), has_source(first), has_source(second));
    drop(outer);

    assert_eq!(after_inner, (before + 1, true, false));
    assert_eq!(SourceMap::with(|map| map.len()), before);
    assert!(!has_source(first));
}

// A scope releases what was lexed after it was made, whatever order scopes
// drop in
#[test]
fn a_scope_dropped_out_of_order_releases_what_it_saw_lexed() {
    use moxy::token::source::SourceMap;

    let first = SourceMap::scope();
    let before = last_span("x");
    let second = SourceMap::scope();
    drop(first);
    let during = last_span("y");
    drop(second);

    assert!(!has_source(before) && !has_source(during));
}

// Joining a span of no source, as an identifier made from text, gives the
// other span
#[test]
fn a_span_of_no_source_joins_as_the_other_span() {
    let ident: moxy::token::Ident = "a".parse().unwrap();
    let lexed = last_span("b c");

    assert_eq!(ident.span().join(lexed), lexed);
    assert_eq!(lexed.join(ident.span()), lexed);
}

// A byte order mark takes no column
#[test]
fn a_byte_order_mark_takes_no_column() {
    let span = last_span("\u{feff}a b");
    assert_eq!((span.start().line(), span.start().column()), (0, 2));
}

// A lex error spans the character at the cursor, which is none at the end of
// the input, so the error stays in its source and never splits a character
#[test]
fn a_lex_error_spans_the_character_it_is_at() {
    use moxy::token::source::SourceMap;
    use moxy::token::{Span, TokenStream};

    let source_text = |span: Span| {
        let Span::Fallback(span) = span else { panic!() };
        SourceMap::with(|map| map.find(span).map(|source| source.slice(span)))
    };
    let unterminated = "fn f() {\n  \"abc".parse::<TokenStream>().unwrap_err().span();
    let unexpected = "a €".parse::<TokenStream>().unwrap_err().span();

    assert_eq!((unterminated.start().line(), unterminated.start().column()), (1, 6));
    assert_eq!(source_text(unterminated).as_deref(), Some(""));
    assert_eq!(source_text(unexpected).as_deref(), Some("€"));
}

// Two spans of no source join by their positions
#[test]
fn spans_of_no_source_join_by_their_positions() {
    use moxy::token::Cursor;

    let joined = Cursor::new("a", 0).span().join(Cursor::new("abc", 5).span());
    assert_eq!(joined.byte_range(), 0..6);
}

// An item after a shebang line is located on the line after it
#[test]
fn an_item_after_a_shebang_line_keeps_its_line_and_column() {
    let file = File::from_source_file("#!/bin/sh\n  fn f() {}".to_owned()).unwrap();
    let start = moxy::token::Spanner::span(&file.items[0]).start();
    assert_eq!((start.line(), start.column()), (1, 2));
}
