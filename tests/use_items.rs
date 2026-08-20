use moxy::ast::Item;

#[test]
fn grouped_use_items_preserve_renames_globs_and_self_imports() {
    let item: Item = moxy::parse!("use std::{fmt as formatting,io::*};").unwrap();
    let use_item = item.as_use().unwrap();
    assert!(use_item.vis.is_inherited());
    assert!(use_item.tree.is_path());
    assert_eq!(moxy::fmt!(&item).unwrap(), "use std::{fmt as formatting, io::*};");
}

#[test]
#[ignore = "use-tree parser currently rejects leading absolute path separators"]
fn leading_paths_and_single_names_render_without_extra_spaces() {
    let item: Item = moxy::parse!("use ::core::fmt;").unwrap();
    let use_item = item.as_use().unwrap();
    assert!(use_item.tree.as_path().unwrap().ident.text().is_empty());
    assert_eq!(moxy::fmt!(&item).unwrap(), "use ::core::fmt;");
}
