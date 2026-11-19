use moxy::ast::Type;
use moxy::token::ToTokenStream;

#[test]
fn reference_pointer_array_slice_tuple_and_path_types_render_exactly() {
    for (source, expected, kind) in [
        ("&'a mut Vec<u8>", "&'a mut Vec<u8>", "reference"),
        ("*const T", "*const T", "pointer"),
        ("[u8;32]", "[u8; 32]", "array"),
        ("[u8]", "[u8]", "slice"),
        ("(A,B,C)", "(A, B, C)", "tuple"),
        ("std::vec::Vec<T>", "std::vec::Vec<T>", "path"),
    ] {
        let ty: Type = moxy::parse!(source).unwrap();
        assert_eq!(
            [
                ty.is_reference(),
                ty.is_pointer(),
                ty.is_array(),
                ty.is_slice(),
                ty.is_tuple(),
                ty.is_path()
            ],
            match kind {
                "reference" => [true, false, false, false, false, false],
                "pointer" => [false, true, false, false, false, false],
                "array" => [false, false, true, false, false, false],
                "slice" => [false, false, false, true, false, false],
                "tuple" => [false, false, false, false, true, false],
                _ => [false, false, false, false, false, true],
            }
        );
        assert_eq!(
            [
                ty.as_reference().is_some(),
                ty.as_pointer().is_some(),
                ty.as_array().is_some(),
                ty.as_slice().is_some(),
                ty.as_tuple().is_some(),
                ty.as_path().is_some(),
            ],
            match kind {
                "reference" => [true, false, false, false, false, false],
                "pointer" => [false, true, false, false, false, false],
                "array" => [false, false, true, false, false, false],
                "slice" => [false, false, false, true, false, false],
                "tuple" => [false, false, false, false, true, false],
                _ => [false, false, false, false, false, true],
            }
        );
        assert!(!ty.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&ty).unwrap(), expected);
    }
}

#[test]
fn trait_object_impl_trait_bare_function_and_macro_types_render_exactly() {
    for (source, expected, expected_kind) in [
        ("dyn Clone+Send+'a", "dyn Clone + Send + 'a", 0),
        ("impl Clone+Send", "impl Clone + Send", 1),
        (
            "unsafe extern \"C\" fn(&str)->Result<T,E>",
            "unsafe extern \"C\" fn(&str) -> Result<T, E>",
            2,
        ),
        ("factory!(T)", "factory!(T)", 3),
    ] {
        let ty: Type = moxy::parse!(source).unwrap();
        assert_eq!(
            [ty.is_trait_object(), ty.is_impl_trait(), ty.is_bare_fn(), ty.is_macro()],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert_eq!(
            [
                ty.as_trait_object().is_some(),
                ty.as_impl_trait().is_some(),
                ty.as_bare_fn().is_some(),
                ty.as_macro().is_some(),
            ],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert!(!ty.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&ty).unwrap(), expected);
    }
}

#[test]
fn never_infer_and_parenthesized_types_preserve_variants() {
    for (source, expected, expected_kind) in [("!", "!", 0), ("_", "_", 1), ("(T)", "(T)", 2)] {
        let ty: Type = moxy::parse!(source).unwrap();
        assert_eq!(
            [ty.is_never(), ty.is_infer(), ty.is_paren()],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert_eq!(
            [ty.as_never().is_some(), ty.as_infer().is_some(), ty.as_paren().is_some()],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert!(!ty.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&ty).unwrap(), expected);
    }
}

#[test]
fn const_generic_arguments_end_at_the_closing_angle_bracket() {
    for (source, expected) in [
        ("Foo<3>", "Foo<3>"),
        ("Foo<-1>", "Foo<-1>"),
        ("Foo<{ N + 1 }>", "Foo<{\n\tN + 1\n}>"),
        ("Foo<'a, T, 3>", "Foo<'a, T, 3>"),
        ("Foo<3, u8>", "Foo<3, u8>"),
        ("Foo<N = 3>", "Foo<N = 3>"),
        ("Vec<Foo<3>>", "Vec<Foo<3>>"),
    ] {
        let ty: Type = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(moxy::fmt!(&ty).unwrap(), expected);
    }
}

#[test]
fn generic_arguments_accept_paths_with_several_segments() {
    for (source, expected) in [
        ("Vec<a::B>", "Vec<a::B>"),
        ("Result<a::B, c::D>", "Result<a::B, c::D>"),
        ("Option<::std::string::String>", "Option<::std::string::String>"),
        (
            "&'static mut Vec<std::borrow::Cow<'_, str>>",
            "&'static mut Vec<std::borrow::Cow<'_, str>>",
        ),
        ("Vec<a::B<u8>>", "Vec<a::B<u8>>"),
    ] {
        let ty: Type = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(moxy::fmt!(&ty).unwrap(), expected);
    }
}

#[test]
fn unnamed_fields_and_bare_fn_arguments_accept_multi_segment_paths() {
    for (source, tokens) in [
        ("struct M(io::Error);", "struct M (io :: Error) ;"),
        (
            "enum E { Io(io::Error), Named { e: io::Error } }",
            "enum E {Io (io :: Error) , Named {e : io :: Error}}",
        ),
    ] {
        let item: moxy::ast::Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(item.to_token_stream().to_string(), tokens);
    }

    for (source, tokens) in [
        ("fn(a::B) -> u8", "fn (a :: B) -> u8"),
        ("fn(x: a::B) -> u8", "fn (x : a :: B) -> u8"),
    ] {
        let ty: Type = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(ty.to_token_stream().to_string(), tokens);
    }
}

#[test]
fn qualified_paths_emit_joined_path_separators() {
    let ty: Type = moxy::parse!("Option<<Self as Trait>::Item>").unwrap();
    assert_eq!(ty.to_token_stream().to_string(), "Option << Self as Trait > :: Item >");

    let built = moxy::ast::ColonColon::default().to_token_stream();
    assert_eq!(built.to_string(), "::");
}

#[test]
fn qualified_paths_keep_the_leading_colons_of_their_trait() {
    for (source, tokens) in [
        ("type X = <T as ::a::B>::C;", "type X = < T as :: a :: B > :: C ;"),
        ("fn f() { <T as ::B>::f(x); }", "fn f () {< T as :: B > :: f (x) ;}"),
        (
            "fn f() { match x { <T as ::B>::C(a) => {} } }",
            "fn f () {match x {< T as :: B > :: C (a) => {}}}",
        ),
    ] {
        let item: moxy::ast::Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(item.to_token_stream().to_string(), tokens);
    }

    let ty: Type = moxy::parse!("<T as ::a::B>::C").unwrap();
    assert_eq!(moxy::fmt!(&ty).unwrap(), "<T as ::a::B>::C");
}

#[test]
fn a_plus_after_a_function_return_type_bounds_the_whole_type() {
    for (source, bounds) in [
        ("dyn Fn(A) -> B + Send", 2),
        ("dyn FnMut() -> &'a u8 + Send + 'a", 3),
        ("dyn Fn(A) -> B", 1),
        ("impl Fn(A) -> B + Send", 2),
    ] {
        let ty: Type = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(ty.to_token_stream().to_string(), lexed.to_string());

        let count = match &ty {
            Type::TraitObject(object) => object.bounds.len(),
            Type::ImplTrait(object) => object.bounds.len(),
            other => panic!("`{source}` parsed as {}", other.to_token_stream()),
        };
        assert_eq!(count, bounds, "`{source}`");
    }

    let ty: Type = moxy::parse!("Box<dyn Fn(A) -> B + Send>").unwrap();
    assert_eq!(ty.to_token_stream().to_string(), "Box < dyn Fn (A) -> B + Send >");
}

#[test]
fn any_path_segment_takes_angle_or_parenthesized_arguments() {
    for source in [
        "impl AsyncFn() -> u8",
        "impl AsyncFnMut(u8) -> u8",
        "dyn MyTrait(A) -> B",
        "impl a::MyTrait(A) -> B",
        "Box<dyn AsyncFn() -> u8>",
        "Fn<u8>",
        "a::Fn<'a>",
        "FnMut<(A,), Output = B>",
        "Box<Fn(u8)>",
    ] {
        let ty: Type = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(ty.to_token_stream().to_string(), lexed.to_string());
    }

    let item: moxy::ast::Item = moxy::parse!("#[derive(Clone)] struct S;").unwrap();
    let attr = &item.as_struct().unwrap().attrs[0];
    assert!(attr.path.is_ident("derive"));
}

#[test]
fn trait_bounds_may_stand_in_parentheses() {
    for (source, bounds) in [
        ("Box<dyn (Fn() -> u8) + Send>", 2),
        ("&(dyn (A) + B)", 2),
        ("dyn (A)", 1),
        ("impl (A) + B", 2),
        ("impl ?Sized + (B)", 2),
        ("dyn (for<'a> A<'a>) + B", 2),
    ] {
        let ty: Type = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(ty.to_token_stream().to_string(), lexed.to_string());
        assert_eq!(
            ty.to_token_stream().to_string().matches('+').count() + 1,
            bounds,
            "`{source}`"
        );
    }

    let ty: Type = moxy::parse!("dyn (Fn() -> u8) + Send").unwrap();
    let Type::TraitObject(object) = &ty else {
        panic!("expected a trait object");
    };
    let moxy::ast::TypeBound::Trait(first) = object.bounds.first().unwrap() else {
        panic!("expected a trait bound");
    };
    assert!(first.paren.is_some());
    assert_eq!(moxy::fmt!(&ty).unwrap(), "dyn (Fn() -> u8) + Send");
}

#[test]
fn impl_and_dyn_take_one_bound_where_plus_does_not_continue_a_type() {
    for source in ["&dyn A + B", "&impl A + B", "*const dyn A + B", "fn() -> dyn A + B"] {
        assert!(moxy::parse!(source as Type).is_err(), "`{source}`");
    }

    let ty: Type = moxy::parse!("impl Fn() -> impl A + B").unwrap();
    let Type::ImplTrait(object) = &ty else {
        panic!("expected impl Trait");
    };
    assert_eq!(object.bounds.len(), 2);

    let ty: Type = moxy::parse!("&(dyn A + B)").unwrap();
    assert_eq!(ty.to_token_stream().to_string(), "& (dyn A + B)");
}

#[test]
fn nested_function_sugar_in_generic_arguments_parses_in_linear_time() {
    // Telling a bare trait object from a path skipped each bound before
    // parsing it, which doubled the work per level
    let depth = 40;
    let closed = format!("{}u8{}", "A<B() -> ".repeat(depth), ">".repeat(depth));
    let unclosed = "A<B() -> ".repeat(depth);
    let start = std::time::Instant::now();
    assert!(moxy::parse!(closed as Type).is_ok());
    assert!(moxy::parse!(unclosed as Type).is_err());
    assert!(
        start.elapsed() < std::time::Duration::from_secs(5),
        "took {:?}",
        start.elapsed()
    );

    let ty: Type = moxy::parse!("Box<A + Send>").unwrap();
    assert_eq!(ty.to_token_stream().to_string(), "Box < A + Send >");
}
