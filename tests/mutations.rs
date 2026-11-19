//! Truncated and damaged sources lex and parse to a result, never a panic or a
//! hang. The fuzz targets under `fuzz/` search further; this check runs with
//! the other tests.

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::mpsc;
use std::time::Duration;

use moxy::ast::{Expr, File};
use moxy::token::{ToTokenStream, TokenStream};

const SOURCES: [&str; 5] = [
    include_str!("../fixtures/parse/expr/control_flow.rs"),
    include_str!("../fixtures/parse/expr/postfix_async.rs"),
    include_str!("../fixtures/parse/item/macro_heavy.rs"),
    include_str!("../fixtures/parse/item/mixed_items.rs"),
    include_str!("../fixtures/parse/item/pattern_heavy.rs"),
];

// Every prefix, every suffix, and the source without each one character
fn mutations(source: &str) -> Vec<String> {
    let bounds: Vec<usize> = source.char_indices().map(|(index, _)| index).chain([source.len()]).collect();
    let mut mutations = Vec::new();

    for (index, &bound) in bounds.iter().enumerate() {
        mutations.push(source[..bound].to_string());
        mutations.push(source[bound..].to_string());

        if let Some(&next) = bounds.get(index + 1) {
            mutations.push(format!("{}{}", &source[..bound], &source[next..]));
        }
    }

    mutations
}

// Lexes and parses one source in every way a caller would, and emits and
// formats what parses
fn exercise(source: &str) {
    let Ok(tokens) = source.parse::<TokenStream>() else {
        return;
    };

    let _ = tokens.to_string();

    if let Ok(file) = moxy::parse!(tokens as File) {
        let _ = file.to_token_stream().to_string();
        let _ = moxy::fmt!(&file);
    }

    if let Ok(expression) = moxy::parse!(source as Expr) {
        let _ = expression.to_token_stream().to_string();
        let _ = moxy::fmt!(&expression);
    }
}

// The longest any one source may take; a hang fails the test with the
// source instead of stalling it
const LIMIT: Duration = Duration::from_secs(10);

#[test]
fn truncated_and_damaged_sources_neither_panic_nor_hang() {
    let sources: Vec<String> = SOURCES.iter().flat_map(|source| mutations(source)).collect();
    let (sender, receiver) = mpsc::channel();
    let worker_sources = sources.clone();

    std::thread::spawn(move || {
        for source in worker_sources {
            let panicked = catch_unwind(AssertUnwindSafe(|| exercise(&source))).is_err();

            if sender.send(panicked).is_err() {
                return;
            }
        }
    });

    let mut panics = Vec::new();

    for source in &sources {
        match receiver.recv_timeout(LIMIT) {
            Ok(true) => panics.push(source.as_str()),
            Ok(false) => {}
            Err(_) => panic!("took longer than {LIMIT:?} on:\n{source}"),
        }
    }

    assert!(panics.is_empty(), "panics on:\n{}", panics.join("\n---\n"));
}
