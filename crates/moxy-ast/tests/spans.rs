use moxy_ast::{Attribute, BinOp, Expr, FieldsNamed, Type};
use moxy_token::parse::Parse;
use moxy_token::{Span, Spanner, ToTokenStream, TokenStream};

fn parse<T: Parse>(src: &str) -> T {
    let ts: TokenStream = src.parse().unwrap();
    let mut ps = ts.parse();
    ps.parse::<T>().unwrap()
}

fn span_of<T: Spanner>(value: &T) -> Span {
    value.span()
}

#[test]
fn token_types_implement_spanner() {
    let plus = parse::<BinOp>("    +");
    let BinOp::Add(plus) = plus else { panic!("expected Add") };
    assert_eq!(span_of(&plus).start().index(), 4);

    let fields = parse::<FieldsNamed>("{ a: A }");
    assert!(span_of(&fields.fields).start().index() <= span_of(&fields.fields).end().index());
}

#[test]
fn leaf_op_preserves_span() {
    let op = parse::<BinOp>("    +");
    let BinOp::Add(plus) = op else { panic!("expected Add") };
    assert_eq!(
        plus.span().start().index(),
        4,
        "leaf op span was not the real source position"
    );
}

#[test]
fn leaf_equality_ignores_span() {
    assert_eq!(parse::<BinOp>("+  "), parse::<BinOp>("  +"));
}

#[test]
fn delimiter_preserves_span() {
    let fields = parse::<FieldsNamed>("{ a: A }");
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
    let outer = parse::<Attribute>("#[inline]");
    assert_eq!(outer.to_token_stream().to_string(), "# [inline]");

    let inner = parse::<Attribute>("#![no_std]");
    assert_eq!(inner.to_token_stream().to_string(), "# ! [no_std]");
}

#[test]
fn assign_eq_preserves_span() {
    let e = parse::<Expr>("a = b");
    let Expr::Binary(moxy_ast::BinaryExpr::Assign(assign)) = e else {
        panic!("expected assignment");
    };
    assert_eq!(assign.eq.span().start().index(), 2);
}

#[test]
fn never_type_preserves_span() {
    let ty = parse::<Type>("  !");
    let Type::Never(not) = ty else { panic!("expected never type") };
    assert_eq!(not.span().start().index(), 2);
}
