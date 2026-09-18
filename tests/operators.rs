use moxy::ast::Expr;
use moxy::token::Spanner;

#[test]
fn binary_operator_precedence_is_preserved_in_rendered_expressions() {
    for (source, expected) in [
        ("a + b * c - d / e", "a + b * c - d / e"),
        ("a == b && c != d || ready", "a == b && c != d || ready"),
        ("flags & mask | extra ^ removed", "flags & mask | extra ^ removed"),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert!(expression.is_binary());
        assert!(!expression.span().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}

#[test]
fn assignment_range_cast_and_unary_operators_render_exactly() {
    for (source, expected) in [
        ("target += value", "target += value"),
        ("start..=end", "start..=end"),
        ("value as u64", "value as u64"),
        ("&mut value", "&mut value"),
        ("!flag", "!flag"),
        ("future?", "future?"),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}
