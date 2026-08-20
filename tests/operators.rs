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
    for (source, expected, expected_kind) in [
        ("target += value", "target += value", 0),
        ("start..=end", "start..=end", 0),
        ("value as u64", "value as u64", 1),
        ("&mut value", "&mut value", 1),
        ("!flag", "!flag", 1),
        ("future?", "future?", 1),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert_eq!(
            [expression.is_binary(), expression.is_unary()],
            std::array::from_fn(|i| i == expected_kind)
        );
        assert!(!expression.span().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
    }
}
