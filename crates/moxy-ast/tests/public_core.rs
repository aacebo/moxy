use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use moxy_ast::args::{AssocConstArgument, AssocTypeArgument, ConstraintArgument};
use moxy_ast::expr::{BinaryExpr, ExprGroup, ExprType, PrimaryExpr};
use moxy_ast::pat::{PatGroup, PatParen, PatRange};
use moxy_ast::ty::TypeGroup;
use moxy_ast::{
    Attributes, BoundLifetimes, ClosureParam, Crate, Delimited, Expr, GenericArgument, Ident, Pattern, RangeLimits, ReturnType,
    Signature, Type,
};
use moxy_token::{Delim, Span, Spanner, ToTokenStream, TokenStream};

#[test]
fn crate_root_preserves_inner_attributes_items_tokens_and_all_span_shapes() {
    let empty: Crate = moxy_token::parse!("").unwrap();
    assert!(empty.attrs.is_empty() && empty.items.is_empty());
    assert_eq!(empty.to_token_stream().to_string(), "");
    assert_eq!(empty.span(), Span::call_site());

    let attrs_only: Crate = moxy_token::parse!("#![allow(dead_code)]").unwrap();
    assert_eq!(attrs_only.attrs.len(), 1);
    assert!(attrs_only.items.is_empty());
    assert_eq!(attrs_only.to_token_stream().to_string(), "# ! [allow (dead_code)]");
    assert_eq!(attrs_only.span(), attrs_only.attrs[0].span());

    let items_only: Crate = moxy_token::parse!("struct Record; const VALUE: usize = 1;").unwrap();
    assert!(items_only.attrs.is_empty());
    assert_eq!(items_only.items.len(), 2);
    assert_eq!(
        items_only.to_token_stream().to_string(),
        "struct Record ; const VALUE : usize = 1 ;"
    );
    assert_eq!(items_only.span(), items_only.items[0].span().join(items_only.items[1].span()));

    let complete: Crate = moxy_token::parse!("#![no_std] pub mod api {} fn run() {}").unwrap();
    assert_eq!(complete.attrs.len(), 1);
    assert_eq!(complete.items.len(), 2);
    assert_eq!(
        complete.to_token_stream().to_string(),
        "# ! [no_std] pub mod api {} fn run () {}"
    );
    assert_eq!(complete.span(), complete.attrs[0].span().join(complete.items[1].span()));
}

#[test]
fn delimited_values_parse_mutate_surround_and_hash_exactly() {
    let tokens: TokenStream = "(alpha) {beta} [gamma]".parse().unwrap();
    let mut stream = tokens.parse();
    let mut paren = Delimited::<Ident>::parse_paren(&mut stream).unwrap();
    let brace = Delimited::<Ident>::parse_brace(&mut stream).unwrap();
    let bracket = Delimited::<Ident>::parse_bracket(&mut stream).unwrap();
    assert!(stream.is_empty());
    assert_eq!(
        [paren.style, brace.style, bracket.style],
        [Delim::Paren, Delim::Brace, Delim::Bracket]
    );
    assert_eq!([paren.text(), brace.text(), bracket.text()], ["alpha", "beta", "gamma"]);
    assert_eq!(paren.open(), paren.span.open());
    assert_eq!(paren.close(), paren.span.close());
    assert_eq!(paren.span(), paren.open().join(paren.close()));
    *paren = Ident::new("changed");
    assert_eq!(paren.to_token_stream().to_string(), "(changed)");
    assert_eq!(brace.clone().into_inner().text(), "beta");

    let direct = Delimited::new(Delim::Bracket, Default::default(), Ident::new("value"));
    let round = Delimited::paren(Default::default(), Ident::new("value"));
    let curly = Delimited::brace(Default::default(), Ident::new("value"));
    let square = Delimited::bracket(Default::default(), Ident::new("value"));
    assert_eq!(direct.to_token_stream().to_string(), "[value]");
    assert_eq!(round.to_token_stream().to_string(), "(value)");
    assert_eq!(curly.to_token_stream().to_string(), "{value}");
    assert_eq!(square.to_token_stream().to_string(), "[value]");

    let mut surrounded = TokenStream::new();
    square.surround(&mut surrounded, "inside".parse().unwrap());
    assert_eq!(surrounded.to_string(), "[inside]");
    assert_eq!(square, direct);
    let mut left = DefaultHasher::new();
    square.hash(&mut left);
    let mut right = DefaultHasher::new();
    direct.hash(&mut right);
    assert_eq!(left.finish(), right.finish());
    #[cfg(feature = "serde")]
    assert_eq!(
        serde_json::to_value(square).unwrap(),
        serde_json::json!({"style": "bracket", "inner": "value"})
    );
}

#[test]
fn closure_return_and_bound_models_expose_exact_variants_spans_and_tokens() {
    let inferred: ClosureParam = moxy_token::parse!("value").unwrap();
    let typed: ClosureParam = moxy_token::parse!("mut value: Option<T>").unwrap();
    assert_eq!([inferred.is_inferred(), inferred.is_typed()], [true, false]);
    assert_eq!([typed.is_inferred(), typed.is_typed()], [false, true]);
    assert_eq!(inferred.to_token_stream().to_string(), "value");
    assert_eq!(typed.to_token_stream().to_string(), "mut value : Option < T >");
    assert!(!inferred.span().is_empty() && !typed.span().is_empty());

    let default: ReturnType = moxy_token::parse!("").unwrap();
    let output: ReturnType = moxy_token::parse!("-> Result<T, E>").unwrap();
    assert_eq!([default.is_default(), default.is_type()], [true, false]);
    assert_eq!([output.is_default(), output.is_type()], [false, true]);
    assert!(default.as_type().is_none());
    assert_eq!(output.as_type().unwrap().to_token_stream().to_string(), "Result < T , E >");
    assert_eq!(default.to_token_stream().to_string(), "");
    assert_eq!(output.to_token_stream().to_string(), "-> Result < T , E >");
    assert_eq!(default.span(), Span::call_site());
    assert!(!output.span().is_empty());

    let lifetimes: BoundLifetimes = moxy_token::parse!("for<'a, 'b,>").unwrap();
    assert_eq!(lifetimes.params.len(), 2);
    assert!(lifetimes.params.is_trailing());
    assert_eq!(lifetimes.to_token_stream().to_string(), "for < 'a , 'b , >");
    assert!(!lifetimes.span().is_empty());
}

#[test]
fn generic_argument_families_preserve_concrete_payloads_conversions_and_output() {
    for (source, expected, output) in [
        ("'a", 0, "'a"),
        ("Vec<T>", 1, "Vec < T >"),
        ("4", 2, "4"),
        ("Item<T> = Vec<T>", 3, "Item < T > = Vec < T >"),
        ("N<T> = 4", 4, "N < T > = 4"),
        ("Item<T>: Clone + Send", 5, "Item < T > : Clone + Send"),
    ] {
        let argument: GenericArgument = moxy_token::parse!(source).unwrap();
        assert_eq!(argument.to_token_stream().to_string(), output);
        assert!(!argument.span().is_empty());
        assert_eq!(
            [
                matches!(argument, GenericArgument::Lifetime(_)),
                matches!(argument, GenericArgument::Type(_)),
                matches!(argument, GenericArgument::Const(_)),
                matches!(argument, GenericArgument::AssocType(_)),
                matches!(argument, GenericArgument::AssocConst(_)),
                matches!(argument, GenericArgument::Constraint(_)),
            ],
            std::array::from_fn(|index| index == expected)
        );
    }

    let assoc_type: AssocTypeArgument = moxy_token::parse!("Item<T> = Vec<T>").unwrap();
    assert_eq!(assoc_type.to_generic_argument(), assoc_type.clone().into_generic_argument());
    assert_eq!(
        assoc_type.to_generic_argument().to_token_stream().to_string(),
        "Item < T > = Vec < T >"
    );
    let assoc_const: AssocConstArgument = moxy_token::parse!("N<T> = 4").unwrap();
    assert_eq!(assoc_const.to_generic_argument(), assoc_const.clone().into_generic_argument());
    assert_eq!(assoc_const.to_generic_argument().to_token_stream().to_string(), "N < T > = 4");
    let constraint: ConstraintArgument = moxy_token::parse!("Item<T>: Clone + Send").unwrap();
    assert_eq!(constraint.to_generic_argument(), constraint.clone().into_generic_argument());
    assert_eq!(
        constraint.to_generic_argument().to_token_stream().to_string(),
        "Item < T > : Clone + Send"
    );
}

#[test]
fn macro_expansion_groups_type_ascriptions_and_range_patterns_preserve_public_output_and_spans() {
    let grouped_expression = ExprGroup {
        attrs: Attributes::default(),
        expr: Box::new(moxy_token::parse!("value + 1" as Expr).unwrap()),
    };
    assert_eq!(grouped_expression.to_token_stream().to_string(), "value + 1");
    assert_eq!(
        grouped_expression.span(),
        grouped_expression.attrs.span().join(grouped_expression.expr.span())
    );
    let primary = grouped_expression.clone().into_primary_expr();
    assert!(primary.is_group());
    assert_eq!(
        PrimaryExpr::from(grouped_expression).to_token_stream().to_string(),
        "value + 1"
    );

    let typed_expression = ExprType {
        attrs: Attributes::default(),
        expr: Box::new(moxy_token::parse!("value" as Expr).unwrap()),
        colon_punct: Default::default(),
        ty: Box::new(moxy_token::parse!("Option<T>" as Type).unwrap()),
    };
    assert_eq!(typed_expression.to_token_stream().to_string(), "value : Option < T >");
    assert_eq!(
        typed_expression.span(),
        typed_expression.attrs.span().join(typed_expression.ty.span())
    );
    assert!(typed_expression.clone().into_binary_expr().is_type());
    assert!(BinaryExpr::from(typed_expression).is_type());

    let grouped_pattern = PatGroup {
        attrs: Attributes::default(),
        pat: Box::new(moxy_token::parse!("value" as Pattern).unwrap()),
    };
    assert_eq!(grouped_pattern.to_token_stream().to_string(), "value");
    assert_eq!(
        grouped_pattern.span(),
        grouped_pattern.attrs.span().join(grouped_pattern.pat.span())
    );
    assert!(grouped_pattern.clone().into_pattern().is_group());
    assert!(Pattern::from(grouped_pattern).is_group());

    let parenthesized = PatParen {
        attrs: Attributes::default(),
        content: Delimited::paren(Default::default(), Box::new(moxy_token::parse!("A | B" as Pattern).unwrap())),
    };
    assert_eq!(parenthesized.to_token_stream().to_string(), "(A | B)");
    assert_eq!(parenthesized.span(), parenthesized.content.span());
    assert!(parenthesized.clone().into_pattern().is_paren());
    assert!(Pattern::from(parenthesized).is_paren());

    let range = PatRange {
        attrs: Attributes::default(),
        start: Some(moxy_token::parse!("1" as Expr).unwrap()),
        limits: RangeLimits::Closed(Default::default()),
        end: Some(moxy_token::parse!("10" as Expr).unwrap()),
    };
    assert_eq!(range.to_token_stream().to_string(), "1 ..= 10");
    assert_eq!(range.span(), range.attrs.span().join(range.end.as_ref().unwrap().span()));
    assert!(range.clone().into_pattern().is_range());
    assert!(Pattern::from(range).is_range());
    let open_range = PatRange {
        attrs: Attributes::default(),
        start: None,
        limits: RangeLimits::HalfOpen(Default::default()),
        end: None,
    };
    assert_eq!(open_range.to_token_stream().to_string(), "..");
    assert_eq!(open_range.span(), open_range.attrs.span().join(open_range.limits.span()));

    let grouped_type = TypeGroup {
        span: Span::mixed_site(),
        elem: Box::new(moxy_token::parse!("Vec<T>" as Type).unwrap()),
    };
    assert_eq!(grouped_type.span(), Span::mixed_site());
    assert_eq!(grouped_type.elem.to_token_stream().to_string(), "Vec < T >");
    let grouped_type = Type::Group(grouped_type);
    assert!(grouped_type.is_group());
    assert_eq!(grouped_type.as_group().unwrap().span, Span::mixed_site());
    assert_eq!(grouped_type.to_token_stream().to_string(), "");
}

#[test]
fn signatures_expose_modifiers_variadics_generics_outputs_spans_and_start_detection() {
    let plain: Signature = moxy_token::parse!("fn plain()").unwrap();
    assert_eq!(plain.to_token_stream().to_string(), "fn plain ()");
    assert!(matches!(plain.output, ReturnType::Default));
    assert_eq!(plain.params.inner.span(), Span::call_site());
    assert_eq!(plain.span(), plain.fn_keyword.span().join(plain.params.span()));

    let constant: Signature = moxy_token::parse!("const fn make<T>(value: T) -> T where T: Clone").unwrap();
    assert_eq!(constant.generics.params.len(), 1);
    assert!(constant.generics.where_clause.is_some());
    assert_eq!(constant.params.inner.inputs.len(), 1);
    assert_eq!(
        constant.to_token_stream().to_string(),
        "const fn make < T > (value : T) -> T where T : Clone"
    );
    assert_eq!(constant.span(), constant.constness.span().join(constant.output.span()));

    let asynchronous: Signature = moxy_token::parse!("async fn load() -> Result<T, E>").unwrap();
    assert_eq!(
        asynchronous.to_token_stream().to_string(),
        "async fn load () -> Result < T , E >"
    );
    assert_eq!(
        asynchronous.span(),
        asynchronous.asyncness.span().join(asynchronous.output.span())
    );

    let unsafe_signature: Signature = moxy_token::parse!("unsafe fn read(pointer: *const u8)").unwrap();
    assert_eq!(
        unsafe_signature.to_token_stream().to_string(),
        "unsafe fn read (pointer : * const u8)"
    );
    assert_eq!(
        unsafe_signature.span(),
        unsafe_signature.unsafety.span().join(unsafe_signature.params.span())
    );

    let external: Signature = moxy_token::parse!("extern \"C\" fn printf(format: *const u8, ...) -> i32").unwrap();
    assert_eq!(external.params.inner.inputs.len(), 1);
    assert!(external.params.inner.variadic.is_some());
    assert_eq!(
        external.params.inner.to_token_stream().to_string(),
        "format : * const u8 , ..."
    );
    assert_eq!(
        external.to_token_stream().to_string(),
        "extern \"C\" fn printf (format : * const u8 , ...) -> i32"
    );
    assert_eq!(
        external.span(),
        external.abi.as_ref().unwrap().span().join(external.output.span())
    );

    for (source, expected) in [
        ("fn f()", true),
        ("const async unsafe extern \"C\" fn f()", true),
        ("pub fn f()", false),
        ("struct Record;", false),
    ] {
        let tokens: TokenStream = source.parse().unwrap();
        assert_eq!(Signature::is_start(&mut tokens.parse()), expected, "{source}");
    }
}
