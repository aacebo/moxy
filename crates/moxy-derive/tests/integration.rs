use moxy_ast::Item;
use moxy_derive::ToTokens;
use moxy_fmt::fmt;
use moxy_token::ToTokenStream;

#[derive(ToTokens)]
#[template {
    pub const GENERATED: &str = {{ self.value }};
}]
struct Model {
    value: String,
}

#[test]
fn derive_generates_parseable_and_formattable_tokens() {
    let tokens = Model { value: "ready".into() }.to_token_stream();
    let item: Item = moxy_token::parse!(tokens).unwrap();
    assert_eq!(fmt!(&item).unwrap(), "pub const GENERATED: &str = \"ready\";");
}
