use moxy::ast::Item;

#[test]
fn const_items_preserve_visibility_type_and_value() {
    let item: Item = moxy::parse!("pub const LIMIT:usize=8;").unwrap();
    let constant = item.as_const().unwrap();
    assert_eq!(constant.ident.text(), "LIMIT");
    assert!(constant.vis.is_public());
    assert_eq!(constant.expr.to_token_stream().to_string(), "8");
    assert_eq!(moxy::fmt!(&item).unwrap(), "pub const LIMIT: usize = 8;");
}

#[test]
fn mutable_static_items_render_exactly() {
    let item: Item = moxy::parse!("pub static mut VALUE:u64=5;").unwrap();
    let static_item = item.as_static().unwrap();
    assert_eq!(static_item.ident.text(), "VALUE");
    assert!(matches!(static_item.mutability, moxy::ast::Mutability::Mutable(_)));
    assert_eq!(moxy::fmt!(&item).unwrap(), "pub static mut VALUE: u64 = 5;");
}

use moxy::token::ToTokenStream;
