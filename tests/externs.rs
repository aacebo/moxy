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

#[test]
fn foreign_statics_keep_their_safety_qualifier() {
    for (source, tokens, formatted) in [
        (
            "unsafe extern \"C\" { pub safe static A: u8; }",
            "unsafe extern \"C\" {pub safe static A : u8 ;}",
            "unsafe extern \"C\" {\n\tpub safe static A: u8;\n}",
        ),
        (
            "unsafe extern { unsafe static mut B: u8; }",
            "unsafe extern {unsafe static mut B : u8 ;}",
            "unsafe extern {\n\tunsafe static mut B: u8;\n}",
        ),
        (
            "extern \"C\" { static C: u8; }",
            "extern \"C\" {static C : u8 ;}",
            "extern \"C\" {\n\tstatic C: u8;\n}",
        ),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(moxy::token::ToTokenStream::to_token_stream(&item).to_string(), tokens);
        assert_eq!(moxy::fmt!(&item).unwrap(), formatted);
    }
}

#[test]
fn abi_strings_format_and_emit_as_written() {
    for (source, value) in [(r#"extern "a\"b" fn f() {}"#, "a\"b"), (r##"extern r#"C"# fn f() {}"##, "C")] {
        let item: Item = moxy::parse!(source).unwrap();
        let abi = item.as_fn().unwrap().sig.abi.as_ref().unwrap();
        assert_eq!(abi.name.as_ref().unwrap().value(), value);
        assert_eq!(moxy::fmt!(&item).unwrap(), source);
        let emitted = moxy::token::ToTokenStream::to_token_stream(abi).to_string();
        assert_eq!(emitted, source[..source.find(" fn").unwrap()]);
        assert_eq!(
            moxy::token::Spanner::span(abi).byte_range().len(),
            source.find(" fn").unwrap()
        );
    }
}
