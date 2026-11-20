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

#[test]
fn round_tripped_groups_keep_their_whole_span() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/trybuild/spans/fail/round_trip_*.rs");
}

#[test]
fn macro_input_nested_past_the_depth_limit_is_a_compile_error() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/trybuild/spans/pass/depth_*.rs");
    cases.compile_fail("tests/trybuild/spans/fail/depth_*.rs");
}

// The sources a macro lexes are released after each expansion, so the
// compiler's thread holds as many for the second as for the first
#[test]
fn macro_expansions_keep_no_sources() {
    assert_eq!(moxy_macro_tests::lexed_sources!(), moxy_macro_tests::lexed_sources!());
}

// A group the template builds around user tokens takes the call site, as
// proc_macro's groups do, so an error on the whole group points at the macro
// call rather than into the user's tokens
#[test]
fn generated_groups_take_the_call_site() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/trybuild/spans/fail/generated_parentheses.rs");
}
