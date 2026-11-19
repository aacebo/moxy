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
