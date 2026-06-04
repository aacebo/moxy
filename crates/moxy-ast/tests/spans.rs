//! Verifies that AST nodes store the *real* source spans of the tokens they
//! parse, rather than discarding them and reconstructing via `::default()`
//! (which would reset every span to `call_site`).

use moxy_ast::{Attribute, BinOp, Expr, FieldsNamed, Type};
use moxy_token::parse::Parse;
use moxy_token::{Span, Spanner, ToTokenStream, TokenStream};

fn parse<T: Parse>(src: &str) -> T {
    let ts: TokenStream = src.parse().unwrap();
    let mut ps = ts.parse();
    ps.parse::<T>().unwrap()
}

/// Anything implementing `Spanner` exposes its source span through the trait.
fn span_of<T: Spanner>(value: &T) -> Span {
    value.span()
}

/// Token primitives implement `Spanner` (punct, keyword, delimiter, group, ...).
#[test]
fn token_types_implement_spanner() {
    let plus = parse::<BinOp>("    +");
    let BinOp::Add(plus) = plus else { panic!("expected Add") };
    assert_eq!(span_of(&plus).start().index(), 4);

    let fields = parse::<FieldsNamed>("{ a: A }");
    // `Bracket`/`Brace`/`Paren` also implement `Spanner`.
    assert!(span_of(&fields.brace).start().index() <= span_of(&fields.brace).end().index());
}

/// A leaf operator parsed at a non-zero offset keeps that offset in its span.
#[test]
fn leaf_op_preserves_span() {
    // `+` sits at index 4 in the source.
    let op = parse::<BinOp>("    +");
    let BinOp::Add(plus) = op else { panic!("expected Add") };
    assert_eq!(
        plus.span().start().index(),
        4,
        "leaf op span was not the real source position"
    );
}

/// Equality on leaf enums ignores spans: same operator at different offsets is equal.
#[test]
fn leaf_equality_ignores_span() {
    assert_eq!(parse::<BinOp>("+  "), parse::<BinOp>("  +"));
}

/// A delimiter token stores the real open/close spans of its group.
#[test]
fn delimiter_preserves_span() {
    // `{` is at index 1, `}` at index 9 in "S { a: A }" → fields group `{ a: A }`.
    let fields = parse::<FieldsNamed>("{ a: A }");
    let open = fields.brace.span().open().start().index();
    let close = fields.brace.span().close().start().index();
    assert!(close > open, "brace close span should follow its open span");
    assert_ne!(
        (open, close),
        (0, 0),
        "brace delimiter span was reset to call_site instead of the real positions",
    );
}

/// An attribute stores its `#`, optional `!`, and `[...]` tokens, and round-trips.
#[test]
fn attribute_roundtrips_with_stored_tokens() {
    let outer = parse::<Attribute>("#[inline]");
    assert_eq!(outer.to_token_stream().to_string(), "# [inline]");

    let inner = parse::<Attribute>("#![no_std]");
    assert_eq!(inner.to_token_stream().to_string(), "# ! [no_std]");
}

/// A punctuation field (`=` in an assignment) is stored, not reconstructed.
#[test]
fn assign_eq_preserves_span() {
    let e = parse::<Expr>("a = b");
    let Expr::Binary(moxy_ast::BinaryExpr::Assign(assign)) = e else {
        panic!("expected assignment");
    };
    // `=` is at index 2 in "a = b".
    assert_eq!(assign.eq.span().start().index(), 2);
}

/// The never type `!` carries the real span of its `!` token.
#[test]
fn never_type_preserves_span() {
    let ty = parse::<Type>("  !");
    let Type::Never(not) = ty else { panic!("expected never type") };
    assert_eq!(not.span().start().index(), 2);
}
