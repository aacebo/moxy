use moxy_template::template;

fn main() {
    let name = "value";
    let tokens = template! { let {{ name }} = 1; };
    assert_eq!(tokens.to_string(), "let value = 1 ;");
}

