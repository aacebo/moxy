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
fn leading_paths_and_single_names_render_without_extra_spaces() {
    let item: Item = moxy::parse!("use ::core::fmt;").unwrap();
    assert_eq!(moxy::fmt!(&item).unwrap(), "use ::core::fmt;");
}

#[test]
fn use_paths_accept_path_keywords() {
    for (source, expected) in [
        ("use crate::a;", "use crate::a;"),
        ("use self::a;", "use self::a;"),
        ("use super::super::a;", "use super::super::a;"),
        ("use super::{A, B as C};", "use super::{A, B as C};"),
        ("use a::{self, b};", "use a::{self, b};"),
        ("use a::{self as c};", "use a::{self as c};"),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(moxy::fmt!(&item).unwrap(), expected);
    }
}

#[test]
fn extern_crate_self_names_the_current_crate() {
    let item: Item = moxy::parse!("extern crate self as name;").unwrap();
    assert_eq!(moxy::fmt!(&item).unwrap(), "extern crate self as name;");
}

#[test]
fn crate_only_begins_a_use_path() {
    let error = moxy::parse!("use a::crate;" as Item).err().unwrap();
    assert_eq!(error.message(), "`crate` can only begin a path");
}
