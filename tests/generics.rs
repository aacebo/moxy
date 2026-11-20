use moxy::ast::{GenericArgument, GenericParam, Item, PathArguments, Type, WherePredicate};
use moxy::token::{Spanner, ToTokenStream};

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
fn split_generics_emits_impl_type_and_where_tokens_separately() {
    let item: Item =
        moxy::parse!("struct Buffer<'a, T: Clone = bool, const N: usize = 1> where T: Send { data: &'a [T; N] }").unwrap();
    let generics = &item.as_struct().unwrap().generics;
    let (impl_generics, type_generics, where_clause) = generics.split();

    assert_eq!(
        impl_generics.to_token_stream().to_string(),
        "< 'a , T : Clone , const N : usize >"
    );
    assert_eq!(type_generics.to_token_stream().to_string(), "< 'a , T , N >");
    assert_eq!(where_clause.unwrap().to_token_stream().to_string(), "where T : Send");
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

#[test]
fn literal_const_generic_arguments_parse_in_implementations() {
    let item: Item = moxy::parse!("impl<P> Builder<P, 0> { fn end(self) -> P { self.0 } }").unwrap();
    let implementation = item.as_impl().unwrap();
    let Type::Path(self_ty) = &implementation.self_ty else {
        panic!("expected a path type");
    };
    let PathArguments::AngleBracketed(arguments) = &self_ty.path.last().unwrap().args else {
        panic!("expected angle-bracketed arguments");
    };

    assert_eq!(implementation.items.inner.len(), 1);
    assert!(matches!(arguments.args[1], GenericArgument::Const(_)));
}

#[test]
fn associated_const_generic_arguments_parse() {
    let item: Item = moxy::parse!("impl<P> Builder<P, COUNT = 0> {}").unwrap();

    assert_eq!(moxy::fmt!(&item).unwrap(), "impl<P> Builder<P, COUNT = 0> {}");
}

#[test]
fn const_generic_default_permutations_parse() {
    for source in [
        "pub struct Unit<const N: usize = 0>;",
        "struct Tuple<P, const N: usize = 0>(P);",
        "struct Named<'a, T: 'a, const N: usize = 0, const ENABLED: bool = true> { value: &'a T, bytes: [u8; N] }",
        "struct Multiple<const A: u8 = 0, const B: u8 = { 1 + 1 }>;",
    ] {
        let _: Item = moxy::parse!(source as Item).unwrap_or_else(|err| panic!("failed to parse {source:?}: {err}"));
    }
}
