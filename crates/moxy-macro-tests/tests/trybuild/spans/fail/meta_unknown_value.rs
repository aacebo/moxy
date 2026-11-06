#[derive(moxy_macro_tests::Range)]
#[range(min = 1, max = 2, nope = 3)]
struct Foo {}

fn main() {}
