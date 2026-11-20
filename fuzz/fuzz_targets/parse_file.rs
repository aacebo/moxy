//! Parsing any text as a file returns a tree or an error, never a panic or a
//! hang, and formatting a tree never panics.

#![no_main]

use libfuzzer_sys::fuzz_target;
use moxy::ast::File;
use moxy::token::TokenStream;

fuzz_target!(|text: &str| {
    let Ok(tokens) = TokenStream::from_source_file(text.to_owned()) else {
        return;
    };

    if let Ok(file) = moxy::parse!(tokens as File) {
        let _ = moxy::fmt!(&file);
    }
});
