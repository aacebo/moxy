use moxy::ast::Crate;
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
        .with_indent(moxy::fmt::Indent::space(2))
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
