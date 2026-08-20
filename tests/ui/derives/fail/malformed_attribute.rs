use moxy::ToTokens;

#[derive(ToTokens)]
#[template = "not a code block"]
struct Malformed;

fn main() {}
