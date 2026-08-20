use moxy::ast::Item;

#[test]
fn union_fields_and_generics_are_inspectable_and_rendered() {
    let item: Item = moxy::parse!("pub union Storage<T>{integer:u64,value:T}").unwrap();
    let union = item.as_union().unwrap();
    assert_eq!(union.ident.text(), "Storage");
    assert_eq!(union.generics.params.len(), 1);
    assert_eq!(union.fields.fields.inner.len(), 2);
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "pub union Storage<T> {\n\tinteger: u64,\n\tvalue: T,\n}"
    );
}
