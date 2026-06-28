use moxy_ast::{Attribute, BinOp, Expr, FieldsNamed, Type};
use moxy_token::ToTokenStream;

#[test]
fn token_types_implement_spanner() {
    let plus = moxy_token::parse!("    +" as BinOp).unwrap();
    let BinOp::Add(plus) = plus else { panic!("expected Add") };
    assert_eq!(plus.span().start().index(), 4);

    let fields = moxy_token::parse!("{ a: A }" as FieldsNamed).unwrap();
    assert!(fields.fields.open().start().index() <= fields.fields.close().start().index());
}

#[test]
fn leaf_op_preserves_span() {
    let op = moxy_token::parse!("    +" as BinOp).unwrap();
    let BinOp::Add(plus) = op else { panic!("expected Add") };
    assert_eq!(
        plus.span().start().index(),
        4,
        "leaf op span was not the real source position"
    );
}

#[test]
fn leaf_equality_ignores_span() {
    assert_eq!(
        moxy_token::parse!("+  " as BinOp).unwrap(),
        moxy_token::parse!("  +" as BinOp).unwrap(),
    );
}

#[test]
fn delimiter_preserves_span() {
    let fields = moxy_token::parse!("{ a: A }" as FieldsNamed).unwrap();
    let open = fields.fields.open().start().index();
    let close = fields.fields.close().start().index();
    assert!(close > open, "brace close span should follow its open span");
    assert_ne!(
        (open, close),
        (0, 0),
        "brace delimiter span was reset to call_site instead of the real positions",
    );
}

#[test]
fn attribute_roundtrips_with_stored_tokens() {
    let outer = moxy_token::parse!("#[inline]" as Attribute).unwrap();
    assert_eq!(outer.to_token_stream().to_string(), "# [inline]");

    let inner = moxy_token::parse!("#![no_std]" as Attribute).unwrap();
    assert_eq!(inner.to_token_stream().to_string(), "# ! [no_std]");
}

#[test]
fn assign_eq_preserves_span() {
    let e = moxy_token::parse!("a = b" as Expr).unwrap();
    let Expr::Binary(moxy_ast::BinaryExpr::Assign(assign)) = e else {
        panic!("expected assignment");
    };
    assert_eq!(assign.eq.span().start().index(), 2);
}

#[test]
fn never_type_preserves_span() {
    let ty = moxy_token::parse!("  !" as Type).unwrap();
    let Type::Never(not) = ty else { panic!("expected never type") };
    assert_eq!(not.span().start().index(), 2);
}
