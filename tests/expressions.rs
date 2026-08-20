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
        let primary = expression.as_primary().unwrap();
        assert_eq!(
            [
                primary.is_tuple(),
                primary.is_array(),
                primary.is_repeat(),
                primary.is_struct()
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
    assert!(conditional.is_block());
    assert_eq!(moxy::fmt!(&conditional).unwrap(), "if ready {\n\tyes()\n} else {\n\tno()\n}");

    let matching: Expr = moxy::parse!("match value { Some(x) if x > 0 => x, None => 0, _ => 1 }").unwrap();
    assert!(matching.is_block());
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
            [expression.is_primary(), expression.is_block(), expression.is_jump()],
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
        assert!(expression.is_block());
        assert!(!expression.span().is_empty());
        assert!(!expression.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}

#[test]
fn while_for_break_continue_and_yield_complete_the_pipeline() {
    for (source, expected, is_jump) in [
        (
            "while let Some(value) = next() { consume(value); }",
            "while let Some(value) = next() {\n\tconsume(value);\n}",
            false,
        ),
        (
            "for item in items { consume(item); }",
            "for item in items {\n\tconsume(item);\n}",
            false,
        ),
        ("break value", "break value", true),
        ("continue", "continue", true),
        ("yield value", "yield value", true),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert_eq!(expression.is_jump(), is_jump);
        assert!(!expression.span().is_empty());
        assert!(!expression.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}

#[test]
fn postfix_expression_families_complete_the_pipeline() {
    for (source, expected) in [
        ("function(a, b)", "function(a, b)"),
        ("object.method(a, b)", "object\n.method(a, b)"),
        ("object.field", "object.field"),
        ("tuple.0", "tuple.0"),
        ("array[index]", "array[index]"),
        ("future.await", "future.await"),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert!(expression.is_postfix());
        assert!(!expression.span().is_empty());
        assert!(!expression.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}

#[cfg(feature = "serde")]
#[test]
fn expression_syntax_has_concrete_serde_tags_and_exact_output() {
    for (source, outer, inner, expected) in [
        ("-value", "Unary", "Unary", "-value"),
        ("target = value", "Binary", "Assign", "target = value"),
        ("function(a)", "Postfix", "Call", "function(a)"),
        ("if ready { yes() }", "Block", "If", "if ready {\n\tyes()\n}"),
        ("return value", "Jump", "Return", "return value"),
        ("[a, b]", "Primary", "Array", "[a, b]"),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        let serialized = serde_json::to_value(&expression).unwrap();
        assert!(serialized.get(outer).and_then(|value| value.get(inner)).is_some());
        assert!(!expression.span().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}

#[test]
fn unary_binary_and_postfix_public_variants_match_rendered_syntax() {
    for (source, expected, kind) in [
        ("&mut value", "&mut value", 0),
        ("-value", "-value", 1),
        ("value as u64", "value as u64", 2),
        ("value?", "value?", 3),
    ] {
        let mut expression: Expr = moxy::parse!(source).unwrap();
        let unary = expression.as_unary().unwrap();
        assert_eq!(
            [unary.is_reference(), unary.is_unary(), unary.is_cast(), unary.is_try()],
            std::array::from_fn(|index| index == kind)
        );
        assert_eq!(
            [
                unary.as_reference().is_some(),
                unary.as_unary().is_some(),
                unary.as_cast().is_some(),
                unary.as_try().is_some(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert!(unary.attrs().is_empty());
        assert!(unary.clone().into_expr().is_unary());
        assert!(expression.attrs_mut().unwrap().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }

    for (source, expected, kind) in [
        ("a + b", "a + b", 0),
        ("target = value", "target = value", 1),
        ("target += value", "target += value", 2),
        ("start..=end", "start..=end", 3),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        let binary = expression.as_binary().unwrap();
        assert_eq!(
            [
                binary.is_binary(),
                binary.is_assign(),
                binary.is_assign_op(),
                binary.is_range(),
                binary.is_type(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert_eq!(
            [
                binary.as_binary().is_some(),
                binary.as_assign().is_some(),
                binary.as_assign_op().is_some(),
                binary.as_range().is_some(),
                binary.as_type().is_some(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert!(binary.attrs().is_empty());
        assert!(binary.clone().into_expr().is_binary());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }

    for (source, expected, kind) in [
        ("function(value)", "function(value)", 0),
        ("object.method(value)", "object\n.method(value)", 1),
        ("object.field", "object.field", 2),
        ("array[index]", "array[index]", 3),
        ("future.await", "future.await", 4),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        let postfix = expression.as_postfix().unwrap();
        assert_eq!(
            [
                postfix.is_call(),
                postfix.is_method_call(),
                postfix.is_field(),
                postfix.is_index(),
                postfix.is_await(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert_eq!(
            [
                postfix.as_call().is_some(),
                postfix.as_method_call().is_some(),
                postfix.as_field().is_some(),
                postfix.as_index().is_some(),
                postfix.as_await().is_some(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert!(postfix.attrs().is_empty());
        assert!(postfix.clone().into_expr().is_postfix());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}

#[test]
fn block_jump_and_primary_public_variants_match_rendered_syntax() {
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
        let block = expression.as_block().unwrap();
        assert_eq!(
            [
                block.is_brace(),
                block.is_if(),
                block.is_while(),
                block.is_for_loop(),
                block.is_loop(),
                block.is_match(),
                block.is_async(),
                block.is_unsafe(),
                block.is_const(),
                block.is_try_block(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert_eq!(
            [
                block.as_brace().is_some(),
                block.as_if().is_some(),
                block.as_while().is_some(),
                block.as_for_loop().is_some(),
                block.as_loop().is_some(),
                block.as_match().is_some(),
                block.as_async().is_some(),
                block.as_unsafe().is_some(),
                block.as_const().is_some(),
                block.as_try_block().is_some(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert!(block.attrs().is_empty());
        assert!(block.clone().into_expr().is_block());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }

    for (source, expected, kind) in [
        ("return value", "return value", 0),
        ("break value", "break value", 1),
        ("continue", "continue", 2),
        ("yield value", "yield value", 3),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        let jump = expression.as_jump().unwrap();
        assert_eq!(
            [jump.is_return(), jump.is_break(), jump.is_continue(), jump.is_yield()],
            std::array::from_fn(|index| index == kind)
        );
        assert_eq!(
            [
                jump.as_return().is_some(),
                jump.as_break().is_some(),
                jump.as_continue().is_some(),
                jump.as_yield().is_some(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert!(jump.attrs().is_empty());
        assert!(jump.clone().into_expr().is_jump());
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
        let primary = expression.as_primary().unwrap();
        assert_eq!(
            [
                primary.is_lit(),
                primary.is_path(),
                primary.is_struct(),
                primary.is_closure(),
                primary.is_tuple(),
                primary.is_array(),
                primary.is_repeat(),
                primary.is_let(),
                primary.is_paren(),
                primary.is_group(),
                primary.is_macro(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert_eq!(
            [
                primary.as_lit().is_some(),
                primary.as_path().is_some(),
                primary.as_struct().is_some(),
                primary.as_closure().is_some(),
                primary.as_tuple().is_some(),
                primary.as_array().is_some(),
                primary.as_repeat().is_some(),
                primary.as_let().is_some(),
                primary.as_paren().is_some(),
                primary.as_group().is_some(),
                primary.as_macro().is_some(),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert!(primary.attrs().is_empty());
        assert!(primary.clone().into_expr().is_primary());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}
