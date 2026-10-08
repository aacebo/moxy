use moxy::ast::Item;

fn main() {
    let name = moxy::ident!("Generated");
    let tokens = moxy::template! { struct {{ name }}; };
    let item: Item = moxy::parse!(tokens).unwrap();
    let structure = item.as_struct().unwrap();
    assert_eq!(structure.ident.text(), "Generated");
    assert!(structure.fields.is_unit());
    assert_eq!(moxy::fmt!(&item).unwrap(), "struct Generated;");

    let value = Some("enabled");
    let fields = [(
        moxy::ident!("name"),
        moxy::ident!("String"),
    )];
    let tokens = moxy::template! {
        @if let Some(value) = value { const VALUE: &str = {{ value }}; }
        @for ((name, ty) in &fields) { struct {{ name }} { value: {{ ty }} } }
        {{
            let rendered = "done".to_string();
            rendered
        }}
    };
    assert!(tokens.to_string().contains("enabled"));
}
