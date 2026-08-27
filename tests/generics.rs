use moxy::ast::generics::GenericParam;
use moxy::ast::{Item, WherePredicate};
use moxy::token::Spanner;

#[test]
fn lifetime_type_and_const_parameters_are_inspectable_in_struct_syntax() {
    let item: Item = moxy::parse!("pub struct Buffer<'a, T: Clone, const N: usize> where T: Send { data: &'a [T; N] }").unwrap();
    let structure = item.as_struct().unwrap();
    assert_eq!(structure.generics.params.len(), 3);
    assert!(matches!(structure.generics.params[0], GenericParam::Lifetime(_)));
    assert!(matches!(structure.generics.params[1], GenericParam::Type(_)));
    assert!(matches!(structure.generics.params[2], GenericParam::Const(_)));
    assert_eq!(structure.generics.where_clause.as_ref().unwrap().predicates.len(), 1);
    assert!(!structure.generics.span().is_empty());
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "pub struct Buffer<'a, T: Clone, const N: usize>\nwhere\nT: Send {\n\tdata: &'a [T; N],\n}"
    );
}

#[test]
fn higher_ranked_where_predicates_render_on_functions() {
    let item: Item = moxy::parse!("fn borrow<T>(value: T) where for<'a> &'a T: IntoIterator, T: 'static { }").unwrap();
    let function = item.as_fn().unwrap();
    let predicates = &function.sig.generics.where_clause.as_ref().unwrap().predicates;
    assert_eq!(predicates.len(), 2);
    assert!(matches!(predicates[0], WherePredicate::Type(_)));
    assert!(matches!(predicates[1], WherePredicate::Type(_)));
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "fn borrow<T>(value: T)\nwhere\nfor<'a> &'a T: IntoIterator,\nT: 'static {}"
    );
}
