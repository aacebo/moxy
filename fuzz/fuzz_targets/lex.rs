//! Lexing any text, as tokens or as a source file, returns tokens or an
//! error, never a panic or a hang.

#![no_main]

use libfuzzer_sys::fuzz_target;
use moxy::token::TokenStream;

fuzz_target!(|text: &str| {
    if let Ok(tokens) = text.parse::<TokenStream>() {
        let _ = tokens.to_string();
    }

    if let Ok(tokens) = TokenStream::from_source_file(text.to_owned()) {
        let _ = tokens.to_string();
    }
});
