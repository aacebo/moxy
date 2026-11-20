//! Parsing any text as a file returns a tree or an error, never a panic or a
//! hang, and formatting a tree never panics.

#![no_main]

use libfuzzer_sys::fuzz_target;
use moxy::ast::File;
use moxy::token::TokenStream;
use moxy::token::source::SourceMap;

fuzz_target!(|text: &str| {
    // Each input releases the sources it lexed, so memory stays flat
    let _sources = SourceMap::scope();

    let Ok(tokens) = TokenStream::from_source_file(text.to_owned()) else {
        return;
    };

    if let Ok(file) = moxy::parse!(tokens as File) {
        let _ = moxy::fmt!(&file);
    }
});
