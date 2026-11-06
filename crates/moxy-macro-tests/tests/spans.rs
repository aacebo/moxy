#[test]
fn proc_macro2_interpolated_errors_retain_their_source_span() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/trybuild/spans/pass/nonzero_u8.rs");
    cases.compile_fail("tests/trybuild/spans/fail/nonzero_u8_*.rs");
}

#[test]
fn derived_meta_errors_retain_their_source_span() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/trybuild/spans/pass/meta_*.rs");
    cases.compile_fail("tests/trybuild/spans/fail/meta_*.rs");
}
