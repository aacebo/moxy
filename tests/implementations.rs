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
        // The older place before `=` moves after the type
        (
            "impl A for B { type X<T> where T: Clone = Vec<T>; }",
            "impl A for B {type X < T > = Vec < T > where T : Clone ;}",
            "impl A for B {\n\ttype X<T> = Vec<T>\n\twhere\n\tT: Clone;\n}",
        ),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(item.to_token_stream().to_string(), tokens);
        assert_eq!(moxy::fmt!(&item).unwrap(), formatted);
    }
}
