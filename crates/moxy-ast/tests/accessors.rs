use moxy_ast::{Expr, Item, Pattern, Stmt, Type};
use moxy_token::{Span, Spanner, ToTokenStream};

#[test]
fn expression_accessors_identify_every_public_expression_variant() {
    for (source, expected) in [
        ("&mut value", 0),
        ("a + b", 1),
        ("function(value)", 2),
        ("if ready { yes() } else { no() }", 3),
        ("return value", 4),
        ("Point { x: 1 }", 5),
    ] {
        let mut expression: Expr = moxy_token::parse!(source).unwrap();
        assert_eq!(
            [
                expression.is_unary(),
                expression.is_binary(),
                expression.is_postfix(),
                expression.is_block(),
                expression.is_jump(),
                expression.is_primary(),
            ],
            std::array::from_fn(|index| index == expected),
            "wrong expression category for {source}"
        );
        assert_eq!(
            [
                expression.as_unary().is_some(),
                expression.as_binary().is_some(),
                expression.as_postfix().is_some(),
                expression.as_block().is_some(),
                expression.as_jump().is_some(),
                expression.as_primary().is_some(),
            ],
            std::array::from_fn(|index| index == expected)
        );
        assert_eq!(expression.attrs().unwrap().len(), 0);
        assert_eq!(expression.attrs_mut().unwrap().len(), 0);
        assert!(!expression.to_token_stream().is_empty());
        assert!(!expression.span().is_empty());
    }

    let infer = Expr::Infer;
    assert!(infer.is_infer());
    assert!(!infer.is_verbatim());
    assert!(infer.attrs().is_none());
    let verbatim = Expr::Verbatim("custom tokens".parse().unwrap());
    assert!(verbatim.is_verbatim());
    assert!(!verbatim.is_infer());
    assert_eq!(verbatim.to_token_stream().to_string(), "custom tokens");
}

#[test]
fn nested_expression_accessors_report_concrete_variants_and_attributes() {
    for (source, expected) in [("&value", 0), ("-value", 1), ("value as u64", 2), ("value?", 3)] {
        let mut expression: Expr = moxy_token::parse!(source).unwrap();
        let unary = expression.as_unary().unwrap();
        assert_eq!(
            [unary.is_reference(), unary.is_unary(), unary.is_cast(), unary.is_try()],
            std::array::from_fn(|index| index == expected)
        );
        assert_eq!(
            [
                unary.as_reference().is_some(),
                unary.as_unary().is_some(),
                unary.as_cast().is_some(),
                unary.as_try().is_some(),
            ],
            std::array::from_fn(|index| index == expected)
        );
        assert!(unary.attrs().is_empty());
        assert!(expression.as_unary().unwrap().clone().into_expr().is_unary());
        let converted = match expected {
            0 => Expr::from(unary.as_reference().unwrap().clone()),
            1 => Expr::from(unary.as_unary().unwrap().clone()),
            2 => Expr::from(unary.as_cast().unwrap().clone()),
            3 => Expr::from(unary.as_try().unwrap().clone()),
            _ => unreachable!(),
        };
        assert_eq!(converted.to_token_stream(), expression.to_token_stream());
        assert!(expression.as_unary().unwrap().clone().into_expr().attrs().unwrap().is_empty());
        assert!(expression.attrs_mut().unwrap().is_empty());
    }

    for (source, expected) in [
        ("a + b", 0),
        ("target = value", 1),
        ("target += value", 2),
        ("start..=end", 3),
    ] {
        let mut expression: Expr = moxy_token::parse!(source).unwrap();
        let binary = expression.as_binary().unwrap();
        assert_eq!(
            [
                binary.is_binary(),
                binary.is_assign(),
                binary.is_assign_op(),
                binary.is_range(),
                binary.is_type(),
            ],
            std::array::from_fn(|index| index == expected)
        );
        assert_eq!(
            [
                binary.as_binary().is_some(),
                binary.as_assign().is_some(),
                binary.as_assign_op().is_some(),
                binary.as_range().is_some(),
                binary.as_type().is_some(),
            ],
            std::array::from_fn(|index| index == expected)
        );
        assert!(binary.attrs().is_empty());
        assert!(binary.clone().into_expr().is_binary());
        let converted = match expected {
            0 => Expr::from(binary.as_binary().unwrap().clone()),
            1 => Expr::from(binary.as_assign().unwrap().clone()),
            2 => Expr::from(binary.as_assign_op().unwrap().clone()),
            3 => Expr::from(binary.as_range().unwrap().clone()),
            _ => unreachable!(),
        };
        assert_eq!(converted.to_token_stream(), expression.to_token_stream());
        assert!(expression.attrs_mut().unwrap().is_empty());
    }

    for (source, expected) in [
        ("function(value)", 0),
        ("object.method(value)", 1),
        ("object.field", 2),
        ("array[index]", 3),
        ("future.await", 4),
    ] {
        let mut expression: Expr = moxy_token::parse!(source).unwrap();
        let postfix = expression.as_postfix().unwrap();
        assert_eq!(
            [
                postfix.is_call(),
                postfix.is_method_call(),
                postfix.is_field(),
                postfix.is_index(),
                postfix.is_await(),
            ],
            std::array::from_fn(|index| index == expected)
        );
        assert_eq!(
            [
                postfix.as_call().is_some(),
                postfix.as_method_call().is_some(),
                postfix.as_field().is_some(),
                postfix.as_index().is_some(),
                postfix.as_await().is_some(),
            ],
            std::array::from_fn(|index| index == expected)
        );
        assert!(postfix.attrs().is_empty());
        assert!(postfix.clone().into_expr().is_postfix());
        let converted = match expected {
            0 => Expr::from(postfix.as_call().unwrap().clone()),
            1 => Expr::from(postfix.as_method_call().unwrap().clone()),
            2 => Expr::from(postfix.as_field().unwrap().clone()),
            3 => Expr::from(postfix.as_index().unwrap().clone()),
            4 => Expr::from(postfix.as_await().unwrap().clone()),
            _ => unreachable!(),
        };
        assert_eq!(converted.to_token_stream(), expression.to_token_stream());
        assert!(expression.attrs_mut().unwrap().is_empty());
    }
}

#[test]
fn block_jump_and_primary_accessors_report_every_concrete_variant() {
    for (source, expected) in [
        ("{ value }", 0),
        ("if ready { yes() }", 1),
        ("while ready { work(); }", 2),
        ("for item in items { work(item); }", 3),
        ("loop { break; }", 4),
        ("match value { Some(x) => x, _ => 0 }", 5),
        ("async move { work().await }", 6),
        ("unsafe { call() }", 7),
        ("const { 1 }", 8),
        ("try { work()? }", 9),
    ] {
        let expression: Expr = moxy_token::parse!(source).unwrap();
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
            std::array::from_fn(|index| index == expected)
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
            std::array::from_fn(|index| index == expected)
        );
        assert!(block.attrs().is_empty());
        assert!(block.clone().into_expr().is_block());
        let converted = match expected {
            0 => Expr::from(block.as_brace().unwrap().clone()),
            1 => Expr::from(block.as_if().unwrap().clone()),
            2 => Expr::from(block.as_while().unwrap().clone()),
            3 => Expr::from(block.as_for_loop().unwrap().clone()),
            4 => Expr::from(block.as_loop().unwrap().clone()),
            5 => Expr::from(block.as_match().unwrap().clone()),
            6 => Expr::from(block.as_async().unwrap().clone()),
            7 => Expr::from(block.as_unsafe().unwrap().clone()),
            8 => Expr::from(block.as_const().unwrap().clone()),
            9 => Expr::from(block.as_try_block().unwrap().clone()),
            _ => unreachable!(),
        };
        assert_eq!(converted.to_token_stream(), expression.to_token_stream());
        assert!(!expression.span().is_empty());
    }

    for (source, expected) in [
        ("return value", 0),
        ("break 'label value", 1),
        ("continue 'label", 2),
        ("yield value", 3),
    ] {
        let expression: Expr = moxy_token::parse!(source).unwrap();
        let jump = expression.as_jump().unwrap();
        assert_eq!(
            [jump.is_return(), jump.is_break(), jump.is_continue(), jump.is_yield()],
            std::array::from_fn(|index| index == expected)
        );
        assert_eq!(
            [
                jump.as_return().is_some(),
                jump.as_break().is_some(),
                jump.as_continue().is_some(),
                jump.as_yield().is_some(),
            ],
            std::array::from_fn(|index| index == expected)
        );
        assert!(jump.attrs().is_empty());
        assert!(jump.clone().into_expr().is_jump());
        let converted = match expected {
            0 => Expr::from(jump.as_return().unwrap().clone()),
            1 => Expr::from(jump.as_break().unwrap().clone()),
            2 => Expr::from(jump.as_continue().unwrap().clone()),
            3 => Expr::from(jump.as_yield().unwrap().clone()),
            _ => unreachable!(),
        };
        assert_eq!(converted.to_token_stream(), expression.to_token_stream());
        assert!(!expression.span().is_empty());
    }

    for (source, expected) in [
        ("42", 0),
        ("path::value", 1),
        ("Point { x: 1 }", 2),
        ("|x| x", 3),
        ("(a, b)", 4),
        ("[a, b]", 5),
        ("[value; 3]", 6),
        ("let Some(value) = option", 7),
        ("(value)", 8),
        ("macro_call!(tokens)", 10),
    ] {
        let expression: Expr = moxy_token::parse!(source).unwrap();
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
            std::array::from_fn(|index| index == expected)
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
            std::array::from_fn(|index| index == expected)
        );
        assert!(primary.attrs().is_empty());
        assert!(primary.clone().into_expr().is_primary());
        let converted = match expected {
            0 => Expr::from(primary.as_lit().unwrap().clone()),
            1 => Expr::from(primary.as_path().unwrap().clone()),
            2 => Expr::from(primary.as_struct().unwrap().clone()),
            3 => Expr::from(primary.as_closure().unwrap().clone()),
            4 => Expr::from(primary.as_tuple().unwrap().clone()),
            5 => Expr::from(primary.as_array().unwrap().clone()),
            6 => Expr::from(primary.as_repeat().unwrap().clone()),
            7 => Expr::from(primary.as_let().unwrap().clone()),
            8 => Expr::from(primary.as_paren().unwrap().clone()),
            10 => Expr::from(primary.as_macro().unwrap().clone()),
            _ => unreachable!(),
        };
        assert_eq!(converted.to_token_stream(), expression.to_token_stream());
        assert!(!expression.span().is_empty());
    }
}

#[test]
fn type_pattern_statement_and_item_accessors_match_concrete_public_output() {
    for (source, expected) in [
        ("!", 0),
        ("_", 1),
        ("Vec<T>", 2),
        ("(A, B)", 3),
        ("[u8; 32]", 4),
        ("[u8]", 5),
        ("&mut T", 6),
        ("*const T", 7),
        ("fn(T) -> U", 8),
        ("impl Clone", 9),
        ("dyn Trait", 10),
        ("(T)", 11),
        ("m!(T)", 13),
    ] {
        let ty: Type = moxy_token::parse!(source).unwrap();
        assert_eq!(
            [
                ty.is_never(),
                ty.is_infer(),
                ty.is_path(),
                ty.is_tuple(),
                ty.is_array(),
                ty.is_slice(),
                ty.is_reference(),
                ty.is_pointer(),
                ty.is_bare_fn(),
                ty.is_impl_trait(),
                ty.is_trait_object(),
                ty.is_paren(),
                ty.is_group(),
                ty.is_macro(),
            ],
            std::array::from_fn(|index| index == expected)
        );
        assert_eq!(
            [
                ty.as_never().is_some(),
                ty.as_infer().is_some(),
                ty.as_path().is_some(),
                ty.as_tuple().is_some(),
                ty.as_array().is_some(),
                ty.as_slice().is_some(),
                ty.as_reference().is_some(),
                ty.as_pointer().is_some(),
                ty.as_bare_fn().is_some(),
                ty.as_impl_trait().is_some(),
                ty.as_trait_object().is_some(),
                ty.as_paren().is_some(),
                ty.as_group().is_some(),
                ty.as_macro().is_some(),
            ],
            std::array::from_fn(|index| index == expected)
        );
        assert!(!ty.to_token_stream().is_empty());
        assert!(!ty.span().is_empty());
        let converted = match expected {
            2 => Some(Type::from(ty.as_path().unwrap().clone())),
            3 => Some(Type::from(ty.as_tuple().unwrap().clone())),
            5 => Some(Type::from(ty.as_slice().unwrap().clone())),
            6 => Some(Type::from(ty.as_reference().unwrap().clone())),
            7 => Some(Type::from(ty.as_pointer().unwrap().clone())),
            8 => Some(Type::from(ty.as_bare_fn().unwrap().clone())),
            9 => Some(Type::from(ty.as_impl_trait().unwrap().clone())),
            10 => Some(Type::from(ty.as_trait_object().unwrap().clone())),
            11 => Some(Type::from(ty.as_paren().unwrap().clone())),
            _ => None,
        };
        if let Some(converted) = converted {
            assert_eq!(converted.to_token_stream(), ty.to_token_stream());
        }
    }

    for (source, expected) in [
        ("_", 0),
        ("..", 1),
        ("name", 2),
        ("path::CONST", 3),
        ("(a, b)", 4),
        ("Some(value)", 5),
        ("Point { x, .. }", 6),
        ("[a, b]", 7),
        ("&mut value", 8),
        ("A | B", 9),
        ("1", 10),
        ("box value", 16),
        ("const { 1 }", 17),
    ] {
        let pattern: Pattern = moxy_token::parse!(source).unwrap();
        assert_eq!(
            [
                pattern.is_wild(),
                pattern.is_rest(),
                pattern.is_ident(),
                pattern.is_path(),
                pattern.is_tuple(),
                pattern.is_tuple_struct(),
                pattern.is_struct(),
                pattern.is_slice(),
                pattern.is_reference(),
                pattern.is_or(),
                pattern.is_lit(),
                pattern.is_range(),
                pattern.is_macro(),
                pattern.is_type(),
                pattern.is_group(),
                pattern.is_paren(),
                pattern.is_box(),
                pattern.is_const(),
            ],
            std::array::from_fn(|index| index == expected)
        );
        assert_eq!(
            [
                false,
                false,
                pattern.as_ident().is_some(),
                pattern.as_path().is_some(),
                pattern.as_tuple().is_some(),
                pattern.as_tuple_struct().is_some(),
                pattern.as_struct().is_some(),
                pattern.as_slice().is_some(),
                pattern.as_reference().is_some(),
                pattern.as_or().is_some(),
                pattern.as_lit().is_some(),
                pattern.as_range().is_some(),
                pattern.as_macro().is_some(),
                pattern.as_type().is_some(),
                pattern.as_group().is_some(),
                pattern.as_paren().is_some(),
                pattern.as_box().is_some(),
                pattern.as_const().is_some(),
            ][2..],
            std::array::from_fn::<_, 16, _>(|index| index + 2 == expected)
        );
        assert!(!pattern.to_token_stream().is_empty());
        if expected <= 1 {
            assert_eq!(pattern.span(), Span::call_site());
        } else {
            assert!(!pattern.span().is_empty());
        }
        let converted = match expected {
            2 => Some(Pattern::from(pattern.as_ident().unwrap().clone())),
            3 => Some(Pattern::from(pattern.as_path().unwrap().clone())),
            4 => Some(Pattern::from(pattern.as_tuple().unwrap().clone())),
            5 => Some(Pattern::from(pattern.as_tuple_struct().unwrap().clone())),
            6 => Some(Pattern::from(pattern.as_struct().unwrap().clone())),
            7 => Some(Pattern::from(pattern.as_slice().unwrap().clone())),
            8 => Some(Pattern::from(pattern.as_reference().unwrap().clone())),
            9 => Some(Pattern::from(pattern.as_or().unwrap().clone())),
            10 => Some(Pattern::from(pattern.as_lit().unwrap().clone())),
            11 => Some(Pattern::from(pattern.as_range().unwrap().clone())),
            _ => None,
        };
        if let Some(converted) = converted {
            assert_eq!(converted.to_token_stream(), pattern.to_token_stream());
        }
    }

    for (source, expected) in [
        ("let value = 1;", 0),
        ("{ work(); }", 1),
        ("const VALUE: usize = 1;", 2),
        ("work();", 3),
    ] {
        let statement: Stmt = moxy_token::parse!(source).unwrap();
        assert_eq!(
            [
                statement.is_local(),
                statement.is_block(),
                statement.is_item(),
                statement.is_expr(),
                statement.is_macro()
            ],
            std::array::from_fn(|index| index == expected)
        );
        assert_eq!(
            [
                statement.as_local().is_some(),
                statement.as_block().is_some(),
                statement.as_item().is_some(),
                false,
                statement.as_macro().is_some(),
            ],
            std::array::from_fn(|index| index == expected && index != 3)
        );
        assert!(!statement.to_token_stream().is_empty());
        assert!(!statement.span().is_empty());
    }

    let macro_statement = moxy_token::parse!("macro_call!();" as moxy_ast::stmt::StmtMacro)
        .unwrap()
        .into_stmt();
    assert!(macro_statement.is_macro());
    assert_eq!(
        macro_statement.as_macro().unwrap().mac.path.to_token_stream().to_string(),
        "macro_call"
    );

    for source in [
        "use std::fmt;",
        "extern crate core;",
        "mod module;",
        "fn function() {}",
        "struct Record;",
        "enum Choice { One }",
        "union Storage { value: u64 }",
        "trait Service {}",
        "trait Alias = Clone;",
        "impl Record {}",
        "type AliasType = usize;",
        "const VALUE: usize = 1;",
        "static VALUE: usize = 1;",
        "macro_call!();",
        "macro_rules! local { () => {}; }",
        "extern \"C\" {}",
    ] {
        let item: Item = moxy_token::parse!(source).unwrap();
        let matches = [
            item.as_use().is_some(),
            item.as_extern_crate().is_some(),
            item.as_mod().is_some(),
            item.as_fn().is_some(),
            item.as_struct().is_some(),
            item.as_enum().is_some(),
            item.as_union().is_some(),
            item.as_trait().is_some(),
            item.as_trait_alias().is_some(),
            item.as_impl().is_some(),
            item.as_type_alias().is_some(),
            item.as_const().is_some(),
            item.as_static().is_some(),
            item.as_macro().is_some(),
            item.as_macro2().is_some(),
            item.as_foreign_mod().is_some(),
        ];
        assert_eq!(
            matches.iter().filter(|matched| **matched).count(),
            1,
            "ambiguous item accessors for {source}"
        );
        let kinds = [
            item.is_use(),
            item.is_extern_crate(),
            item.is_mod(),
            item.is_fn(),
            item.is_struct(),
            item.is_enum(),
            item.is_union(),
            item.is_trait(),
            item.is_trait_alias(),
            item.is_impl(),
            item.is_type_alias(),
            item.is_const(),
            item.is_static(),
            item.is_macro(),
            item.is_macro2(),
            item.is_foreign_mod(),
        ];
        assert_eq!(kinds, matches, "item kind disagreed with accessor for {source}");
        let converted = if let Some(value) = item.as_use() {
            Item::from(value.clone())
        } else if let Some(value) = item.as_extern_crate() {
            Item::from(value.clone())
        } else if let Some(value) = item.as_mod() {
            Item::from(value.clone())
        } else if let Some(value) = item.as_fn() {
            Item::from(value.clone())
        } else if let Some(value) = item.as_struct() {
            Item::from(value.clone())
        } else if let Some(value) = item.as_enum() {
            Item::from(value.clone())
        } else if let Some(value) = item.as_union() {
            Item::from(value.clone())
        } else if let Some(value) = item.as_trait() {
            Item::from(value.clone())
        } else if let Some(value) = item.as_trait_alias() {
            Item::from(value.clone())
        } else if let Some(value) = item.as_impl() {
            Item::from(value.clone())
        } else if let Some(value) = item.as_type_alias() {
            Item::from(value.clone())
        } else if let Some(value) = item.as_const() {
            Item::from(value.clone())
        } else if let Some(value) = item.as_static() {
            Item::from(value.clone())
        } else if let Some(value) = item.as_macro() {
            Item::from(value.clone())
        } else if let Some(value) = item.as_macro2() {
            Item::from(value.clone())
        } else {
            Item::from(item.as_foreign_mod().unwrap().clone())
        };
        assert_eq!(converted.to_token_stream(), item.to_token_stream());
        assert!(!item.to_token_stream().is_empty());
        assert_eq!(item.span().join(item.span()), item.span());
    }
}
