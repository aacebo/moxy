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
    fn go<T: m::Parse + ToTokenStream>(tokens: &TokenStream, canon: fn(&T) -> String) -> Rendering {
        let node: T = moxy::parse!(tokens).map_err(|error| error.message().to_string())?;
        Ok((canon(&node), strip(&node.to_token_stream().to_string())))
    }

    // A file is read as the text of a source file, as `syn::parse_file` reads
    // one
    if kind == "f" {
        let file = m::File::from_source_file(source.to_owned()).map_err(|error| error.message().to_string())?;
        let items = file.items.iter().map(mitem).collect::<Vec<_>>().join("\n");
        let tree = format!("file{}\n{items}", mattrs(&file.attrs));
        return Ok((tree, strip(&file.to_token_stream().to_string())));
    }

    let tokens = TokenStream::from_string(source.to_owned()).map_err(|error| error.to_string())?;

    match kind {
        "e" => go::<m::Expr>(&tokens, mexpr),
        "b" => go::<m::StmtBlock>(&tokens, mblock),
        "p" => go::<m::Pattern>(&tokens, mpat),
        "t" => go::<m::Type>(&tokens, mtype),
        "i" => go::<m::Item>(&tokens, mitem),
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

// The style of every error message: lowercase with no closing period, code in
// backticks (double ones with spaces around code that holds a backtick), no
// type names (an acronym such as `ASCII` is fine), no article after
// any "expected", and an "expected" message names what it found: code, or one
// of the forms `end of input`, `end of group`, `empty group`, `trait alias`,
// `no value` (a meta with no value after its name), and `no rules`
pub(crate) fn follows_style(message: &str) -> bool {
    // The prose of a message, without its quoted code
    let mut prose = String::new();
    let mut rest = message;

    while let Some(start) = rest.find('`') {
        prose.push_str(&rest[..start]);
        let (close, skip) = if rest[start..].starts_with("`` ") {
            (" ``", 3)
        } else {
            ("`", 1)
        };
        let Some(end) = rest[start + skip..].find(close) else {
            return false;
        };
        rest = &rest[start + skip + end + close.len()..];
    }

    prose.push_str(rest);

    let quoted = |found: &str| {
        (found.starts_with("`` ") && found.ends_with(" ``"))
            || (found.len() > 1 && found.starts_with('`') && found.ends_with('`'))
    };
    let found = message.split_once(", found ").is_none_or(|(_, found)| {
        quoted(found)
            || [
                "end of input",
                "end of group",
                "empty group",
                "trait alias",
                "no value",
                "no rules",
            ]
            .contains(&found)
    });

    message.starts_with(|ch: char| ch.is_lowercase() || ch == '`')
        && !message.ends_with('.')
        && !prose.contains('"')
        && !prose.contains("received")
        && !prose.to_lowercase().contains("eof")
        && message
            .split("expected ")
            .skip(1)
            .all(|rest| !["a ", "an ", "the "].iter().any(|article| rest.starts_with(article)))
        && (!message.starts_with("expected ") || message.contains(", found "))
        && prose
            .split(|ch: char| !ch.is_alphanumeric())
            .all(|word| !word.contains(char::is_lowercase) || !word.contains(char::is_uppercase))
        && found
}

// Every message moxy gives for a corpus source it rejects follows the style
// of `follows_style`
#[test]
fn error_messages_follow_one_style() {
    let mut messages = 0;
    let mut outliers = Vec::new();

    for line in entries(CORPUS) {
        let (kind, source) = line.split_once(": ").unwrap();
        let source = source.replace("\\n", "\n");

        if let Ok(Err(message)) = catch_unwind(AssertUnwindSafe(|| moxy_render(kind, &source))) {
            messages += 1;

            if !follows_style(&message) {
                outliers.push(format!("{line}: {message}"));
            }
        }
    }

    assert!(messages > 100, "{messages} messages");
    assert!(outliers.is_empty(), "{}", outliers.join("\n"));
}

// At every token of every corpus line, `skip` ends where `parse` ends, or both
// fail, and `peek` holds wherever `parse` succeeds, for each node a hand-written
// `skip` exists for. They are kept by hand for speed: a skip that parsed would
// parse nested input again at each level of nesting.
#[test]
fn skip_and_peek_agree_with_parse() {
    fn check<T: m::Parse>(kind: &str, tokens: &TokenStream, line: &str, failures: &mut Vec<String>) {
        let root = m::Cursor::from_tokens(tokens);

        for index in 0..=tokens.len() {
            let cursor = root.seek(index);
            let parser = m::Parser::from_cursor(cursor);
            let parsed = T::parse(&parser).ok().map(|_| parser.cursor().remaining());
            let skipped = T::skip(cursor).map(|cursor| cursor.remaining());

            if parsed != skipped {
                failures.push(format!(
                    "{kind} skip at token {index}: parse {parsed:?}, skip {skipped:?}: {line}"
                ));
            }

            if parsed.is_some() && !T::peek(cursor) {
                failures.push(format!("{kind} peek at token {index}: {line}"));
            }
        }
    }

    let mut failures = Vec::new();

    for line in entries(CORPUS) {
        let (_, source) = line.split_once(": ").unwrap();
        let Ok(tokens) = source.replace("\\n", "\n").parse::<TokenStream>() else {
            continue;
        };

        check::<m::Expr>("expression", &tokens, line, &mut failures);
        check::<m::Pattern>("pattern", &tokens, line, &mut failures);
        check::<m::Type>("type", &tokens, line, &mut failures);
        check::<m::Item>("item", &tokens, line, &mut failures);
        check::<m::Stmt>("statement", &tokens, line, &mut failures);

        // A file parses from any position, empty if no item follows, so its
        // peek answers for the positions where it reads tokens
        let root = m::Cursor::from_tokens(&tokens);

        for index in 0..tokens.len() {
            let cursor = root.seek(index);
            let parser = m::Parser::from_cursor(cursor);
            let parsed = <m::File as m::Parse>::parse(&parser)
                .ok()
                .map(|_| parser.cursor().remaining());
            let skipped = <m::File as m::Parse>::skip(cursor).map(|cursor| cursor.remaining());

            if parsed != skipped {
                failures.push(format!(
                    "file skip at token {index}: parse {parsed:?}, skip {skipped:?}: {line}"
                ));
            }

            if parsed.is_some_and(|remaining| remaining < cursor.remaining()) && !<m::File as m::Parse>::peek(cursor) {
                failures.push(format!("file peek at token {index}: {line}"));
            }
        }
    }

    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

// Serialization keeps what tells two trees apart: two corpus sources that
// serialize alike have the same tree, rendered with every operator
#[cfg(feature = "serde")]
#[test]
fn sources_that_serialize_alike_have_the_same_tree() {
    fn serialized<T: m::Parse + serde::Serialize>(source: &str) -> Option<String> {
        let node: T = moxy::parse!(source).ok()?;
        Some(serde_json::to_string(&node).unwrap())
    }

    let mut seen = std::collections::BTreeMap::new();
    let mut collisions = Vec::new();

    for line in entries(CORPUS) {
        let (kind, source) = line.split_once(": ").unwrap();
        let source = source.replace("\\n", "\n");
        let json = match kind {
            "e" => serialized::<m::Expr>(&source),
            "b" => serialized::<m::StmtBlock>(&source),
            "p" => serialized::<m::Pattern>(&source),
            "t" => serialized::<m::Type>(&source),
            "i" => serialized::<m::Item>(&source),
            "f" => serialized::<m::File>(&source),
            _ => panic!("unknown kind `{kind}`"),
        };

        if let Some(json) = json {
            // A trailing separator is no part of the tree
            let tree = moxy_render(kind, &source).unwrap().0.replace(",}", "}");

            if let Some((other, other_tree)) = seen.insert((kind, json), (line, tree.clone()))
                && other_tree != tree
            {
                collisions.push(format!("{other}\n{line}"));
            }
        }
    }

    assert!(collisions.is_empty(), "{}", collisions.join("\n\n"));
}

// Two parses of one source at different positions compare equal, so tree
// equality depends on no span
#[test]
fn parses_of_one_source_compare_equal() {
    // The second parse reads the source shifted, so every span differs; a
    // byte order mark only starts a source, so it stays first
    fn check<T: m::Parse + PartialEq>(source: &str) -> bool {
        if source.starts_with('\u{feff}') {
            return true;
        }

        let shifted = format!("\n  {source}");

        match (moxy::parse!(source as T), moxy::parse!(shifted as T)) {
            (Ok(first), Ok(second)) => first == second,
            _ => true,
        }
    }

    let mut unequal = Vec::new();

    for line in entries(CORPUS) {
        let (kind, source) = line.split_once(": ").unwrap();
        let source = source.replace("\\n", "\n");
        let equal = match kind {
            "e" => check::<m::Expr>(&source),
            "b" => check::<m::StmtBlock>(&source),
            "p" => check::<m::Pattern>(&source),
            "t" => check::<m::Type>(&source),
            "i" => check::<m::Item>(&source),
            "f" => check::<m::File>(&source),
            _ => panic!("unknown kind `{kind}`"),
        };

        if !equal {
            unequal.push(line);
        }
    }

    assert!(unequal.is_empty(), "parses differ:\n{}", unequal.join("\n"));
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

        let Ok(tokens) = TokenStream::from_source_file(source) else {
            continue;
        };

        files += 1;

        let parsed = catch_unwind(AssertUnwindSafe(|| moxy::parse!((&tokens) as m::File)));
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

// Formatting is idempotent: the text formatted from a corpus source that
// parses parses again, and formats to the same text
#[test]
fn formatting_every_parsed_corpus_source_is_idempotent() {
    fn tokens<T: m::Parse>(source: &str) -> Option<T> {
        moxy::parse!(source).ok()
    }

    // A file is read as the text of a source file
    fn file(source: &str) -> Option<m::File> {
        m::File::from_source_file(source.to_owned()).ok()
    }

    fn again<T: moxy::fmt::Format>(source: &str, read: fn(&str) -> Option<T>) -> Option<Result<(), String>> {
        let node: T = read(source)?;
        let first = match moxy::fmt!(&node) {
            Ok(first) => first,
            Err(error) => return Some(Err(format!("fails to format: {error}"))),
        };
        let Some(reparsed) = read(&first) else {
            return Some(Err(format!("formats to `{first}`, which fails to parse")));
        };
        let second = moxy::fmt!(&reparsed).map_err(|error| error.to_string());

        Some(match second {
            Ok(second) if second == first => Ok(()),
            second => Err(format!("formats to `{first}`, then to `{second:?}`")),
        })
    }

    let mut failures = Vec::new();
    let mut checked = 0;

    for line in entries(CORPUS) {
        let (kind, source) = line.split_once(": ").unwrap();
        let source = source.replace("\\n", "\n");
        let outcome = match kind {
            "e" => again(&source, tokens::<m::Expr>),
            "b" => again(&source, tokens::<m::StmtBlock>),
            "p" => again(&source, tokens::<m::Pattern>),
            "t" => again(&source, tokens::<m::Type>),
            "i" => again(&source, tokens::<m::Item>),
            "f" => again(&source, file),
            _ => panic!("unknown kind `{kind}`"),
        };

        match outcome {
            Some(Ok(())) => checked += 1,
            Some(Err(failure)) => failures.push(format!("{line}: {failure}")),
            None => {}
        }
    }

    assert!(checked > 1_000, "{checked} sources");
    assert!(
        failures.is_empty(),
        "{} of {}:\n{}",
        failures.len(),
        checked + failures.len(),
        failures.join("\n")
    );
}
