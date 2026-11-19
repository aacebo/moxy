use moxy::ast::Stmt;
use moxy::token::{Spanner, ToTokenStream};

#[test]
fn local_statements_preserve_patterns_types_initializers_and_else_blocks() {
    for (source, expected, has_else) in [
        ("let value: Option<T>;", "let value: Option<T>;", false),
        (
            "let mut value: u64 = compute() else { return; };",
            "let mut value: u64 = compute() else {\n\treturn;\n};",
            true,
        ),
    ] {
        let statement: Stmt = moxy::parse!(source).unwrap();
        let local = statement.as_local().unwrap();
        assert_eq!(local.init.as_ref().and_then(|init| init.diverge.as_ref()).is_some(), has_else);
        assert!(!statement.span().is_empty());
        assert!(!statement.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&statement).unwrap(), expected);
    }
}

#[test]
fn item_expression_block_and_macro_statements_have_distinct_shapes() {
    for (source, expected) in [
        ("const LOCAL: usize = 1;", "const LOCAL: usize = 1;"),
        ("value;", "value;"),
        ("{ work(); value }", "{\n\twork();\n\tvalue\n}"),
        ("call!();", "call!();"),
    ] {
        let statement: Stmt = moxy::parse!(source).unwrap();
        assert!(!statement.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&statement).unwrap(), expected);
    }
}

#[test]
fn malformed_local_syntax_returns_a_specific_parse_failure() {
    let result: Result<Stmt, _> = moxy::parse!("let = 1;");
    assert_eq!(result.unwrap_err().message(), "expected pattern");
}

#[test]
fn keywords_followed_by_a_negation_are_expressions_not_macro_calls() {
    for source in [
        "if !ready { wait(); }",
        "while !done { step(); }",
        "return !value;",
        "default!();",
    ] {
        let statement: Stmt = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(statement.as_macro().is_some(), source == "default!();", "`{source}`");
    }

    let item: moxy::ast::Item = moxy::parse!("fn f() { if !ready { wait(); } done() }").unwrap();
    assert_eq!(item.as_fn().unwrap().body.as_ref().unwrap().stmts.len(), 2);
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "fn f() {\n\tif !ready {\n\t\twait();\n\t}\n\tdone()\n}"
    );
}

#[test]
fn macro_rules_definitions_are_statements() {
    let item: moxy::ast::Item = moxy::parse!("fn f() { macro_rules! scan { ($x:ident) => { $x }; } g() }").unwrap();
    let statements = &item.as_fn().unwrap().body.as_ref().unwrap().stmts.inner;
    assert_eq!(statements.len(), 2);
    assert!(statements[0].as_item().is_some());
}

#[test]
fn block_like_expression_statements_end_at_their_block() {
    let item: moxy::ast::Item = moxy::parse!("fn f() { if a { b; } <T>::g(c) }").unwrap();
    let statements = &item.as_fn().unwrap().body.as_ref().unwrap().stmts.inner;
    assert_eq!(statements.len(), 2);

    // A method call continues the block-like expression
    let item: moxy::ast::Item = moxy::parse!("fn f() { if a { b } else { c }.len(); }").unwrap();
    assert_eq!(item.as_fn().unwrap().body.as_ref().unwrap().stmts.inner.len(), 1);
}

#[test]
fn macro_rules_definitions_in_parentheses_or_brackets_end_with_a_semicolon() {
    for (source, tokens) in [
        ("macro_rules! m ( () => {} );", "macro_rules ! m (() => {}) ;"),
        ("macro_rules! m [ () => {} ];", "macro_rules ! m [() => {}] ;"),
        ("macro_rules! m { () => {} }", "macro_rules ! m {() => {}}"),
    ] {
        let item: moxy::ast::Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(item.to_token_stream().to_string(), tokens);
    }

    let item: moxy::ast::Item = moxy::parse!("fn f() { macro_rules! m ( () => {} ); g() }").unwrap();
    let statements = &item.as_fn().unwrap().body.as_ref().unwrap().stmts.inner;
    assert_eq!(statements.len(), 2);
    assert!(statements[0].as_item().unwrap().as_macro2().unwrap().semi.is_some());

    let error = moxy::parse!("macro_rules! m ( () => {} )" as moxy::ast::Item).err().unwrap();
    assert_eq!(
        error.message(),
        "expected `;` after a `macro_rules!` body in parentheses or brackets"
    );
}

#[test]
fn a_macro_call_that_an_operator_follows_is_an_expression_statement() {
    for (source, tokens) in [
        ("{ write!(f, \"x\")?; Ok(()) }", "{write ! (f , \"x\") ? ; Ok (())}"),
        ("{ matches!(a, B) || c }", "{matches ! (a , B) || c}"),
        ("{ vec![1].len() }", "{vec ! [1] . len ()}"),
        ("{ cfg!(a) && b }", "{cfg ! (a) && b}"),
        ("{ m! {}.f(); }", "{m ! {} . f () ;}"),
    ] {
        let block: moxy::ast::StmtBlock = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(block.to_token_stream().to_string(), tokens);
        let stmts = &block.stmts.inner;
        assert!(stmts[0].is_expr(), "`{source}`");
    }

    let block: moxy::ast::StmtBlock = moxy::parse!("{ write!(f, \"x\")?; Ok(()) }").unwrap();
    assert_eq!(block.stmts.inner.len(), 2);
    let moxy::ast::Stmt::Expr(expression, Some(_)) = &block.stmts.inner[0] else {
        panic!("expected an expression statement");
    };
    let chain = expression.as_postfix().unwrap();
    assert!(chain.base.is_macro());
    assert!(matches!(chain.ops[..], [moxy::ast::Postfix::Try(_)]));

    let block: moxy::ast::StmtBlock = moxy::parse!("{ matches!(a, B) || c }").unwrap();
    assert_eq!(block.stmts.inner.len(), 1);

    // A braced call ends the statement; a call followed by `;` or nothing stays a macro statement
    for source in ["{ m! {} - 1 }", "{ m!(); n!() }"] {
        let block: moxy::ast::StmtBlock = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert!(block.stmts.inner[0].is_macro(), "`{source}`");
    }
}

#[test]
fn a_block_that_a_postfix_operator_follows_is_an_expression_statement() {
    for (source, tokens) in [
        ("{ {}.b }", "{{} . b}"),
        ("{ {}.b(); }", "{{} . b () ;}"),
        ("{ {}? }", "{{} ?}"),
    ] {
        let block: moxy::ast::StmtBlock = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(block.to_token_stream().to_string(), tokens);
        assert_eq!(block.stmts.inner.len(), 1, "`{source}`");
        assert!(block.stmts.inner[0].is_expr(), "`{source}`");
    }

    let block: moxy::ast::StmtBlock = moxy::parse!("{ {} -1 }").unwrap();
    assert!(block.stmts.inner[0].is_block());
}

#[test]
fn nested_block_statements_that_a_method_call_follows_parse_in_linear_time() {
    // Each level parsed its block twice before, which never ends at this depth
    let depth = 34;
    let source = format!("{}a{}", "{ {".repeat(depth), "}.x }".repeat(depth));
    let start = std::time::Instant::now();
    let block: moxy::ast::StmtBlock = moxy::parse!(source).unwrap();
    assert!(
        start.elapsed() < std::time::Duration::from_secs(5),
        "took {:?}",
        start.elapsed()
    );
    assert!(block.stmts.inner[0].is_expr());
}

#[test]
fn statements_in_blocks_inside_generic_arguments_parse_in_linear_time() {
    // Peeking for a macro call skipped each statement's path with its generic
    // arguments, and with them every block nested inside, before parsing it
    let depth = 28;
    let source = format!("{}1{}", "{ A::<{ ".repeat(depth), " }>::C }".repeat(depth));
    let start = std::time::Instant::now();
    let expression: moxy::ast::Expr = moxy::parse!(source).unwrap();
    assert!(
        start.elapsed() < std::time::Duration::from_secs(5),
        "took {:?}",
        start.elapsed()
    );
    assert!(expression.is_block());
}

#[test]
fn an_expression_statement_needs_a_semicolon_unless_it_is_block_like_or_last() {
    for source in [
        "{ unsafe {} b }",
        "{ const {} b }",
        "{ if a {} b }",
        "{ 'l: loop {} b }",
        "{ #[a] {} b }",
        "{ a; b }",
        "{ a }",
    ] {
        assert!(moxy::parse!(source as moxy::ast::StmtBlock).is_ok(), "`{source}`");
    }

    for source in ["{ a b }", "{ async {} b }", "{ {}.x b }", "{ x? b }"] {
        let error = moxy::parse!(source as moxy::ast::StmtBlock).err();
        assert_eq!(
            error.map(|error| error.message().to_string()).as_deref(),
            Some("expected `;`"),
            "`{source}`"
        );
    }
}

#[test]
fn a_let_statement_ends_with_a_semicolon_and_no_brace_before_else() {
    for source in [
        "{ let x = 1; }",
        "{ let Some(x) = y else { return }; }",
        "{ let x = f({}) else { return }; }",
    ] {
        assert!(moxy::parse!(source as moxy::ast::StmtBlock).is_ok(), "`{source}`");
    }

    for source in [
        "{ let x = 1 }",
        "{ let x }",
        "{ let x = 1 b }",
        "{ let x = S {} else { return }; }",
        "{ let x = if a { b } else { c } else { return }; }",
    ] {
        assert!(moxy::parse!(source as moxy::ast::StmtBlock).is_err(), "`{source}`");
    }
}

#[test]
fn a_try_block_ends_a_statement_as_other_blocks_do() {
    let block: moxy::ast::StmtBlock = moxy::parse!("{ try {} - 1 }").unwrap();
    assert_eq!(block.stmts.inner.len(), 2);
    assert!(moxy::parse!("{ try {} b }" as moxy::ast::StmtBlock).is_ok());
}
