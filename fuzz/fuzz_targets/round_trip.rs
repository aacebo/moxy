//! A file that parses emits tokens that lex and parse again into the same
//! tokens.

#![no_main]

use libfuzzer_sys::fuzz_target;
use moxy::ast::File;
use moxy::token::source::SourceMap;
use moxy::token::{ToTokenStream, TokenStream};

fuzz_target!(|text: &str| {
    // Each input releases the sources it lexed, so memory stays flat
    let _sources = SourceMap::scope();

    let Ok(tokens) = TokenStream::from_source_file(text.to_owned()) else {
        return;
    };

    let Ok(file) = moxy::parse!(tokens as File) else {
        return;
    };

    let emitted = file.to_token_stream().to_string();
    let relexed: TokenStream = emitted
        .parse()
        .unwrap_or_else(|error| panic!("emitted `{emitted}` does not lex: {error:?}"));
    let reparsed = moxy::parse!(relexed as File).unwrap_or_else(|error| panic!("emitted `{emitted}` does not parse: {error}"));
    assert_eq!(reparsed.to_token_stream().to_string(), emitted);
});
