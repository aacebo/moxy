//! Parsing any text as a file returns a tree or an error, never a panic or a
//! hang, and formatting a tree never panics.

#![no_main]

use libfuzzer_sys::fuzz_target;
use moxy::ast::File;

fuzz_target!(|text: &str| {
    if let Ok(file) = moxy::parse!(text as File) {
        let _ = moxy::fmt!(&file);
    }
});
