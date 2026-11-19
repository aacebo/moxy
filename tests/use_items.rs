use moxy::ast::Item;
use moxy::token::ToTokenStream;

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

#[test]
fn empty_use_groups_parse() {
    for (source, tokens, formatted) in [
        ("use a::{};", "use a :: {} ;", "use a::{};"),
        ("use {};", "use {} ;", "use {};"),
        ("use a::{b::{}, c};", "use a :: {b :: {} , c} ;", "use a::{b::{}, c};"),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(item.to_token_stream().to_string(), tokens);
        assert_eq!(moxy::fmt!(&item).unwrap(), formatted);
    }
}

#[test]
fn use_trees_take_a_leading_path_separator() {
    for (source, formatted) in [
        ("use ::serde;", "use ::serde;"),
        ("use ::core as core2;", "use ::core as core2;"),
        ("use ::{a, b};", "use ::{a, b};"),
        ("use ::*;", "use ::*;"),
        ("use ::a::b;", "use ::a::b;"),
    ] {
        let item: moxy::ast::Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(item.to_token_stream().to_string(), lexed.to_string());
        assert_eq!(moxy::fmt!(&item).unwrap(), formatted);
    }

    // rustc's parser accepts `::` inside braces too, and rejects `use a::{::b}`
    // only when it resolves the path
    for source in ["use {::a::b, ::c::d};", "use a::{::b};"] {
        let item: moxy::ast::Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(moxy::fmt!(&item).unwrap(), source);
    }

    let item: moxy::ast::Item = moxy::parse!("use ::serde;").unwrap();
    let tree = &item.as_use().unwrap().tree;
    assert!(tree.as_name().unwrap().prefix.is_some());
}
