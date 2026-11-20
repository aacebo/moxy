#[derive(moxy::FromMeta)]
struct Config {
    #[meta = 1]
    value: u8,
}

#[derive(moxy::FromMeta)]
enum Mode {
    #[meta]
    Default,
}

#[derive(moxy::FromMeta)]
struct Large {
    #[meta = 1 + "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz"]
    value: u8,
}

fn main() {}
