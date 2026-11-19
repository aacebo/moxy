use moxy::ast::Item;
use moxy::token::ToTokenStream;

#[test]
fn inherent_implementations_preserve_members_and_render_layout() {
    let item: Item = moxy::parse!("impl Record{const VALUE:usize=1;type Output=usize;fn run(&self){}call!();}").unwrap();
    let implementation = item.as_impl().unwrap();
    assert!(implementation.trait_ref.is_none());
    assert_eq!(implementation.items.inner.len(), 4);
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "impl Record {\n\tconst VALUE: usize = 1;\n\t\n\ttype Output = usize;\n\t\n\tfn run(&self) {}\n\t\n\tcall!();\n}"
    );
}

#[test]
fn unsafe_negative_trait_implementations_expose_polarity() {
    let item: Item = moxy::parse!("unsafe impl !Send for Record{}").unwrap();
    let implementation = item.as_impl().unwrap();
    assert!(implementation.unsafety.is_some());
    assert!(implementation.trait_ref.as_ref().unwrap().polarity.is_some());
    assert_eq!(moxy::fmt!(&item).unwrap(), "unsafe impl !Send for Record {}");
}

#[test]
fn impl_where_clauses_follow_the_self_type() {
    let item: Item = moxy::parse!("impl<T> A for B<T> where T: C {}").unwrap();
    assert_eq!(item.to_token_stream().to_string(), "impl < T > A for B < T > where T : C {}");
    assert_eq!(moxy::fmt!(&item).unwrap(), "impl<T> A for B<T>\nwhere\nT: C {}");
}

#[test]
fn associated_type_where_clauses_follow_the_type() {
    for (source, tokens, formatted) in [
        (
            "impl A for B { type X<T> = Vec<T> where T: Clone; }",
            "impl A for B {type X < T > = Vec < T > where T : Clone ;}",
            "impl A for B {\n\ttype X<T> = Vec<T>\n\twhere\n\tT: Clone;\n}",
        ),
        (
            "trait A { type X<T>: B = Vec<T> where T: Clone; }",
            "trait A {type X < T > : B = Vec < T > where T : Clone ;}",
            "trait A {\n\ttype X<T>: B = Vec<T>\n\twhere\n\tT: Clone;\n}",
        ),
        // The older place before `=` stays where it is written
        (
            "impl A for B { type X<T> where T: Clone = Vec<T>; }",
            "impl A for B {type X < T > where T : Clone = Vec < T > ;}",
            "impl A for B {\n\ttype X<T>\n\twhere\n\tT: Clone = Vec<T>;\n}",
        ),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(item.to_token_stream().to_string(), tokens);
        assert_eq!(moxy::fmt!(&item).unwrap(), formatted);
    }
}

#[test]
fn a_type_alias_takes_its_where_clause_before_or_after_the_type() {
    for source in [
        "type T<U> = Vec<U> where U: A;",
        "type T<U> where U: A = Vec<U>;",
        "type T<U> where U: A = Vec<U> where U: B;",
        "type T<U> where U: A;",
        "type T: = u8;",
        "impl X { type T<U> where U: A = Vec<U> where U: B; }",
        "trait X { type T<U>: C where U: A = Vec<U> where U: B; }",
        "extern \"C\" { type T where Self: A = u8 where Self: B; }",
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(item.to_token_stream().to_string(), lexed.to_string(), "`{source}`");
    }

    let item: Item = moxy::parse!("type T<U> where U: A = Vec<U>;").unwrap();
    assert_eq!(moxy::fmt!(&item).unwrap(), "type T<U>\nwhere\nU: A = Vec<U>;");
}

#[test]
fn only_a_trait_impl_is_negative() {
    let item: Item = moxy::parse!("impl !A for B {}").unwrap();
    assert_eq!(item.to_token_stream().to_string(), "impl ! A for B {}");
    assert!(moxy::parse!("impl !A {}" as Item).is_err());
}

#[test]
fn impl_types_and_functions_may_lack_a_type_or_body_as_rustc_parses_them() {
    for source in [
        "impl X { type Item; }",
        "impl A { fn f(); }",
        "impl A { fn f() {} type T = u8; }",
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(item.to_token_stream().to_string(), lexed.to_string(), "`{source}`");
        let formatted = moxy::fmt!(&item).unwrap();
        assert!(moxy::parse!(formatted as Item).is_ok(), "`{source}`");
    }
}

#[test]
fn a_colon_without_bounds_formats() {
    for (source, formatted) in [
        ("type T:;", "type T:;"),
        ("trait A { type T:; }", "trait A {\n\ttype T:;\n}"),
        ("extern \"C\" { type T:; }", "extern \"C\" {\n\ttype T:;\n}"),
        ("type T: A = u8;", "type T: A = u8;"),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(moxy::fmt!(&item).unwrap(), formatted, "`{source}`");
    }
}

#[test]
fn impl_trait_and_extern_members_take_the_forms_rustc_parses() {
    // rustc's parser reads the members of the three kinds of block with one
    // grammar and rejects most of these later
    let accepted = [
        ("impl A", "const X: u8;"),
        ("impl A", "type T: B = u8;"),
        ("impl A", "type T: B where Self: A;"),
        ("trait A", "default const X: u8 = 1;"),
        ("trait A", "default type T = u8;"),
        ("trait A", "pub default unsafe fn f() {}"),
        ("extern \"C\"", "default type T = u8;"),
        ("extern \"C\"", "default fn f() {}"),
        ("extern \"C\"", "static X: u8 = 1;"),
    ];
    let rejected = [
        ("extern \"C\"", "const X: u8;"),
        ("extern \"C\"", "default static X: u8;"),
        ("impl A", "default static X: u8;"),
        ("trait A", "default pub fn f();"),
    ];

    for (block, member) in accepted {
        let source = format!("{block} {{ {member} }}");
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let expected: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(item.to_token_stream().to_string(), expected.to_string());
        let formatted = moxy::fmt!(&item).unwrap();
        let again: Item = moxy::parse!(formatted).unwrap_or_else(|error| panic!("`{formatted}`: {error}"));
        assert_eq!(again.to_token_stream().to_string(), expected.to_string());
    }

    for (block, member) in rejected {
        let source = format!("{block} {{ {member} }}");
        assert!(moxy::parse!(source as Item).is_err(), "`{source}`");
    }
}

#[test]
fn a_bang_without_a_type_after_it_is_the_never_type_of_an_impl() {
    for source in ["impl ! {}", "impl<T> ! where T: A {}"] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let implementation = item.as_impl().unwrap();
        assert!(
            implementation.self_ty.is_never() && implementation.trait_ref.is_none(),
            "`{source}`"
        );
    }

    let item: Item = moxy::parse!("impl !Send for X {}").unwrap();
    assert!(item.as_impl().unwrap().trait_ref.is_some());

    for source in ["impl !A {}", "impl ! for X {}"] {
        assert!(moxy::parse!(source as Item).is_err(), "`{source}`");
    }
}
