use moxy::ast::Item;

#[test]
fn extern_crate_aliases_are_parsed_and_rendered() {
    let item: Item = moxy::parse!("extern crate core as rust_core;").unwrap();
    let external = item.as_extern_crate().unwrap();
    assert_eq!(external.ident.text(), "core");
    assert_eq!(external.rename.as_ref().unwrap().text(), "rust_core");
    assert_eq!(moxy::fmt!(&item).unwrap(), "extern crate core as rust_core;");
}

#[test]
fn foreign_modules_preserve_functions_statics_types_and_macros() {
    let item: Item =
        moxy::parse!("extern \"C\"{fn call(value:usize)->usize;static mut VALUE:usize;type Output;invoke!();}").unwrap();
    let foreign = item.as_foreign_mod().unwrap();
    assert!(foreign.abi.name.is_some());
    assert_eq!(foreign.items.inner.len(), 4);
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "extern \"C\" {\n\tfn call(value: usize) -> usize;\n\t\n\tstatic mut VALUE: usize;\n\t\n\ttype Output;\n\t\n\tinvoke!();\n}"
    );
}
