#[derive(moxy::FromMeta)]
struct Args {
    #[meta(message = "first", message = "second")]
    value: bool,
}

fn main() {}
