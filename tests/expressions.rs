use moxy::ast::{Expr, Postfix, Type};
use moxy::token::{Spanner, ToTokenStream};

#[test]
fn binary_and_unary_operators_complete_the_expression_pipeline() {
    for operator in [
        "<<=", ">>=", "+=", "-=", "*=", "/=", "%=", "^=", "&=", "|=", "&&", "||", "<<", ">>", "==", "!=", "<=", ">=", "+", "-",
        "*", "/", "%", "^", "&", "|", "<", ">",
    ] {
        let source = format!("(left) {operator} (right)");
        let expression: Expr = moxy::parse!(source).unwrap();
        assert!(expression.is_binary_chain());
        assert_eq!(moxy::fmt!(&expression).unwrap(), source);
    }

    for source in ["*value", "!value", "-value"] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert!(expression.is_unary());
        assert_eq!(moxy::fmt!(&expression).unwrap(), source);
    }
}

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
    assert!(conditional.is_if_chain());
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
            postfix_kinds(&expression),
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
        ("function(a)", "Postfix", "function(a)"),
        ("if ready { yes() }", "IfChain", "if ready {\n\tyes()\n}"),
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
                expression.is_cast_chain(),
                matches!(last_op(&expression), Some(Postfix::Try(_))),
            ],
            std::array::from_fn(|index| index == kind)
        );
        assert_eq!(
            [
                expression.as_reference().is_some(),
                expression.as_unary().is_some(),
                expression.as_cast_chain().is_some(),
                expression.as_postfix().is_some(),
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
            [expression.is_binary_chain(), expression.is_assign(), expression.is_range(),],
            std::array::from_fn(|index| index == kind)
        );
        assert_eq!(
            [
                expression.as_binary_chain().is_some(),
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
        assert_eq!(postfix_kinds(&expression), std::array::from_fn(|index| index == kind));
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
                expression.is_if_chain(),
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
                expression.as_if_chain().is_some(),
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

#[test]
fn a_less_than_after_a_path_is_a_comparison() {
    for (source, tokens) in [
        ("a < b", "a < b"),
        ("n < 2.0 && m > 3", "n < 2.0 && m > 3"),
        ("x::y < z", "x :: y < z"),
        ("f::<u8>(x) < g", "f :: < u8 > (x) < g"),
        ("Vec::<u8>::new()", "Vec :: < u8 > :: new ()"),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(expression.to_token_stream().to_string(), tokens);
    }

    let expression: Expr = moxy::parse!("if n < 2.0 { return 1; }").unwrap();
    assert!(expression.as_if_chain().unwrap().cond.is_binary_chain());
}

#[test]
fn block_bodied_match_arms_end_without_a_comma() {
    let expression: Expr = moxy::parse!("match x { (a, b) => { 1 } (c, d) => 2, e => if f { 3 } [g] => 5 }").unwrap();
    let arms = &expression.as_match().unwrap().arms.inner;
    assert_eq!(arms.len(), 4);
    assert_eq!(
        expression.to_token_stream().to_string(),
        "match x {(a , b) => {1} (c , d) => 2 , e => if f {3} [g] => 5}"
    );

    // A method call, field access, or `?` continues the block
    let expression: Expr = moxy::parse!("match x { a => { y }.len(), b => { z }?, }").unwrap();
    let arms = &expression.as_match().unwrap().arms.inner;
    assert!(matches!(last_op(&arms[0].body), Some(Postfix::MethodCall(_))));
    assert!(matches!(last_op(&arms[1].body), Some(Postfix::Try(_))));

    let expression: Expr = moxy::parse!("match x { (1, _) => const { 1 } (_, _) => 3 }").unwrap();
    assert_eq!(expression.as_match().unwrap().arms.inner.len(), 2);
}

#[test]
fn else_branches_end_at_their_block() {
    let expression: Expr =
        moxy::parse!("match x { e => if f { 3 } else { 4 }, [g] => if h { 5 } else if i { 6 } else { 7 } }").unwrap();
    assert_eq!(expression.as_match().unwrap().arms.inner.len(), 2);

    // The index applies to the whole `if`, not to the else block
    let expression: Expr = moxy::parse!("if a { 1 } else { 2 }[0]").unwrap();
    let chain = expression.as_postfix().unwrap();
    assert!(matches!(chain.ops[..], [Postfix::Index(_)]));
    let base = &chain.base;
    assert!(base.as_if_chain().unwrap().else_branch.is_some());

    let error = moxy::parse!("if a { 1 } else 2" as Expr).err().unwrap();
    assert_eq!(error.message(), "expected `{` or `if` after `else`");
}

#[test]
fn a_cast_type_ends_before_a_binary_plus() {
    let expression: Expr = moxy::parse!("a as usize + 1").unwrap();
    assert!(expression.is_binary_chain());
    assert_eq!(expression.to_token_stream().to_string(), "a as usize + 1");

    let expression: Expr = moxy::parse!("x as Box<dyn A + Send>").unwrap();
    assert!(expression.is_cast_chain());
}

#[test]
fn closure_parameters_take_reference_patterns() {
    for (source, tokens) in [
        ("|&b| b == n", "| & b | b == n"),
        ("|&(a, b), &mut c| a", "| & (a , b) , & mut c | a"),
        ("match x { &C | &D => 2 }", "match x {& C | & D => 2}"),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(expression.to_token_stream().to_string(), tokens);
    }
}

#[test]
fn qualified_paths_keep_their_qualifier_in_expressions_and_patterns() {
    for (source, tokens) in [
        ("<T as a::U>::f(x)", "< T as a :: U > :: f (x)"),
        ("<Vec<u8>>::new()", "< Vec < u8 >> :: new ()"),
        ("<T as U>::C { x: 1 }", "< T as U > :: C {x : 1}"),
        (
            "match x { <T as U>::A => 1, <T as U>::B { x } => 2, <T as U>::C(y) => 3 }",
            "match x {< T as U > :: A => 1 , < T as U > :: B {x} => 2 , < T as U > :: C (y) => 3}",
        ),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(expression.to_token_stream().to_string(), tokens);
    }
}

#[test]
fn raw_borrows_keep_the_raw_keyword() {
    for (source, tokens) in [("&raw const x", "& raw const x"), ("&raw mut x", "& raw mut x")] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert!(expression.is_raw_addr());
        assert_eq!(expression.to_token_stream().to_string(), tokens);
    }
}

#[test]
fn break_keeps_its_label() {
    let expression: Expr = moxy::parse!("'a: loop { break 'a; }").unwrap();
    let Some(moxy::ast::Stmt::Expr(statement, _)) = expression.as_loop().unwrap().body.stmts.inner.first() else {
        panic!("expected an expression statement");
    };
    let label = statement.as_break().unwrap().label.as_ref().unwrap();
    assert_eq!(label.to_token_stream().to_string(), "'a");
}

#[test]
fn comparisons_do_not_chain() {
    for source in ["a < b > c", "a == b == c", "x < y && a < b < c"] {
        let error = moxy::parse!(source as Expr)
            .err()
            .unwrap_or_else(|| panic!("`{source}` parsed"));
        assert_eq!(error.message(), "comparison operators cannot be chained", "`{source}`");
    }

    for source in ["a < b && c > d", "a + b < c * d", "a == b || c != d"] {
        let expression: Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert!(expression.is_binary_chain());
    }
}

#[test]
fn a_cast_type_ends_before_a_comparison() {
    for source in ["a as u8 != b", "x.len() as u64 <= max", "a as i64 >= b", "a as &u8 + 1"] {
        let expression: Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert!(expression.is_binary_chain(), "`{source}`");
        assert!(
            expression.as_binary_chain().unwrap().operands[0].is_cast_chain(),
            "`{source}`"
        );
        assert_eq!(expression.to_token_stream().to_string(), lexed.to_string());
    }

    let ty: Type = moxy::parse!("Vec<<T as A>::B>").unwrap();
    assert_eq!(ty.to_token_stream().to_string(), "Vec << T as A > :: B >");
}

#[test]
fn an_open_range_ends_before_a_loop_or_branch_body() {
    for source in [
        "for i in 0.. { a(i) }",
        "if x == (..) { a } else { b }",
        "while let Some(v) = f(a..) { g() }",
        "match a.. { _ => {} }",
    ] {
        let expression: Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(expression.to_token_stream().to_string(), lexed.to_string());
    }

    let expression: Expr = moxy::parse!("for i in 0.. { a(i) }").unwrap();
    let range = expression.as_for_loop().unwrap().expr.as_range().unwrap();
    assert!(range.start.is_some());
    assert!(range.end.is_none());

    // Outside a loop or branch header, a block is the range's end
    let expression: Expr = moxy::parse!("0..{ 5 }").unwrap();
    assert!(expression.as_range().unwrap().end.is_some());
}

#[test]
fn an_absent_operand_ends_before_a_separator() {
    for (source, tokens) in [
        (
            "match y { None => return, Some(v) => v }",
            "match y {None => return , Some (v) => v}",
        ),
        (
            "loop { match a { _ => break, b => continue } }",
            "loop {match a {_ => break , b => continue}}",
        ),
        ("f(return, 1)", "f (return , 1)"),
        ("S { a: return, b }", "S {a : return , b}"),
        ("(yield, 1)", "(yield , 1)"),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(expression.to_token_stream().to_string(), tokens);
    }

    let expression: Expr = moxy::parse!("match y { None => return, Some(v) => v }").unwrap();
    let arms = &expression.as_match().unwrap().arms.inner;
    assert_eq!(arms.len(), 2);
    assert!(arms[0].body.as_return().unwrap().expr.is_none());

    let expression: Expr = moxy::parse!("f(return, 1)").unwrap();
    let Some(Postfix::Call(call)) = last_op(&expression) else {
        panic!("expected a call");
    };
    let args = &call.args.inner;
    assert_eq!(args.len(), 2);
    assert!(args.iter().next().unwrap().as_return().unwrap().expr.is_none());

    let expression: Expr = moxy::parse!("(yield, 1)").unwrap();
    assert!(expression.is_tuple());
}

#[test]
fn a_float_after_a_field_dot_is_two_tuple_indices() {
    for source in ["a.0.1", "self.0.0", "a.1.0.2", "NonZeroI32::new(self.0.0)"] {
        let expression: Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(expression.to_token_stream().to_string(), lexed.to_string());
    }

    let expression: Expr = moxy::parse!("a.0.1").unwrap();
    let members: Vec<String> = expression
        .as_postfix()
        .unwrap()
        .ops
        .iter()
        .map(|op| match op {
            Postfix::Field(field) => field.member.to_token_stream().to_string(),
            _ => panic!("expected a field access"),
        })
        .collect();
    assert_eq!(members, ["0", "1"]);

    // A float the compiler gives as one token splits the same way
    let tokens: moxy::token::TokenStream = [
        moxy::token::TokenTree::Ident(moxy::token::Ident::new("a")),
        moxy::token::TokenTree::Punct(moxy::token::Dot::default().into()),
        moxy::token::TokenTree::Literal(moxy::token::Lit::f64_unsuffixed(0.1)),
    ]
    .into_iter()
    .collect();
    let expression: Expr = moxy::parse!(tokens).unwrap();
    assert!(matches!(
        expression.as_postfix().unwrap().ops[..],
        [Postfix::Field(_), Postfix::Field(_)]
    ));

    // After a range, `0.1` stays a float
    let expression: Expr = moxy::parse!("a..0.1").unwrap();
    assert!(expression.as_range().unwrap().end.as_ref().unwrap().is_lit());
}

#[test]
fn a_token_that_starts_no_expression_is_an_error() {
    for source in ["a +", "+ a", "f(,)", "f(a,,b)", "[a;]", "(,)", "&raw const", ";"] {
        let error = moxy::parse!(source as Expr)
            .err()
            .unwrap_or_else(|| panic!("`{source}` parsed"));
        assert!(!error.message().is_empty(), "`{source}`");
    }

    for (source, tokens) in [
        ("return.x", "return . x"),
        ("return?", "return ?"),
        ("S { a, .. } = b", "S {a , ..} = b"),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(expression.to_token_stream().to_string(), tokens);
    }

    let expression: Expr = moxy::parse!("return.x").unwrap();
    assert!(expression.as_postfix().unwrap().base.as_return().unwrap().expr.is_none());

    let expression: Expr = moxy::parse!("S { a, .. } = b").unwrap();
    let body = &expression.as_assign().unwrap().left.as_struct().unwrap().body.inner;
    assert!(body.rest.as_ref().unwrap().1.is_none());
}

#[test]
fn skipping_an_expression_ends_where_parsing_it_ends() {
    use moxy::ast::{Cursor, Parse, Parser};

    for source in [
        "a.b < c",
        "continue 'a",
        "break 'a 1 + 2",
        "a += b + c",
        "match a { x if x > 1 => 1, _ => 2 }",
        "a < b == c",
        "a.b::<c>",
    ] {
        let tokens: moxy::token::TokenStream = source.parse().unwrap();
        let parser = Parser::from_tokens(&tokens);
        let parsed = Expr::parse(&parser).ok().map(|_| parser.remaining());
        let skipped = Expr::skip(Cursor::from_tokens(&tokens)).map(|cursor| cursor.remaining());
        assert_eq!(skipped, parsed, "`{source}`");
    }
}

#[test]
fn a_closure_with_a_return_type_takes_a_block_and_an_inclusive_range_an_end() {
    let expression: Expr = moxy::parse!("|| -> u8 { 1 }.x").unwrap();
    assert!(expression.as_postfix().unwrap().base.is_closure());

    for (source, message) in [
        ("|x: u8| -> u8 x", "expected a block after the closure's return type"),
        ("a..=", "an inclusive range needs an end"),
        ("..=", "an inclusive range needs an end"),
    ] {
        let error = moxy::parse!(source as Expr)
            .err()
            .unwrap_or_else(|| panic!("`{source}` parsed"));
        assert_eq!(error.message(), message, "`{source}`");
    }
}

#[test]
fn any_unsuffixed_integer_after_a_field_dot_is_a_tuple_index() {
    for (source, emitted) in [
        ("a.0_0", "a . 0_0"),
        ("a.0x1", "a . 0x1"),
        ("a.99999999999", "a . 99999999999"),
        ("x.0. 1", "x . 0 . 1"),
        ("x.0.1", "x . 0 . 1"),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert!(
            matches!(last_op(&expression), Some(Postfix::Field(field)) if field.member.is_unnamed()),
            "`{source}`"
        );
        assert_eq!(expression.to_token_stream().to_string(), emitted);
    }

    // A number with an exponent names a member too, as rustc's parser reads it
    for (source, emitted) in [("a.1e5", "a . 1e5"), ("a.0.1e1", "a . 0 . 1e1"), ("a.1E2", "a . 1E2")] {
        let expression: Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(expression.to_token_stream().to_string(), emitted);
    }

    for source in ["x.0u8", "x.0.", "x.1e5f32", "x.1E+2", "x.1.5e-3"] {
        assert!(moxy::parse!(source as Expr).is_err(), "`{source}`");
    }

    // A field name in a struct literal or pattern is an integer only
    assert_eq!(
        moxy::parse!("S { 0x1: a }" as Expr).unwrap().to_token_stream().to_string(),
        "S {0x1 : a}"
    );

    for source in ["S { 1e5: a }", "S { 1E2: a }"] {
        assert!(moxy::parse!(source as Expr).is_err(), "`{source}`");
        assert!(moxy::parse!(source as moxy::ast::Pattern).is_err(), "`{source}`");
    }
}

#[test]
fn the_reserved_keyword_abstract_is_no_identifier() {
    for source in ["abstract", "abstract + 1", "a.abstract"] {
        assert!(moxy::parse!(source as Expr).is_err(), "`{source}`");
    }

    let expression: Expr = moxy::parse!("r#abstract").unwrap();
    assert_eq!(expression.to_token_stream().to_string(), "r#abstract");
}

#[test]
fn an_optional_operand_ends_at_the_body_of_a_condition() {
    for source in ["if break {}", "while break 'l {}", "match return {}", "for x in break {}"] {
        let expression: Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let emitted = expression.to_token_stream().to_string();
        assert!(emitted.ends_with("{}"), "`{source}`: {emitted}");
    }

    let expression: Expr = moxy::parse!("return {}").unwrap();
    assert!(matches!(expression, Expr::Return(value) if value.expr.is_some()));
}

#[test]
fn a_match_arm_ends_with_a_comma_unless_its_body_is_block_like() {
    for source in [
        "match a { b => {} d => e }",
        "match a { b => c, d => e }",
        "match a { b => c }",
        "match a { b => if x {} else {} c => d }",
    ] {
        assert!(moxy::parse!(source as Expr).is_ok(), "`{source}`");
    }

    for source in [
        "match a { b => c d => e }",
        "match a { b => x? d => e }",
        "match a { b => {}.x d => e }",
    ] {
        assert!(moxy::parse!(source as Expr).is_err(), "`{source}`");
    }
}

// The last postfix operation of an expression that is a postfix chain
fn last_op(expression: &Expr) -> Option<&Postfix> {
    expression.as_postfix().and_then(|chain| chain.ops.last())
}

// Which of a call, method call, field access, index, and await ends a chain
fn postfix_kinds(expression: &Expr) -> [bool; 5] {
    let op = last_op(expression);
    [
        matches!(op, Some(Postfix::Call(_))),
        matches!(op, Some(Postfix::MethodCall(_))),
        matches!(op, Some(Postfix::Field(_))),
        matches!(op, Some(Postfix::Index(_))),
        matches!(op, Some(Postfix::Await(_))),
    ]
}

#[cfg(feature = "serde")]
#[test]
fn chain_nodes_serialize_every_step_and_operator() {
    use serde_json::json;

    let path = |name: &str| json!({"Path": {"attrs": [], "qself": null, "path": {"colon": null, "segments": [{"ident": name, "args": "None"}]}}});
    let ty =
        |name: &str| json!({"Path": {"qself": null, "path": {"colon": null, "segments": [{"ident": name, "args": "None"}]}}});
    let block = json!({"inner_attrs": [], "stmts": {"style": "brace", "inner": []}});

    for (source, expected) in [
        (
            "a - b / c",
            json!({"BinaryChain": {"attrs": [], "operands": [
                path("a"),
                {"Sub": "-"},
                {"BinaryChain": {"attrs": [], "operands": [path("b"), {"Div": "/"}, path("c")]}},
            ]}}),
        ),
        (
            "a.b(c)[d]?",
            json!({"Postfix": {"attrs": [], "base": path("a"), "ops": [
                {"MethodCall": {"dot": ".", "method": "b", "turbofish": null, "args": {"style": "paren", "inner": [path("c")]}}},
                {"Index": {"index": {"style": "bracket", "inner": path("d")}}},
                {"Try": {"question_punct": "?"}},
            ]}}),
        ),
        (
            "a as T as U",
            json!({"CastChain": {"attrs": [], "expr": path("a"), "casts": [["as", ty("T")], ["as", ty("U")]]}}),
        ),
        (
            "if a {} else if b {} else {}",
            json!({"IfChain": {
                "attrs": [],
                "if_keyword": "if",
                "cond": path("a"),
                "then_branch": block,
                "else_ifs": [{"else_keyword": "else", "if_keyword": "if", "cond": path("b"), "then_branch": block}],
                "else_keyword": "else",
                "else_branch": block,
            }}),
        ),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert_eq!(serde_json::to_value(&expression).unwrap(), expected, "{source}");
    }

    let item: moxy::ast::Item = moxy::parse!("use a::b::c;").unwrap();
    assert_eq!(
        serde_json::to_value(&item).unwrap(),
        json!({"Use": {
            "attrs": [],
            "vis": "Inherited",
            "use_keyword": "use",
            "tree": {"PathChain": {"prefix": null, "segments": ["a", "b"], "tree": {"Name": {"prefix": null, "ident": "c"}}}},
            "semi_punct": ";",
        }})
    );
}

#[test]
fn postfix_steps_tell_their_kind() {
    let expression: Expr = moxy::parse!("a(b).c(d).e[f]?.g.await").unwrap();
    let ops = &expression.as_postfix().unwrap().ops;
    let kinds: Vec<[bool; 6]> = ops
        .iter()
        .map(|op| {
            [
                op.is_call(),
                op.is_method_call(),
                op.is_field(),
                op.is_index(),
                op.is_try(),
                op.is_await(),
            ]
        })
        .collect();
    let accessed: Vec<[bool; 6]> = ops
        .iter()
        .map(|op| {
            [
                op.as_call().is_some(),
                op.as_method_call().is_some(),
                op.as_field().is_some(),
                op.as_index().is_some(),
                op.as_try().is_some(),
                op.as_await().is_some(),
            ]
        })
        .collect();
    let expected: Vec<[bool; 6]> = [0, 1, 2, 3, 4, 2, 5]
        .iter()
        .map(|&kind| std::array::from_fn(|index| index == kind))
        .collect();
    assert_eq!(kinds, expected);
    assert_eq!(accessed, expected);
    assert_eq!(ops[1].as_method_call().unwrap().method.text(), "c");
}

#[test]
fn visitors_see_each_chain_once_and_each_step_through_its_own_callback() {
    use moxy::ast::{ElseIf, ExprBinaryChain, ExprCastChain, ExprIfChain, PostfixMethodCall, UsePathChain, Visit};

    #[derive(Default)]
    struct Count([usize; 6]);

    impl<'ast> Visit<'ast> for Count {
        fn visit_expr_binary_chain(&mut self, node: &'ast ExprBinaryChain) {
            self.0[0] += 1;
            moxy::ast::walk_expr_binary_chain(self, node);
        }
        fn visit_expr_cast_chain(&mut self, node: &'ast ExprCastChain) {
            self.0[1] += 1;
            moxy::ast::walk_expr_cast_chain(self, node);
        }
        fn visit_expr_if_chain(&mut self, node: &'ast ExprIfChain) {
            self.0[2] += 1;
            moxy::ast::walk_expr_if_chain(self, node);
        }
        fn visit_else_if(&mut self, node: &'ast ElseIf) {
            self.0[3] += 1;
            moxy::ast::walk_else_if(self, node);
        }
        fn visit_postfix_method_call(&mut self, node: &'ast PostfixMethodCall) {
            self.0[4] += 1;
            moxy::ast::walk_postfix_method_call(self, node);
        }
        fn visit_use_path_chain(&mut self, node: &'ast UsePathChain) {
            self.0[5] += 1;
            moxy::ast::walk_use_path_chain(self, node);
        }
    }

    let expression: Expr = moxy::parse!("if a + b + c * d {} else if x.f().g() as u8 as u16 {} else if y {}").unwrap();
    let mut count = Count::default();
    count.visit_expr(&expression);
    // `a + b + c * d` is two chains, `+` and `*`
    assert_eq!(count.0, [2, 1, 1, 2, 2, 0]);

    let item: moxy::ast::Item = moxy::parse!("use a::b::{c::d, e};").unwrap();
    let mut count = Count::default();
    count.visit_item(&item);
    assert_eq!(count.0, [0, 0, 0, 0, 0, 2]);
}

#[test]
fn chains_split_off_their_last_step_as_expressions() {
    use moxy::ast::ExprPostfix;

    // `a.b()` rewritten as `b(a)`
    let Expr::Postfix(chain) = moxy::parse!("a.b()" as Expr).unwrap() else {
        panic!()
    };
    let (_, receiver, last) = chain.split_last().unwrap();
    let method = &last.as_method_call().unwrap().method;
    let call: Expr = moxy::parse!((format!("{}({})", method, receiver.to_token_stream()))).unwrap();
    assert_eq!(call.to_token_stream().to_string(), "b (a)");

    // The attributes of a chain come back apart; no operand takes them
    let Expr::Postfix(chain) = moxy::parse!("#[a] x.f().g()" as Expr).unwrap() else {
        panic!()
    };
    let (base, before, final_step) = chain.split_last_ref().unwrap();
    assert_eq!(
        [
            base.to_token_stream().to_string(),
            before.len().to_string(),
            final_step.to_token_stream().to_string()
        ],
        ["x", "1", ". g ()"]
    );
    let (attrs, receiver, _) = chain.split_last().unwrap();
    assert_eq!(
        [attrs.len().to_string(), receiver.to_token_stream().to_string()],
        ["1", "x . f ()"]
    );

    // An outer attribute before a sum belongs to its first operand
    let Expr::BinaryChain(chain) = moxy::parse!("#[a] x + y" as Expr).unwrap() else {
        panic!()
    };
    let (attrs, left, _, right) = chain.split_last().unwrap();
    assert_eq!(
        [
            attrs.len().to_string(),
            left.to_token_stream().to_string(),
            right.to_token_stream().to_string()
        ],
        ["0", "# [a] x", "y"]
    );

    let Expr::BinaryChain(chain) = moxy::parse!("x * y * z" as Expr).unwrap() else {
        panic!()
    };
    assert!(chain.as_pair().is_none());
    let (_, left, _, _) = chain.split_last().unwrap();
    let (x, _, y) = left.as_binary_chain().unwrap().as_pair().unwrap();
    assert_eq!([x.to_token_stream().to_string(), y.to_token_stream().to_string()], ["x", "y"]);

    // A chain without a step, which only direct construction makes, splits to
    // nothing instead of panicking
    let Expr::Postfix(mut empty) = moxy::parse!("x?" as Expr).unwrap() else {
        panic!()
    };
    empty.ops.clear();
    assert!(empty.split_last_ref().is_none());
    assert!(empty.split_last().is_none());

    let built = ExprPostfix::new(moxy::parse!("y" as Expr).unwrap(), last.clone());
    assert_eq!(built.to_token_stream().to_string(), "y . b ()");
}

#[test]
fn a_path_keyword_names_a_field_or_method_after_a_dot() {
    // As rustc's parser reads them; the compiler rejects them later
    for source in ["x.self", "x.Self", "x.super", "x.crate", "x.self()", "x.Self::<u8>()"] {
        let expression: Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(expression.to_token_stream().to_string(), lexed.to_string(), "`{source}`");
    }

    for source in ["x.fn", "S { self: 1 }"] {
        assert!(moxy::parse!(source as Expr).is_err(), "`{source}`");
    }
}

// A serializer that checks that every sequence, tuple, map, and struct
// announces its length and holds that many elements, as formats that write
// the length first need
#[cfg(feature = "serde")]
mod length_check {
    use serde::ser::{self, Serialize};

    type Error = serde::de::value::Error;

    pub struct Check;

    pub struct Compound {
        announced: Option<usize>,
        seen: usize,
    }

    impl Compound {
        fn element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
            self.seen += 1;
            value.serialize(Check)
        }

        fn finish(self) -> Result<(), Error> {
            match self.announced {
                Some(announced) if announced == self.seen => Ok(()),
                Some(announced) => Err(ser::Error::custom(format!(
                    "announced {announced} elements, held {}",
                    self.seen
                ))),
                None => Err(ser::Error::custom(format!("announced no length for {} elements", self.seen))),
            }
        }
    }

    macro_rules! leaves {
        ($($name:ident($($ty:ty),*)),*) => {
            $(fn $name(self, $(_: $ty),*) -> Result<(), Error> { Ok(()) })*
        };
    }

    macro_rules! compound {
        ($($trait:ident :: $method:ident $(($key:ident))?),*) => {
            $(impl ser::$trait for Compound {
                type Ok = ();
                type Error = Error;

                fn $method<T: Serialize + ?Sized>(&mut self, $($key: &'static str,)? value: &T) -> Result<(), Error> {
                    $(let _ = $key;)?
                    self.element(value)
                }

                fn end(self) -> Result<(), Error> {
                    self.finish()
                }
            })*
        };
    }

    compound!(
        SerializeSeq::serialize_element,
        SerializeTuple::serialize_element,
        SerializeTupleStruct::serialize_field,
        SerializeTupleVariant::serialize_field,
        SerializeStruct::serialize_field(key),
        SerializeStructVariant::serialize_field(key)
    );

    impl ser::SerializeMap for Compound {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), Error> {
            key.serialize(Check)
        }

        fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
            self.element(value)
        }

        fn end(self) -> Result<(), Error> {
            self.finish()
        }
    }

    impl ser::Serializer for Check {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Compound;
        type SerializeTuple = Compound;
        type SerializeTupleStruct = Compound;
        type SerializeTupleVariant = Compound;
        type SerializeMap = Compound;
        type SerializeStruct = Compound;
        type SerializeStructVariant = Compound;

        leaves!(
            serialize_bool(bool),
            serialize_i8(i8),
            serialize_i16(i16),
            serialize_i32(i32),
            serialize_i64(i64),
            serialize_u8(u8),
            serialize_u16(u16),
            serialize_u32(u32),
            serialize_u64(u64),
            serialize_f32(f32),
            serialize_f64(f64),
            serialize_char(char),
            serialize_str(&str),
            serialize_bytes(&[u8]),
            serialize_none(),
            serialize_unit(),
            serialize_unit_struct(&'static str),
            serialize_unit_variant(&'static str, u32, &'static str)
        );

        fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<(), Error> {
            value.serialize(Check)
        }

        fn serialize_newtype_struct<T: Serialize + ?Sized>(self, _: &'static str, value: &T) -> Result<(), Error> {
            value.serialize(Check)
        }

        fn serialize_newtype_variant<T: Serialize + ?Sized>(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            value: &T,
        ) -> Result<(), Error> {
            value.serialize(Check)
        }

        fn serialize_seq(self, announced: Option<usize>) -> Result<Compound, Error> {
            Ok(Compound { announced, seen: 0 })
        }

        fn serialize_tuple(self, len: usize) -> Result<Compound, Error> {
            self.serialize_seq(Some(len))
        }

        fn serialize_tuple_struct(self, _: &'static str, len: usize) -> Result<Compound, Error> {
            self.serialize_seq(Some(len))
        }

        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, len: usize) -> Result<Compound, Error> {
            self.serialize_seq(Some(len))
        }

        fn serialize_map(self, announced: Option<usize>) -> Result<Compound, Error> {
            self.serialize_seq(announced)
        }

        fn serialize_struct(self, _: &'static str, len: usize) -> Result<Compound, Error> {
            self.serialize_seq(Some(len))
        }

        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, len: usize) -> Result<Compound, Error> {
            self.serialize_seq(Some(len))
        }
    }
}

#[cfg(feature = "serde")]
#[test]
fn every_tree_of_the_parse_fixtures_serializes_with_announced_lengths() {
    use serde::Serialize;

    // A binary chain announced no length for its operands and operators
    for source in ["a - b / c", "a || b && c == d", "f(a, b)", "[a; 2]"] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert_eq!(
            expression.serialize(length_check::Check).map_err(|error| error.to_string()),
            Ok(()),
            "{source}"
        );
    }

    // Every item fixture is a file; the expression and type fixtures that are
    // files are checked too
    for (dir, all_files) in [
        ("fixtures/parse/item", true),
        ("fixtures/parse/expr", false),
        ("fixtures/parse/type", false),
    ] {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            let source = std::fs::read_to_string(&path).unwrap();

            match moxy::parse!(source as moxy::ast::File) {
                Ok(file) => {
                    let serialized = file.serialize(length_check::Check).map_err(|error| error.to_string());
                    assert_eq!(serialized, Ok(()), "{}", path.display());
                }
                Err(error) => assert!(!all_files, "{}: {error}", path.display()),
            }
        }
    }
}
