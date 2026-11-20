//! Lexing any text, as tokens or as a source file, returns tokens or an
//! error, never a panic or a hang.

#![no_main]

use libfuzzer_sys::fuzz_target;
use moxy::token::TokenStream;
use moxy::token::source::SourceMap;

fuzz_target!(|text: &str| {
    // Each input releases the sources it lexed, so memory stays flat
    let _sources = SourceMap::scope();

    if let Ok(tokens) = text.parse::<TokenStream>() {
        let _ = tokens.to_string();
    }

    if let Ok(tokens) = TokenStream::from_source_file(text.to_owned()) {
        let _ = tokens.to_string();
    }
});
