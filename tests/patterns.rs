use moxy::ast::{PatType, Pattern};
use moxy::token::{Spanner, ToTokenStream};

#[test]
fn binding_tuple_slice_and_struct_patterns_preserve_shape() {
    for (source, expected, expected_kind) in [
        ("ref mut value @ Some(_)", "ref mut value @ Some(_)", 0),
        ("(first, second, ..)", "(first, second, ..)", 1),
        ("[first, .., last]", "[first, .., last]", 2),
        ("Point { x: renamed, y, .. }", "Point {\n\tx: renamed,\n\ty,\n\t..\n}", 3),
    ] {
        let pattern: Pattern = moxy::parse!(source).unwrap();
        assert_eq!(
            [
                pattern.is_ident(),
                pattern.is_tuple(),
                pattern.is_slice(),
                pattern.is_struct()
            ],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert!(!pattern.span().is_empty());
        assert!(!pattern.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&pattern).unwrap(), expected);
    }
}

#[test]
fn alternatives_ranges_references_and_typed_patterns_render_exactly() {
    for (source, expected, expected_kind) in [
        ("A | B | C", "A | B | C", 0),
        ("1..=10", "1..=10", 1),
        ("&mut value", "&mut value", 2),
    ] {
        let pattern: Pattern = moxy::parse!(source).unwrap();
        assert_eq!(
            [pattern.is_or(), pattern.is_range(), pattern.is_reference()],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert!(!pattern.span().is_empty());
        assert!(!pattern.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&pattern).unwrap(), expected);
    }
}

#[test]
fn typed_patterns_preserve_their_type_syntax() {
    let pattern: PatType = moxy::parse!("value: Option<T>").unwrap();
    assert!(!pattern.span().is_empty());
    assert_eq!(moxy::fmt!(&pattern).unwrap(), "value: Option<T>");
}

#[test]
fn suffixed_tuple_indices_are_rejected_as_member_patterns() {
    // As in rustc's parser, any unsuffixed integer is a tuple index
    for source in ["0", "0x1", "1_0", "4294967296"] {
        assert!(moxy::parse!(source as moxy::ast::Member).is_ok(), "`{source}`");
    }

    let result = moxy::parse!("0u8" as moxy::ast::Member);
    assert!(result.is_err(), "accepted a suffixed tuple index => {result:#?}");
}

#[test]
fn or_patterns_keep_a_leading_vert() {
    let expression: moxy::ast::Expr = moxy::parse!("match x { | 1 | 2 => {} }").unwrap();
    assert_eq!(
        moxy::token::ToTokenStream::to_token_stream(&expression).to_string(),
        "match x {| 1 | 2 => {}}"
    );
}

#[test]
fn negative_literals_are_patterns() {
    let expression: moxy::ast::Expr = moxy::parse!("match x { -1 => {} (-2, -2.5) => {} }").unwrap();
    assert_eq!(
        moxy::token::ToTokenStream::to_token_stream(&expression).to_string(),
        "match x {- 1 => {} (- 2 , - 2.5) => {}}"
    );
}

#[test]
fn formatting_keeps_the_qualifier_of_qualified_paths() {
    for (source, formatted) in [
        ("match x { <T as U>::C => {} }", "match x {\n\t<T as U>::C => {},\n}"),
        ("match x { <T>::D(a) => {} }", "match x {\n\t<T>::D(a) => {},\n}"),
        (
            "match x { <T as a::U>::S { a: 1 } => {} }",
            "match x {\n\t<T as a::U>::S {\n\t\ta: 1,\n\t} => {},\n}",
        ),
        ("<T as U>::f(x)", "<T as U>::f(x)"),
        ("<T as U>::S { a: 1 }", "<T as U>::S {\n\ta: 1,\n}"),
        ("<Vec<u8>>::new::<A>()", "<Vec<u8>>::new::<A>()"),
    ] {
        let expression: moxy::ast::Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(moxy::fmt!(&expression).unwrap(), formatted);
    }
}

#[test]
fn pattern_paths_read_fn_named_segments_as_names() {
    for source in [
        "Item::Fn(ItemFn { attrs, .. })",
        "Kind::FnMut",
        "Kind::FnOnce { a }",
        "Option::<u8>::Some(x)",
    ] {
        let pattern: Pattern = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(pattern.to_token_stream().to_string(), lexed.to_string());
    }

    let pattern: Pattern = moxy::parse!("Item::Fn(ItemFn { attrs, .. })").unwrap();
    let Pattern::TupleStruct(tuple_struct) = &pattern else {
        panic!("expected a tuple-struct pattern");
    };
    assert_eq!(tuple_struct.path.last().unwrap().ident.text(), "Fn");
    assert!(matches!(
        tuple_struct.path.last().unwrap().args,
        moxy::ast::PathArguments::None
    ));
    assert!(tuple_struct.elems.inner.first().unwrap().is_struct());
}

#[test]
fn half_open_range_patterns_end_before_the_arm() {
    for source in [
        "match x { 74.. => {} }",
        "match x { 1 | 74.. if a => {} }",
        "match x { (1.., ..=2) => {} 'a'..'z' => {} -5..A::B => {} }",
    ] {
        let expression: moxy::ast::Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(expression.to_token_stream().to_string(), lexed.to_string());
    }

    let expression: moxy::ast::Expr = moxy::parse!("match x { 74.. => {} }").unwrap();
    let pattern = &expression.as_match().unwrap().arms.inner[0].pat;
    let Pattern::Range(range) = pattern else {
        panic!("expected a range pattern");
    };
    assert!(range.start.is_some());
    assert!(range.end.is_none());
}

#[test]
fn an_underscore_is_a_wildcard_pattern_and_an_inferred_expression() {
    for source in ["_", "(_)"] {
        let pattern: Pattern = moxy::parse!(source).unwrap();
        assert!(
            pattern.is_wild() || matches!(&pattern, Pattern::Paren(paren) if paren.content.inner.is_wild()),
            "`{source}`"
        );
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(pattern.to_token_stream().to_string(), lexed.to_string());
    }

    let pattern: Pattern = moxy::parse!("(_, a)").unwrap();
    let Pattern::Tuple(tuple) = &pattern else {
        panic!("expected a tuple pattern");
    };
    assert!(tuple.elems.inner.first().unwrap().is_wild());

    let expression: moxy::ast::Expr = moxy::parse!("match a { _ => 1 }").unwrap();
    assert!(expression.as_match().unwrap().arms.inner[0].pat.is_wild());

    let expression: moxy::ast::Expr = moxy::parse!("_ = f()").unwrap();
    assert!(expression.as_assign().unwrap().left.is_infer());

    // A wildcard emits the identifier `_` that the compiler accepts
    #[cfg(feature = "proc-macro2")]
    {
        let pattern: Pattern = moxy::parse!("_").unwrap();
        let emitted: proc_macro2::TokenStream = pattern.to_token_stream().into();
        assert_eq!(emitted.to_string(), "_");
        assert!(matches!(emitted.into_iter().next(), Some(proc_macro2::TokenTree::Ident(_))));

        let emitted: proc_macro2::TokenStream = moxy::token::Underscore::default().to_token_stream().into();
        assert_eq!(emitted.to_string(), "_");
    }
}

#[test]
fn deeply_nested_parenthesized_patterns_parse_in_linear_time() {
    // Each level doubled the work before, which never ends at this depth
    let depth = moxy::token::MAX_DEPTH - 2;
    let source = format!("{}a{}", "(".repeat(depth), ")".repeat(depth));
    let tuples = format!("{}a{}", "(".repeat(depth), ",)".repeat(depth));

    // A debug build needs a larger stack than a test thread has for this depth
    let (paren, tuple, elapsed) = std::thread::Builder::new()
        .stack_size(64 << 20)
        .spawn(move || {
            let start = std::time::Instant::now();
            let pattern: Pattern = moxy::parse!(source).unwrap();
            let tuple: Pattern = moxy::parse!(tuples).unwrap();
            let elapsed = start.elapsed();
            (
                matches!(pattern, Pattern::Paren(_)),
                matches!(tuple, Pattern::Tuple(_)),
                elapsed,
            )
        })
        .unwrap()
        .join()
        .unwrap();

    assert!(elapsed < std::time::Duration::from_secs(5), "took {elapsed:?}");
    assert!(paren);
    assert!(tuple);
}

#[test]
fn a_rest_pattern_in_parentheses_is_a_tuple() {
    let pattern: Pattern = moxy::parse!("(..)").unwrap();
    assert!(matches!(pattern, Pattern::Tuple(_)));

    let pattern: Pattern = moxy::parse!("(a)").unwrap();
    assert!(matches!(pattern, Pattern::Paren(_)));

    let pattern: Pattern = moxy::parse!("(a,)").unwrap();
    assert!(matches!(pattern, Pattern::Tuple(_)));
}

#[test]
fn reference_patterns_before_an_error_fail_in_linear_time() {
    // Peeking for a range skipped the rest of the input as an expression at
    // every `&`, and an expression's `let` skipped a pattern again
    let source = format!("fn f() {{ {} {{ }} }}", format!("let {} ", "&".repeat(12)).repeat(6));
    let start = std::time::Instant::now();
    assert!(moxy::parse!(source as moxy::ast::File).is_err());
    assert!(
        start.elapsed() < std::time::Duration::from_secs(5),
        "took {:?}",
        start.elapsed()
    );

    // A range after `&` is the referenced pattern, as in syn
    let pattern: Pattern = moxy::parse!("&1..=2").unwrap();
    assert!(matches!(pattern, Pattern::Reference(_)));
}

#[test]
fn let_and_parameter_patterns_have_no_top_level_alternation() {
    for source in [
        "fn f() { let (a | b) = c; }",
        "fn f((a | b): u8) {}",
        "fn f() { if let a | b = c {} }",
    ] {
        assert!(moxy::parse!(source as moxy::ast::File).is_ok(), "`{source}`");
    }

    for source in ["fn f() { let a | b = c; }", "fn f() { let | a = c; }", "fn f(a | b: u8) {}"] {
        assert!(moxy::parse!(source as moxy::ast::File).is_err(), "`{source}`");
    }
}

#[test]
fn a_binding_subpattern_has_no_top_level_alternation() {
    let pattern: Pattern = moxy::parse!("a @ 1 | a @ 2").unwrap();
    let Pattern::Or(or) = &pattern else {
        panic!("not an or-pattern: {}", pattern.to_token_stream());
    };
    let cases: Vec<String> = or.cases.iter().map(|case| case.to_token_stream().to_string()).collect();
    assert_eq!(cases, ["a @ 1", "a @ 2"]);
    assert!(
        or.cases
            .iter()
            .all(|case| matches!(case, Pattern::Ident(ident) if ident.subpat.is_some()))
    );

    let pattern: Pattern = moxy::parse!("a @ (1 | 2)").unwrap();
    assert!(matches!(&pattern, Pattern::Ident(ident) if ident.subpat.is_some()));

    let closure: moxy::ast::Expr = moxy::parse!("|a @ S { b, .. }| x").unwrap();
    assert!(matches!(closure, moxy::ast::Expr::Closure(_)));
}

#[test]
fn only_closure_parameters_put_attributes_on_a_pattern() {
    let closure: moxy::ast::Expr = moxy::parse!("|#[a] x, #[b] (y, _): u8| x").unwrap();
    assert_eq!(closure.to_token_stream().to_string(), "| # [a] x , # [b] (y , _) : u8 | x");

    for source in [
        "fn f() { match x { #[a] y => 1 } }",
        "fn f(#[a] x: u8) {}",
        "fn f() { let S { #[a] x } = y; }",
    ] {
        assert!(moxy::parse!(source as moxy::ast::File).is_ok(), "`{source}`");
    }

    for source in ["#[a] x", "(#[a] x, y)", "[#[a] x]", "S(#[a] x)"] {
        assert!(moxy::parse!(source as Pattern).is_err(), "`{source}`");
    }
}
