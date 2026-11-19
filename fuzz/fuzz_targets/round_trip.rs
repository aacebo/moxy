//! A file that parses emits tokens that lex and parse again into the same
//! tokens.

#![no_main]

use libfuzzer_sys::fuzz_target;
use moxy::ast::File;
use moxy::token::{ToTokenStream, TokenStream};

fuzz_target!(|text: &str| {
    let Ok(file) = moxy::parse!(text as File) else {
        return;
    };

    let emitted = file.to_token_stream().to_string();
    let relexed: TokenStream = emitted
        .parse()
        .unwrap_or_else(|error| panic!("emitted `{emitted}` does not lex: {error:?}"));
    let reparsed = moxy::parse!(relexed as File).unwrap_or_else(|error| panic!("emitted `{emitted}` does not parse: {error}"));
    assert_eq!(reparsed.to_token_stream().to_string(), emitted);
});
