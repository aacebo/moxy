//! A round-trip check over a directory of Rust files, run on demand:
//!
//! ```text
//! MOXY_ROUND_TRIP_DIR=path/to/sources cargo test --all-features --test all round_trip -- --ignored --nocapture
//! ```
//!
//! Each `.rs` file under the directory, searched recursively, is parsed as a
//! `File` and emitted again. The check fails when a file that lexes fails to
//! parse, or when its emitted tokens differ from the lexed tokens. Files the
//! lexer rejects are counted and skipped, since they test the lexer, not the
//! parser.

use std::path::{Path, PathBuf};

use moxy::ast::File;
use moxy::token::{ToTokenStream, TokenStream};

#[test]
#[ignore = "set MOXY_ROUND_TRIP_DIR to a directory of Rust files"]
fn round_trip() {
    let root = std::env::var("MOXY_ROUND_TRIP_DIR").expect("MOXY_ROUND_TRIP_DIR names a directory of Rust files");
    let mut paths = Vec::new();
    collect_rust_files(Path::new(&root), &mut paths);
    paths.sort();

    let (mut files, mut items, mut unlexed) = (0, 0, 0);
    let mut failures = Vec::new();

    for path in &paths {
        let source = std::fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        let Ok(lexed) = TokenStream::from_source_file(source) else {
            unlexed += 1;
            continue;
        };

        files += 1;

        match moxy::parse!((&lexed) as File) {
            Err(error) => failures.push(format!(
                "{}:{}: does not parse: {}",
                path.display(),
                error.span().start().line(),
                error.message()
            )),
            Ok(file) => {
                items += file.items.len();

                if file.to_token_stream().to_string() != lexed.to_string() {
                    failures.push(format!("{}: emits different tokens", path.display()));
                }
            }
        }
    }

    println!(
        "{files} files ({items} top-level items) checked, {} failed, {unlexed} skipped by the lexer",
        failures.len()
    );

    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn collect_rust_files(dir: &Path, paths: &mut Vec<PathBuf>) {
    let entries = std::fs::read_dir(dir).unwrap_or_else(|error| panic!("{}: {error}", dir.display()));

    for entry in entries {
        let path = entry.unwrap().path();

        if path.is_dir() {
            collect_rust_files(&path, paths);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            paths.push(path);
        }
    }
}
