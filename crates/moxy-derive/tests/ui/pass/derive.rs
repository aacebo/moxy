use moxy_derive::ToTokens;
use moxy_token::ToTokenStream;

#[derive(ToTokens)]
#[template { const VALUE: &str = {{ self.value }}; }]
struct Generated {
    value: String,
}

fn main() {
    assert_eq!(
        Generated { value: "seven".into() }.to_token_stream().to_string(),
        "const VALUE : & str = \"seven\" ;"
    );
}
