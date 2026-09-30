#[derive(moxy::FromMeta)]
struct Args {
    #[meta(message = true)]
    value: bool,
}

fn main() {}
