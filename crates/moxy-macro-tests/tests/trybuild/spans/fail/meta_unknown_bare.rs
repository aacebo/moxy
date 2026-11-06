#[derive(moxy_macro_tests::Range)]
#[range(min = 1, max = 2, nope)]
struct Foo {}

fn main() {}
