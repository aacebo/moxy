use moxy::ast::{Crate, File, Item, ItemStruct, Lifetime, Visit, VisitMut, walk_item_struct};
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

#[test]
fn visitors_reach_every_lifetime() {
    struct Lifetimes(Vec<String>);

    impl<'ast> Visit<'ast> for Lifetimes {
        fn visit_lifetime(&mut self, node: &'ast Lifetime) {
            self.0.push(node.to_token_stream().to_string());
        }
    }

    struct Named(usize);

    impl VisitMut for Named {
        fn visit_lifetime_mut(&mut self, node: &mut Lifetime) {
            if node.to_token_stream().to_string() == "'_" {
                let name = format!("'p{}", self.0);
                *node = moxy::parse!(name).unwrap();
                self.0 += 1;
            }
        }
    }

    let item: Item = moxy::parse!(
        "fn f<'a: 'b, 'b>(x: &'a str, y: Cow<'_, str>, z: &'_ dyn Trait + 'b) -> impl Sized + use<'a, T> where 'b: 'a, T: for<'c> Fn(&'c u8) { 'outer: loop { break 'outer; continue 'outer; } }"
    )
    .unwrap();
    let mut lifetimes = Lifetimes(Vec::new());
    lifetimes.visit_item(&item);
    // Generic parameters, the where clause (part of the generics), the inputs, the `use` bound of
    // the output, the loop label, and the `break` and `continue` targets
    assert_eq!(
        lifetimes.0,
        [
            "'a", "'b", "'b", "'b", "'a", "'c", "'c", "'a", "'_", "'_", "'b", "'a", "'outer", "'outer", "'outer"
        ]
    );

    let mut item = item;
    let mut named = Named(0);
    named.visit_item_mut(&mut item);
    assert_eq!(named.0, 2);
    assert!(
        item.to_token_stream()
            .to_string()
            .contains("y : Cow < 'p0 , str > , z : & 'p1 dyn Trait + 'b")
    );
}

#[test]
fn visitors_reach_the_fields_and_base_of_a_struct_literal() {
    struct Paths(Vec<String>);

    impl<'ast> moxy::ast::Visit<'ast> for Paths {
        fn visit_expr_path(&mut self, node: &'ast moxy::ast::ExprPath) {
            self.0.push(node.to_token_stream().to_string());
        }
    }

    struct Rename;

    impl moxy::ast::VisitMut for Rename {
        fn visit_expr_path_mut(&mut self, node: &mut moxy::ast::ExprPath) {
            node.path = moxy::parse!("renamed").unwrap();
        }
    }

    let mut expression: moxy::ast::Expr = moxy::parse!("S { a: first, b, ..base }").unwrap();
    let mut paths = Paths(Vec::new());
    moxy::ast::Visit::visit_expr(&mut paths, &expression);
    assert_eq!(paths.0, ["first", "b", "base"]);

    moxy::ast::VisitMut::visit_expr_mut(&mut Rename, &mut expression);
    assert_eq!(expression.to_token_stream().to_string(), "S {a : renamed , b , .. renamed}");
}
