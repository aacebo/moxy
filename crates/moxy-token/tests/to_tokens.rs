use moxy_token::ToTokenStream;

#[test]
fn string_slices_and_owned_strings_emit_equivalent_literals() {
    for value in ["hello world", "it's done", "say \"hi"] {
        let borrowed = value.to_token_stream();
        let owned = value.to_owned().to_token_stream();

        assert_eq!(borrowed, owned);
        assert_eq!(borrowed.to_string(), format!("{value:?}"));
    }
}
