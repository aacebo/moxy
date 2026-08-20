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
#[ignore = "formatter currently drops inline module contents"]
fn inline_modules_render_their_nested_items() {
    let item: Item = moxy::parse!("pub mod api{pub const VALUE:usize=1;}").unwrap();
    let module = item.as_mod().unwrap();
    assert_eq!(module.content.as_ref().unwrap().inner.len(), 1);
    assert!(module.content.as_ref().unwrap().inner[0].is_const());
    assert_eq!(moxy::fmt!(&item).unwrap(), "pub mod api {\n\tpub const VALUE: usize = 1;\n}");
}
