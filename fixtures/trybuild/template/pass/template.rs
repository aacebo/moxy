use moxy::ast::Item;

fn main() {
    let name = "Generated";
    let tokens = moxy::template! { struct {{ name }}; };
    let item: Item = moxy::parse!(tokens).unwrap();
    let structure = item.as_struct().unwrap();
    assert_eq!(structure.ident.text(), "Generated");
    assert!(structure.fields.is_unit());
    assert_eq!(moxy::fmt!(&item).unwrap(), "struct Generated;");
}
