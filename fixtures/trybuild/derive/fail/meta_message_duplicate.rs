#[derive(moxy::Meta)]
struct Args {
    #[meta(message = "first", message = "second")]
    value: bool,
}

fn main() {}
