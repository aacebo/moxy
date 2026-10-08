#[test]
fn proc_macro2_interpolated_errors_retain_their_source_span() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("fixtures/trybuild/bridge/fail/proc_macro2_span.rs");
}
