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
    for (source, expected, expected_kind) in [
        ("const LOCAL: usize = 1;", "const LOCAL: usize = 1;", 0),
        ("value;", "value;", 1),
        ("{ work(); value }", "{\n\twork();\n\tvalue\n}", 2),
        ("call!();", "call!();", 0),
    ] {
        let statement: Stmt = moxy::parse!(source).unwrap();
        assert_eq!(
            [
                statement.is_item(),
                statement.is_expr(),
                statement.is_block(),
                statement.is_macro()
            ],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert_eq!(
            [
                statement.as_item().is_some(),
                matches!(statement, Stmt::Expr(..)),
                statement.as_block().is_some(),
                statement.as_macro().is_some(),
            ],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert!(!statement.span().is_empty());
        assert!(!statement.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&statement).unwrap(), expected);
    }
}

#[test]
fn malformed_local_syntax_returns_a_specific_parse_failure() {
    let result: Result<Stmt, _> = moxy::parse!("let = 1;");
    assert_eq!(result.unwrap_err().message(), "expected pattern");
}
