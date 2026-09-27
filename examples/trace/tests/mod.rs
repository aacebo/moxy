#[test]
fn matches() {
    #[trace::trace]
    fn do_something() -> u16 {
        255
    }

    assert_eq!(do_something(), 255);
}
