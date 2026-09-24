use moxy::ast::{Crate, File};
use moxy::token::{Spanner, ToTokenStream};

#[test]
fn crates_preserve_item_order_and_render_blank_lines() {
    let krate: Crate =
        moxy::parse!("use std::fmt; pub struct Point { x: i32, y: i32 } fn sum(point: Point) -> i32 { point.x + point.y }")
            .unwrap();
    assert_eq!(krate.items.len(), 3);
    assert!(krate.items[0].is_use());
    assert!(krate.items[1].is_struct());
    assert!(krate.items[2].is_fn());
    assert!(!krate.span().is_empty());
    assert!(!krate.to_token_stream().is_empty());
    assert_eq!(
        moxy::fmt!(&krate).unwrap(),
        "use std::fmt;\n\npub struct Point {\n\tx: i32,\n\ty: i32,\n}\n\nfn sum(point: Point) -> i32 {\n\tpoint.x + point.y\n}"
    );
}

#[test]
fn crate_rendering_obeys_indentation_and_newline_configuration() {
    let krate: Crate = moxy::parse!("struct Pair { left: u8, right: u8 }").unwrap();
    let config = moxy::fmt::FmtConfig::default()
        .with_indent(moxy::fmt::IndentStyle::space(2))
        .with_newline(moxy::fmt::NewlineStyle::Windows);
    assert_eq!(krate.items.len(), 1);
    assert!(krate.items[0].as_struct().unwrap().fields.is_named());
    assert!(!krate.to_token_stream().is_empty());
    assert_eq!(
        moxy::fmt!(&krate, config).unwrap(),
        "struct Pair {\r\n  left: u8,\r\n  right: u8,\r\n}"
    );
}

#[test]
fn visitors_traverse_real_crate_syntax_before_exact_rendering() {
    use moxy::ast::item::ItemStruct;
    use moxy::ast::visit::{Visit, walk_item_struct};

    struct StructVisitor {
        names: Vec<String>,
    }

    impl<'ast> Visit<'ast> for StructVisitor {
        fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
            self.names.push(node.ident.text().to_owned());
            walk_item_struct(self, node);
        }
    }

    let mut visitor = StructVisitor { names: Vec::new() };
    let krate: Crate =
        moxy::parse!("struct First { value: u8 } struct Second<T> { value: T } fn consume(value: Second<u8>) { let _ = value; }")
            .unwrap();

    for item in &krate.items {
        visitor.visit_item(item);
    }

    assert_eq!(visitor.names, ["First", "Second"]);
    assert_eq!(krate.items.len(), 3);
    assert!(!krate.to_token_stream().is_empty());
    assert_eq!(
        moxy::fmt!(&krate).unwrap(),
        "struct First {\n\tvalue: u8,\n}\n\nstruct Second<T> {\n\tvalue: T,\n}\n\nfn consume(value: Second<u8>) {\n\tlet _ = value;\n}"
    );
}

#[test]
fn files_preserve_shebangs_and_inner_attributes() {
    let source = "#!/usr/bin/env rustx\n#![allow(dead_code)]\nstruct Entry;";
    let file: File = moxy::parse!(source).unwrap();

    assert_eq!(file.shebang.as_ref().unwrap().ident.clone().unwrap().text(), "rustx");
    assert_eq!(file.attrs.len(), 1);
    assert_eq!(file.items.len(), 1);
    assert!(!file.span().is_empty());
    assert_eq!(
        moxy::fmt!(&file).unwrap(),
        "#!/usr/bin/env rustx\n#![allow(dead_code)]\nstruct Entry;"
    );
}

#[test]
fn files_accept_empty_and_inner_attribute_only_sources() {
    let empty: File = moxy::parse!("").unwrap();
    let attrs: File = moxy::parse!("#![allow(dead_code)]").unwrap();

    assert!(empty.shebang.is_none() && empty.attrs.is_empty() && empty.items.is_empty());
    assert!(attrs.shebang.is_none());
    assert_eq!(attrs.attrs.len(), 1);
}

#[test]
fn parse_files_returns_independent_files_and_errors() {
    let files = moxy::parse_files!("fixtures/trybuild/template/pass/*.rs");
    assert_eq!(files.len(), 2);
    assert!(files.iter().all(|file| !file.items.is_empty()));
    assert_eq!(files.iter().map(|file| file.items.len()).sum::<usize>(), 5);
    assert!(moxy::parse_files!("fixtures/does-not-exist/*.rs").is_empty());
}
