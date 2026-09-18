use moxy::ast::Expr;
use moxy::token::{Spanner, ToTokenStream};

#[test]
fn collection_and_struct_expressions_complete_the_pipeline() {
    for (source, expected, expected_kind) in [
        ("(first, second, third)", "(first, second, third)", 0),
        ("[first, second, third]", "[first, second, third]", 1),
        ("[value; 4]", "[value; 4]", 2),
        ("Point { x: 1, y, ..base }", "Point {\n\tx: 1,\n\ty,\n\t..base\n}", 3),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert_eq!(
            [
                expression.is_tuple(),
                expression.is_array(),
                expression.is_repeat(),
                expression.is_struct()
            ],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert!(!expression.span().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}

#[test]
fn control_flow_expressions_preserve_conditions_arms_and_bodies() {
    let conditional: Expr = moxy::parse!("if ready { yes() } else { no() }").unwrap();
    assert!(conditional.is_if());
    assert_eq!(moxy::fmt!(&conditional).unwrap(), "if ready {\n\tyes()\n} else {\n\tno()\n}");

    let matching: Expr = moxy::parse!("match value { Some(x) if x > 0 => x, None => 0, _ => 1 }").unwrap();
    assert!(matching.is_match());
    assert!(!matching.span().is_empty());
    assert_eq!(
        moxy::fmt!(&matching).unwrap(),
        "match value {\n\tSome(x) if x > 0 => x,\n\tNone => 0,\n\t_ => 1,\n}"
    );
}

#[test]
fn closures_loops_and_jumps_render_exact_rust() {
    for (source, expected, expected_kind) in [
        ("move |x: i32| -> i32 { x + 1 }", "move |x: i32| -> i32 {\n\tx + 1\n}", 0),
        ("loop { break 1 }", "loop {\n\tbreak 1\n}", 1),
        ("return value", "return value", 2),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert_eq!(
            [expression.is_closure(), expression.is_loop(), expression.is_return()],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert!(!expression.span().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}

#[test]
fn malformed_expressions_report_syntax_errors() {
    let result: Result<Expr, _> = moxy::parse!("if ready");
    assert_eq!(result.unwrap_err().message(), "expected `brace` delimiter");
}

#[test]
fn async_const_unsafe_and_try_blocks_complete_the_pipeline() {
    for (source, expected) in [
        ("async move { work().await? }", "async move {\n\twork().await?\n}"),
        ("const { 1 + 2 }", "const {\n\t1 + 2\n}"),
        ("unsafe { call() }", "unsafe {\n\tcall()\n}"),
        ("try { operation()? }", "try {\n\toperation()?\n}"),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert!(expression.is_async() || expression.is_const() || expression.is_unsafe() || expression.is_try_block());
        assert!(!expression.span().is_empty());
        assert!(!expression.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}

#[test]
fn while_for_break_continue_and_yield_complete_the_pipeline() {
    for (source, expected, expected_kind) in [
        (
            "while let Some(value) = next() { consume(value); }",
            "while let Some(value) = next() {\n\tconsume(value);\n}",
            0,
        ),
        (
            "for item in items { consume(item); }",
            "for item in items {\n\tconsume(item);\n}",
            1,
        ),
        ("break value", "break value", 2),
        ("continue", "continue", 3),
        ("yield value", "yield value", 4),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert_eq!(
            [
                expression.is_while(),
                expression.is_for_loop(),
                expression.is_break(),
                expression.is_continue(),
                expression.is_yield(),
            ],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert!(!expression.span().is_empty());
        assert!(!expression.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}

#[test]
fn postfix_expression_families_complete_the_pipeline() {
    for (source, expected, expected_kind) in [
        ("function(a, b)", "function(a, b)", 0),
        ("object.method(a, b)", "object.method(a, b)", 1),
        ("object.field", "object.field", 2),
        ("tuple.0", "tuple.0", 2),
        ("array[index]", "array[index]", 3),
        ("future.await", "future.await", 4),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert_eq!(
            [
                expression.is_call(),
                expression.is_method_call(),
                expression.is_field(),
                expression.is_index(),
                expression.is_await(),
            ],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert!(!expression.span().is_empty());
        assert!(!expression.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}

#[cfg(feature = "serde")]
#[test]
fn expression_syntax_has_concrete_serde_tags_and_exact_output() {
    for (source, variant, expected) in [
        ("-value", "Unary", "-value"),
        ("target = value", "Assign", "target = value"),
        ("function(a)", "Call", "function(a)"),
        ("if ready { yes() }", "If", "if ready {\n\tyes()\n}"),
        ("return value", "Return", "return value"),
        ("[a, b]", "Array", "[a, b]"),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        let serialized = serde_json::to_value(&expression).unwrap();
        assert!(serialized.get(variant).is_some());
        assert!(!expression.span().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}

#[test]
fn expression_public_variants_match_rendered_syntax() {
    for (source, expected, kind) in [
        ("&mut value", "&mut value", 0),
        ("-value", "-value", 1),
        ("value as u64", "value as u64", 2),
        ("value?", "value?", 3),
    ] {
        let mut expression: Expr = moxy::parse!(source).unwrap();
        assert_eq!(
            [
                expression.is_reference(),
                expression.is_unary(),
                expression.is_cast(),
                expression.is_try(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert_eq!(
            [
                expression.as_reference().is_some(),
                expression.as_unary().is_some(),
                expression.as_cast().is_some(),
                expression.as_try().is_some(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert!(expression.attrs_mut().unwrap().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }

    for (source, expected, kind) in [
        ("a + b", "a + b", 0),
        ("target = value", "target = value", 1),
        ("start..=end", "start..=end", 2),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert_eq!(
            [expression.is_binary(), expression.is_assign(), expression.is_range(),],
            std::array::from_fn(|index| index == kind)
        );
        assert_eq!(
            [
                expression.as_binary().is_some(),
                expression.as_assign().is_some(),
                expression.as_range().is_some(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert!(expression.attrs().unwrap().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }

    for (source, expected, kind) in [
        ("function(value)", "function(value)", 0),
        ("object.method(value)", "object.method(value)", 1),
        ("object.field", "object.field", 2),
        ("array[index]", "array[index]", 3),
        ("future.await", "future.await", 4),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert_eq!(
            [
                expression.is_call(),
                expression.is_method_call(),
                expression.is_field(),
                expression.is_index(),
                expression.is_await(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert_eq!(
            [
                expression.as_call().is_some(),
                expression.as_method_call().is_some(),
                expression.as_field().is_some(),
                expression.as_index().is_some(),
                expression.as_await().is_some(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert!(expression.attrs().unwrap().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}

#[test]
fn remaining_expression_public_variants_match_rendered_syntax() {
    for (source, expected, kind) in [
        ("{ value }", "{\n\tvalue\n}", 0),
        ("if ready { yes() }", "if ready {\n\tyes()\n}", 1),
        ("while ready { work(); }", "while ready {\n\twork();\n}", 2),
        (
            "for item in items { work(item); }",
            "for item in items {\n\twork(item);\n}",
            3,
        ),
        ("loop { break; }", "loop {\n\tbreak;\n}", 4),
        (
            "match value { Some(x) => x, _ => 0 }",
            "match value {\n\tSome(x) => x,\n\t_ => 0,\n}",
            5,
        ),
        ("async move { work().await }", "async move {\n\twork().await\n}", 6),
        ("unsafe { call() }", "unsafe {\n\tcall()\n}", 7),
        ("const { 1 }", "const {\n\t1\n}", 8),
        ("try { work()? }", "try {\n\twork()?\n}", 9),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert_eq!(
            [
                expression.is_block(),
                expression.is_if(),
                expression.is_while(),
                expression.is_for_loop(),
                expression.is_loop(),
                expression.is_match(),
                expression.is_async(),
                expression.is_unsafe(),
                expression.is_const(),
                expression.is_try_block(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert_eq!(
            [
                expression.as_block().is_some(),
                expression.as_if().is_some(),
                expression.as_while().is_some(),
                expression.as_for_loop().is_some(),
                expression.as_loop().is_some(),
                expression.as_match().is_some(),
                expression.as_async().is_some(),
                expression.as_unsafe().is_some(),
                expression.as_const().is_some(),
                expression.as_try_block().is_some(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert!(expression.attrs().unwrap().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }

    for (source, expected, kind) in [
        ("return value", "return value", 0),
        ("break value", "break value", 1),
        ("continue", "continue", 2),
        ("yield value", "yield value", 3),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert_eq!(
            [
                expression.is_return(),
                expression.is_break(),
                expression.is_continue(),
                expression.is_yield(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert_eq!(
            [
                expression.as_return().is_some(),
                expression.as_break().is_some(),
                expression.as_continue().is_some(),
                expression.as_yield().is_some(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert!(expression.attrs().unwrap().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }

    for (source, expected, kind) in [
        ("42", "42", 0),
        ("path::value", "path::value", 1),
        ("Point { x: 1 }", "Point {\n\tx: 1,\n}", 2),
        ("|x| x", "|x| x", 3),
        ("(a, b)", "(a, b)", 4),
        ("[a, b]", "[a, b]", 5),
        ("[value; 3]", "[value; 3]", 6),
        ("let Some(value) = option", "let Some(value) = option", 7),
        ("(value)", "(value)", 8),
        ("macro_call!(tokens)", "macro_call!(tokens)", 10),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert_eq!(
            [
                expression.is_lit(),
                expression.is_path(),
                expression.is_struct(),
                expression.is_closure(),
                expression.is_tuple(),
                expression.is_array(),
                expression.is_repeat(),
                expression.is_let(),
                expression.is_paren(),
                expression.is_group(),
                expression.is_macro(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert_eq!(
            [
                expression.as_lit().is_some(),
                expression.as_path().is_some(),
                expression.as_struct().is_some(),
                expression.as_closure().is_some(),
                expression.as_tuple().is_some(),
                expression.as_array().is_some(),
                expression.as_repeat().is_some(),
                expression.as_let().is_some(),
                expression.as_paren().is_some(),
                expression.as_group().is_some(),
                expression.as_macro().is_some(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert!(expression.attrs().unwrap().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}
