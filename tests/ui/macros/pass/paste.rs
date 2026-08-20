use moxy::ast::Item;

moxy::paste! {
    struct {{ Generated Record }} {
        {{ field_ value }}: u32,
    }
}

fn main() {
    let value = GeneratedRecord { field_value: 7 };
    assert_eq!(value.field_value, 7);
    let item: Item = moxy::parse!("struct GeneratedRecord { field_value: u32 }").unwrap();
    let structure = item.as_struct().unwrap();
    assert_eq!(structure.ident.text(), "GeneratedRecord");
    assert_eq!(moxy::fmt!(&item).unwrap(), "struct GeneratedRecord {\n\tfield_value: u32,\n}");
}
