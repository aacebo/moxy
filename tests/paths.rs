use moxy::ast::{Expr, Type};
use moxy::token::Spanner;

#[test]
fn absolute_generic_paths_preserve_segments_and_arguments() {
    let ty: Type = moxy::parse!("::std::collections::HashMap<String, Vec<u8>>").unwrap();
    let path = &ty.as_path().unwrap().path;
    assert!(path.leading_colon().is_some());
    assert_eq!(path.len(), 3);
    assert_eq!(path.last().unwrap().ident.text(), "HashMap");
    assert!(!path.span().is_empty());
    assert_eq!(moxy::fmt!(&ty).unwrap(), "::std::collections::HashMap<String, Vec<u8>>");
}

#[test]
fn associated_and_turbofish_paths_render_in_expressions() {
    let expression: Expr = moxy::parse!("Type::VALUE").unwrap();
    assert!(!expression.is_postfix());
    assert!(!expression.span().is_empty());
    assert_eq!(moxy::fmt!(&expression).unwrap(), "Type::VALUE");
}

#[test]
fn turbofish_paths_render_complete_valid_syntax() {
    for source in ["function::<T>(a, b)", "object.method::<T>(a, b)"] {
        let expression: Expr = moxy::parse!(source).unwrap();
        assert!(expression.is_postfix());
        assert!(!expression.span().is_empty());
        assert_eq!(moxy::fmt!(&expression).unwrap(), source);
    }
}

#[test]
fn qualified_self_paths_render_complete_valid_syntax() {
    let ty: Type = moxy::parse!("<T as Trait>::Assoc").unwrap();
    assert!(ty.is_path());
    assert!(!ty.span().is_empty());
    assert_eq!(moxy::fmt!(&ty).unwrap(), "<T as Trait>::Assoc");
}
