use moxy_template::paste;

paste! {
    fn {{ generated_ name }}() -> u32 { 7 }
}

fn main() {
    assert_eq!(generated_name(), 7);
}

