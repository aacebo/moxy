use moxy::ToTokens;
use moxy::ast::Item;
use moxy::token::{Spanner, ToTokenStream};

#[derive(ToTokens)]
#[moxy(template { const VALUE: &str = {{ self.value }}; })]
struct Generated {
    value: String,
}

fn main() {
    let tokens = Generated { value: "seven".into() }.to_token_stream();
    let item: Item = moxy::parse!(tokens).unwrap();
    let constant = item.as_const().unwrap();
    assert_eq!(constant.ident.text(), "VALUE");
    assert!(!constant.span().is_empty());
    assert_eq!(moxy::fmt!(&item).unwrap(), "const VALUE: &str = \"seven\";");
}
