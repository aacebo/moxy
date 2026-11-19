//! Differential checks against syn. Each source is parsed by both parsers and
//! compared by tree shape, rendered with every operation in parentheses, and by
//! emitted tokens. The corpus check runs with the other tests; a directory of
//! Rust files is checked on demand:
//!
//! ```text
//! MOXY_DIFF_DIR=path/to/sources cargo test --all-features --test all differential::directory -- --ignored --nocapture
//! ```

mod canon;

use std::collections::BTreeSet;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};

use moxy::ast as m;
use moxy::token::{Spanner, ToTokenStream, TokenStream};
use quote::ToTokens;

use canon::*;

const CORPUS: &str = include_str!("corpus.txt");
const KNOWN: &str = include_str!("known.txt");

// The canonical tree and the emitted tokens of one source
type Rendering = Result<(String, String), String>;

fn moxy_render(kind: &str, source: &str) -> Rendering {
    fn go<T: m::Parse + ToTokenStream>(source: &str, canon: fn(&T) -> String) -> Rendering {
        let node: T = moxy::parse!(source).map_err(|error| error.message().to_string())?;
        Ok((canon(&node), strip(&node.to_token_stream().to_string())))
    }

    match kind {
        "e" => go::<m::Expr>(source, mexpr),
        "b" => go::<m::StmtBlock>(source, mblock),
        "p" => go::<m::Pattern>(source, mpat),
        "t" => go::<m::Type>(source, mtype),
        "i" => go::<m::Item>(source, mitem),
        "f" => go::<m::File>(source, |file| {
            let items = file.items.iter().map(mitem).collect::<Vec<_>>().join("\n");
            format!("file{}\n{items}", mattrs(&file.attrs))
        }),
        _ => panic!("unknown kind `{kind}`"),
    }
}

fn syn_render(kind: &str, source: &str) -> Rendering {
    fn go<T: ToTokens>(parsed: syn::Result<T>, canon: fn(&T) -> String) -> Rendering {
        let node = parsed.map_err(|error| error.to_string())?;
        Ok((canon(&node), strip(&node.to_token_stream().to_string())))
    }

    match kind {
        "e" => go(syn::parse_str::<syn::Expr>(source), sexpr),
        "b" => go(syn::parse_str::<syn::Block>(source), sblock),
        "p" => go(
            syn::parse::Parser::parse_str(syn::Pat::parse_multi_with_leading_vert, source),
            spat,
        ),
        "t" => go(syn::parse_str::<syn::Type>(source), stype),
        "i" => go(syn::parse_str::<syn::Item>(source), sitem),
        "f" => go(syn::parse_file(source), |file| {
            let items = file.items.iter().map(sitem).collect::<Vec<_>>().join("\n");
            format!("file{}\n{items}", sattrs(&file.attrs))
        }),
        _ => panic!("unknown kind `{kind}`"),
    }
}

// How moxy differs from syn on one source, or `None` when they agree
fn difference(kind: &str, source: &str) -> Option<&'static str> {
    let moxy = catch_unwind(AssertUnwindSafe(|| moxy_render(kind, source)));
    let syn = syn_render(kind, source);

    match (moxy, syn) {
        (Err(_), _) => Some("PANIC"),
        (Ok(Err(_)), Err(_)) => None,
        (Ok(Err(_)), Ok(_)) => Some("REJECT"),
        (Ok(Ok(_)), Err(_)) => Some("ACCEPT"),
        (Ok(Ok((moxy_tree, moxy_tokens))), Ok((syn_tree, syn_tokens))) => {
            if moxy_tree != syn_tree && !syn_tree.contains(SYN_VERBATIM) {
                Some("TREE")
            } else if moxy_tokens != syn_tokens {
                Some("TOKENS")
            } else {
                None
            }
        }
    }
}

fn entries(text: &str) -> impl Iterator<Item = &str> {
    text.lines()
        .map(str::trim_end)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
}

#[test]
fn corpus() {
    let known: BTreeSet<&str> = entries(KNOWN).collect();
    let mut unexpected = Vec::new();
    let mut found = BTreeSet::new();

    for line in entries(CORPUS) {
        let (kind, source) = line
            .split_once(": ")
            .unwrap_or_else(|| panic!("`{line}` is not `kind: source`"));
        let source = source.replace("\\n", "\n");

        if let Some(tag) = difference(kind, &source) {
            let entry = format!("{tag} {line}");

            if !known.contains(entry.as_str()) {
                let moxy = catch_unwind(AssertUnwindSafe(|| moxy_render(kind, &source)));
                let syn = syn_render(kind, &source);
                unexpected.push(format!("{entry}\n    moxy: {moxy:?}\n    syn:  {syn:?}"));
            }

            found.insert(entry);
        }
    }

    let stale: Vec<&str> = known.iter().copied().filter(|entry| !found.contains(*entry)).collect();

    assert!(
        unexpected.is_empty() && stale.is_empty(),
        "differences not in known.txt:\n{}\n\nentries in known.txt that no longer differ:\n{}",
        unexpected.join("\n"),
        stale.join("\n"),
    );
}

#[test]
#[ignore = "set MOXY_DIFF_DIR to a directory of Rust files"]
fn directory() {
    let root = std::env::var("MOXY_DIFF_DIR").expect("MOXY_DIFF_DIR names a directory of Rust files");
    let mut paths = Vec::new();
    collect_rust_files(Path::new(&root), &mut paths);
    paths.sort();

    let (mut files, mut items) = (0, 0);
    let mut failures = Vec::new();

    for path in &paths {
        let Ok(source) = std::fs::read_to_string(path) else {
            continue;
        };

        // Only files syn accepts and moxy lexes say anything about the parser
        let Ok(expected) = syn::parse_file(&source) else {
            continue;
        };

        if source.parse::<TokenStream>().is_err() {
            continue;
        }

        files += 1;

        let parsed = catch_unwind(AssertUnwindSafe(|| moxy::parse!(source as m::File)));
        let file = match parsed {
            Err(_) => {
                failures.push(format!("{}: panics", path.display()));
                continue;
            }
            Ok(Err(error)) => {
                failures.push(format!(
                    "{}:{}: {}",
                    path.display(),
                    error.span().start().line(),
                    error.message()
                ));
                continue;
            }
            Ok(Ok(file)) => file,
        };

        if file.items.len() != expected.items.len() {
            failures.push(format!(
                "{}: {} items, syn has {}",
                path.display(),
                file.items.len(),
                expected.items.len()
            ));
            continue;
        }

        for (item, expected) in file.items.iter().zip(&expected.items) {
            items += 1;

            let (actual, expected) = (mitem(item), sitem(expected));

            if actual != expected && !expected.contains(SYN_VERBATIM) {
                failures.push(format!("{}:{}: tree differs", path.display(), item.span().start().line()));
            }
        }
    }

    println!("{files} files ({items} top-level items) compared, {} differ", failures.len());

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
