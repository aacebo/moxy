//! Lexing any text returns tokens or an error, never a panic or a hang.

#![no_main]

use libfuzzer_sys::fuzz_target;
use moxy::token::TokenStream;

fuzz_target!(|text: &str| {
    if let Ok(tokens) = text.parse::<TokenStream>() {
        let _ = tokens.to_string();
    }
});
