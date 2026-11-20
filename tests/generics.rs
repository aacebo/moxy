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
    assert_eq!(error.message(), "expected `,`, found `c`");
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

// rustc's parser accepts parenthesized arguments on an associated item's name,
// as in `Item(T): Clone`, and rejects them later; return type notation, as in
// `f(..): Send`, is unstable and stays an error
#[test]
fn associated_items_take_parenthesized_arguments() {
    use moxy::ast::{GenericArgument, Parse, Parser, PathArguments, Type};

    for (source, tokens, formatted) in [
        ("A<Item(T): Clone>", "A < Item (T) : Clone >", "A<Item(T): Clone>"),
        (
            "A<Item(T) -> U: Clone + Send>",
            "A < Item (T) -> U : Clone + Send >",
            "A<Item(T) -> U: Clone + Send>",
        ),
        ("A<Item::(T): Clone>", "A < Item :: (T) : Clone >", "A<Item(T): Clone>"),
        ("A<Item(T) = u8>", "A < Item (T) = u8 >", "A<Item(T) = u8>"),
        ("A<Item(T) = 3>", "A < Item (T) = 3 >", "A<Item(T) = 3>"),
    ] {
        let ty: Type = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let stream: moxy::token::TokenStream = source.parse().unwrap();
        let skipped = Type::skip(Parser::from_tokens(&stream).cursor());
        let Type::Path(path) = &ty else { panic!("`{source}`") };
        let PathArguments::AngleBracketed(args) = &path.path[0].args else {
            panic!("`{source}`")
        };
        let parenthesized = match &args.args[0] {
            GenericArgument::Constraint(v) => &v.args,
            GenericArgument::AssocType(v) => &v.args,
            GenericArgument::AssocConst(v) => &v.args,
            _ => panic!("`{source}`"),
        };

        assert!(matches!(parenthesized, PathArguments::Parenthesized(_)), "`{source}`");
        assert!(skipped.is_some_and(|cursor| cursor.is_empty()), "`{source}`");
        assert_eq!(ty.to_token_stream().to_string(), tokens);
        assert_eq!(moxy::fmt!(&ty).unwrap(), formatted);
    }

    // Only one segment without angle arguments takes them, as in rustc
    for source in ["A<Item<B>(T): Clone>", "A<a::Item(T): Clone>", "A<f(..): Send>"] {
        assert!(moxy::parse!(source as Type).is_err(), "`{source}`");
    }
}

#[test]
fn a_variadic_function_pointer_formats_one_separator() {
    for (source, formatted) in [
        ("type F = fn(A, ...);", "type F = fn(A, ...);"),
        ("type F = fn(A, b: B, ...) -> u8;", "type F = fn(A, b: B, ...) -> u8;"),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(moxy::fmt!(&item).unwrap(), formatted, "`{source}`");
    }
}

#[test]
fn enums_and_unions_format_their_where_clause() {
    for (source, formatted) in [
        ("enum E<T> where T: U { A }", "enum E<T>\nwhere\nT: U {\n\tA,\n}"),
        (
            "union U<T> where T: Copy { a: T }",
            "union U<T>\nwhere\nT: Copy {\n\ta: T,\n}",
        ),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(moxy::fmt!(&item).unwrap(), formatted, "`{source}`");
    }
}

#[test]
fn attribute_arguments_keep_a_space_between_words() {
    for (source, formatted) in [
        ("#[a(pub b)] struct S;", "#[a(pub b)]\nstruct S;"),
        ("#[a(in super)] struct S;", "#[a(in super)]\nstruct S;"),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(moxy::fmt!(&item).unwrap(), formatted, "`{source}`");
    }
}

#[test]
fn generic_parameters_format_their_attributes() {
    for (source, formatted) in [
        (
            "struct S<#[a] T, #[b] 'a, #[c] const N: usize>(T);",
            "struct S<#[a] T, #[b] 'a, #[c] const N: usize>(T);",
        ),
        ("fn f<#[a] #[b] T>() {}", "fn f<#[a] #[b] T>() {}"),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(moxy::fmt!(&item).unwrap(), formatted, "`{source}`");
    }
}

#[test]
fn raw_lifetimes_format_their_prefix() {
    for (source, formatted) in [
        ("fn f<'r#a>() {}", "fn f<'r#a>() {}"),
        ("fn f<'a: 'r#b>() {}", "fn f<'a: 'r#b>() {}"),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(moxy::fmt!(&item).unwrap(), formatted, "`{source}`");
    }
}

#[test]
fn empty_statements_format_as_a_semicolon() {
    for (source, formatted) in [
        ("fn f() { ; }", "fn f() {\n\t;\n}"),
        ("fn f() { a; ; }", "fn f() {\n\ta;\n\t;\n}"),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(moxy::fmt!(&item).unwrap(), formatted, "`{source}`");
    }
}

#[test]
fn bound_lists_take_a_trailing_separator() {
    for source in [
        "fn g<'a: 'a +>() {}",
        "fn f(x: impl Iterator<Item: Clone +>) {}",
        "type F = Box<dyn for<'b,> Fn(&'b u8)>;",
        "fn f<'a>() -> impl Sized + use<'a,> {}",
        "type T = A<B: >;",
        "fn f() where 'a: 'b + {}",
        "trait A { type T: B +; }",
    ] {
        let file: moxy::ast::File = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(
            moxy::token::ToTokenStream::to_token_stream(&file).to_string(),
            lexed.to_string()
        );
    }
}

#[test]
fn a_const_parameter_default_is_a_block_a_literal_or_a_name() {
    for default in ["1", "-1", "{ 1 + 1 }", "A", "true", "Self"] {
        let source = format!("struct S<const N: usize = {default}>;");
        assert!(moxy::parse!(source as moxy::ast::Item).is_ok(), "`{source}`");
    }

    for default in ["A::B", "(1)", "1 + 1", "-A"] {
        let source = format!("struct S<const N: usize = {default}>;");
        assert!(moxy::parse!(source as moxy::ast::Item).is_err(), "`{source}`");
    }
}
