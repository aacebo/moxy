use moxy::ast::Item;

#[test]
fn declared_modules_preserve_visibility_and_semicolon() {
    let item: Item = moxy::parse!("pub mod api;").unwrap();
    let module = item.as_mod().unwrap();
    assert_eq!(module.ident.text(), "api");
    assert!(module.content.is_none());
    assert_eq!(moxy::fmt!(&item).unwrap(), "pub mod api;");
}

#[test]
fn inline_modules_render_their_nested_items() {
    let item: Item = moxy::parse!("pub mod api{pub const VALUE:usize=1;}").unwrap();
    let module = item.as_mod().unwrap();
    assert_eq!(module.content.as_ref().unwrap().items.inner.len(), 1);
    assert!(module.content.as_ref().unwrap().items.inner[0].is_const());
    assert_eq!(moxy::fmt!(&item).unwrap(), "pub mod api {\n\tpub const VALUE: usize = 1;\n}");
}

#[test]
fn a_module_body_holds_its_inner_attributes() {
    use moxy::token::ToTokenStream;

    let item: Item = moxy::parse!("mod m { #![a] fn f() {} }").unwrap();
    let content = item.as_mod().unwrap().content.as_ref().unwrap();
    assert_eq!(content.inner_attrs.len(), 1);
    assert_eq!(content.items.inner.len(), 1);
    assert_eq!(content.to_token_stream().to_string(), "{# ! [a] fn f () {}}");

    let content: moxy::ast::ItemModContent = moxy::parse!("{ #![a] }").unwrap();
    assert_eq!(content.inner_attrs.len(), 1);
    assert!(content.items.inner.is_empty());
}
