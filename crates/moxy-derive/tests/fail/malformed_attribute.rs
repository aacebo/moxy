use moxy::ToTokens;

#[derive(ToTokens)]
#[moxy(template = "not a code block")]
struct Malformed;

fn main() {}
