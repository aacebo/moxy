use moxy::ast::{GenericParam, Item, WherePredicate};
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
fn closed_lists_accept_a_trailing_comma() {
    for (source, tokens) in [
        ("use a::{b, c,};", "use a :: {b , c ,} ;"),
        ("fn f<T: A,>(x: Foo<A, B,>) {}", "fn f < T : A , > (x : Foo < A , B , >) {}"),
        ("struct S<T,>(Vec<T,>);", "struct S < T , > (Vec < T , >) ;"),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(item.to_token_stream().to_string(), tokens);
    }

    let error = moxy::parse!("use a::{b c};" as Item).err().unwrap();
    assert_eq!(error.message(), "expected `,` punctuation");
}

#[test]
fn const_generic_defaults_end_before_the_closing_angle_bracket() {
    for source in [
        "struct S<const N: usize = 3>(u8);",
        "struct S<const B: bool = false>(usize);",
        "struct S<T = u8, const B: usize = 3>(T);",
        "struct S<const B: usize = { 3 }>(u8);",
        "struct S<const B: usize = N>(u8);",
        "struct S<const B: usize = -1>(u8);",
        "trait T<const N: usize = 3>: U {}",
        "type T<const N: usize = 3> = U;",
        "impl<const N: usize = 3> S<N> {}",
        "fn f<const N: usize = 3>(a: u8) {}",
        "struct S<const N: usize = { M + 1 }>(u8) where T: U;",
    ] {
        let item: moxy::ast::Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(item.to_token_stream().to_string(), lexed.to_string());
    }
}

#[test]
fn generic_lists_and_bounds_may_be_empty() {
    for source in [
        "impl<> X {}",
        "struct S<>;",
        "fn f<T:>() where T: {}",
        "fn f<T>() where Self:, T: {}",
        "fn f() where 'a: 'b, 'b: {}",
        "struct S<T: >(T);",
        "impl<T: Sized<>> From<T> for OsString {}",
        "type A = for<> fn();",
        "type A = Result<T, !>;",
        "type A = B<!, C>;",
        "fn f() { Vec::<>::new(); x.collect::<>(); Ok::<B, !>(b) }",
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(item.to_token_stream().to_string(), lexed.to_string());
    }

    let ty: moxy::ast::Type = moxy::parse!("Result<T, !>").unwrap();
    let path = ty.as_path().unwrap();
    let moxy::ast::PathArguments::AngleBracketed(args) = &path.path.iter().next().unwrap().args else {
        panic!("expected angle arguments");
    };
    assert!(matches!(
        args.args.iter().nth(1),
        Some(moxy::ast::GenericArgument::Type(moxy::ast::Type::Never(_)))
    ));
}
