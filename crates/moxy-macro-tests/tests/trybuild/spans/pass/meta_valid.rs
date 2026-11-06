#[derive(moxy_macro_tests::Range)]
#[range(min = 1, max = 2)]
struct Both {}

#[derive(moxy_macro_tests::Range)]
#[range(max = 2)]
struct DefaultMin {}

const _: () = {
    assert!(!Both::contains(0));
    assert!(Both::contains(1));
    assert!(Both::contains(2));
    assert!(!Both::contains(3));
    assert!(DefaultMin::contains(0));
    assert!(DefaultMin::contains(2));
    assert!(!DefaultMin::contains(3));
};

fn main() {}
