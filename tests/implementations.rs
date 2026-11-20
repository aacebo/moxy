use moxy::ast::Item;

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
fn implementations_preserve_where_clauses_when_formatted() {
    let item: Item = moxy::parse!("impl<T> Example<T> where T: Copy { fn value(&self) -> T { self.0 } }").unwrap();
    let implementation = item.as_impl().unwrap();

    assert!(implementation.generics.where_clause.is_some());
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "impl<T> Example<T>\nwhere\nT: Copy {\n\tfn value(&self) -> T {\n\t\tself.0\n\t}\n}"
    );
}
