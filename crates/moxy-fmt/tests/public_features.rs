use moxy_ast::expr::{BinaryExpr, ExprGroup, ExprType, PrimaryExpr};
use moxy_ast::generics::GenericParam;
use moxy_ast::pat::{PatGroup, PatParen, PatType};
use moxy_ast::ty::TypeGroup;
use moxy_ast::{
    Attributes, BoundLifetimes, Expr, GenericArgument, Generics, Pattern, ReturnType, Type, TypeBound, Visibility, WherePredicate,
};
use moxy_fmt::{FmtConfig, NewlineStyle, fmt};
use moxy_token::Span;

#[test]
fn invisible_groups_and_type_ascriptions_have_exact_observable_formatting() {
    let grouped_expression = PrimaryExpr::Group(ExprGroup {
        attrs: Attributes::default(),
        expr: Box::new(moxy_token::parse!("value + 1" as Expr).unwrap()),
    })
    .into_expr();
    assert_eq!(fmt!(&grouped_expression).unwrap(), "value + 1");

    let typed_expression = BinaryExpr::Type(ExprType {
        attrs: Attributes::default(),
        expr: Box::new(moxy_token::parse!("value" as Expr).unwrap()),
        colon_punct: Default::default(),
        ty: Box::new(moxy_token::parse!("Option<T>" as Type).unwrap()),
    })
    .into_expr();
    assert_eq!(fmt!(&typed_expression).unwrap(), "value: Option<T>");

    let grouped_type = Type::Group(TypeGroup {
        span: Span::mixed_site(),
        elem: Box::new(moxy_token::parse!("Vec<T>" as Type).unwrap()),
    });
    assert_eq!(fmt!(&grouped_type).unwrap(), "Vec<T>");

    let grouped_pattern = Pattern::Group(PatGroup {
        attrs: Attributes::default(),
        pat: Box::new(moxy_token::parse!("value" as Pattern).unwrap()),
    });
    assert_eq!(fmt!(&grouped_pattern).unwrap(), "value");

    let parenthesized_pattern = Pattern::Paren(PatParen {
        attrs: Attributes::default(),
        content: moxy_ast::Delimited::paren(Default::default(), Box::new(moxy_token::parse!("A | B" as Pattern).unwrap())),
    });
    assert_eq!(fmt!(&parenthesized_pattern).unwrap(), "(A | B)");

    let typed_pattern = Pattern::Type(PatType {
        attrs: Attributes::default(),
        pat: Box::new(moxy_token::parse!("value" as Pattern).unwrap()),
        colon: Default::default(),
        ty: Box::new(moxy_token::parse!("Result<T, E>" as Type).unwrap()),
    });
    assert_eq!(fmt!(&typed_pattern).unwrap(), "value: Result<T, E>");
}

#[test]
fn generic_parameters_bounds_predicates_and_arguments_have_exact_output() {
    for (source, expected) in [
        ("'a: 'b + 'static", "'a: 'b + 'static"),
        ("T: Clone + Send = DefaultType", "T: Clone + Send = DefaultType"),
        ("const N: usize = 4", "const N: usize = 4"),
    ] {
        let parameter: GenericParam = moxy_token::parse!(source).unwrap();
        assert_eq!(fmt!(&parameter).unwrap(), expected);
    }

    for (source, expected) in [
        ("Trait", "Trait"),
        ("?Sized", "?Sized"),
        ("for<'a> Trait<'a>", "for<'a> Trait<'a>"),
        ("'static", "'static"),
        ("use<'a, 'b>", "use<'a, 'b>"),
    ] {
        let bound: TypeBound = moxy_token::parse!(source).unwrap();
        assert_eq!(fmt!(&bound).unwrap(), expected);
    }

    for (source, expected) in [
        ("'a: 'b + 'static", "'a: 'b + 'static"),
        ("T: Clone + Send", "T: Clone + Send"),
        ("for<'a> &'a T: IntoIterator", "for<'a> &'a T: IntoIterator"),
    ] {
        let predicate: WherePredicate = moxy_token::parse!(source).unwrap();
        assert_eq!(fmt!(&predicate).unwrap(), expected);
    }

    for (source, expected) in [
        ("'a", "'a"),
        ("Vec<T>", "Vec<T>"),
        ("4", "4"),
        ("Item = T", "Item = T"),
        ("N = 4", "N = 4"),
        ("Item: Clone + Send", "Item: Clone + Send"),
    ] {
        let argument: GenericArgument = moxy_token::parse!(source).unwrap();
        assert_eq!(fmt!(&argument).unwrap(), expected);
    }
}

#[test]
fn generics_visibility_lifetimes_and_return_types_cover_empty_and_populated_forms() {
    let generics: Generics = moxy_token::parse!("<'a, T: Clone, const N: usize> where 'a: 'static, T: Send").unwrap();
    assert_eq!(
        fmt!(&generics, FmtConfig::default().with_newline(NewlineStyle::Unix)).unwrap(),
        "<'a, T: Clone, const N: usize>\nwhere\n'a: 'static,\nT: Send"
    );
    let empty: Generics = moxy_token::parse!("").unwrap();
    assert_eq!(fmt!(&empty).unwrap(), "");

    let lifetimes: BoundLifetimes = moxy_token::parse!("for<'a, 'b>").unwrap();
    assert_eq!(fmt!(&lifetimes).unwrap(), "for<'a, 'b>");

    let output: ReturnType = moxy_token::parse!("-> Result<T, E>").unwrap();
    assert_eq!(fmt!(&output).unwrap(), " -> Result<T, E>");
    assert_eq!(fmt!(&ReturnType::Default).unwrap(), "");

    for (source, expected) in [
        ("pub", "pub"),
        ("pub(crate)", "pub(crate)"),
        ("pub(self)", "pub(self)"),
        ("pub(super)", "pub(super)"),
        ("pub(in module::nested)", "pub(in module::nested)"),
    ] {
        let visibility: Visibility = moxy_token::parse!(source).unwrap();
        assert_eq!(fmt!(&visibility).unwrap(), expected);
    }
}

#[test]
fn expression_optional_branches_have_exact_output_instead_of_only_parsing() {
    for (source, expected) in [
        ("Point{}", "Point {}"),
        ("Point{x,..base}", "Point {\n\tx,\n\t..base\n}"),
        ("|| value", "|| value"),
        ("move|x:i32|->i32{x}", "move |x: i32| -> i32 {\n\tx\n}"),
        ("..", ".."),
        ("..end", "..end"),
        ("start..", "start.."),
        ("object.method::<T>()", "object\n.method::<T>()"),
        ("if ready{yes()}", "if ready {\n\tyes()\n}"),
        (
            "match value{Some(x)if x>0=>x,_=>0}",
            "match value {\n\tSome(x) if x > 0 => x,\n\t_ => 0,\n}",
        ),
        ("async{work()}", "async {\n\twork()\n}"),
        ("async move{work()}", "async move {\n\twork()\n}"),
        ("return", "return"),
        ("break", "break"),
        ("break value", "break value"),
        ("continue", "continue"),
        ("yield", "yield"),
    ] {
        let expression: Expr = moxy_token::parse!(source).unwrap();
        assert_eq!(fmt!(&expression).unwrap(), expected, "wrong output for {source}");
    }
}
