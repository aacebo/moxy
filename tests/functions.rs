use moxy::ast::Item;
use moxy::token::ToTokenStream;

#[test]
fn function_signatures_and_bodies_complete_the_syntax_pipeline() {
    let item: Item = moxy::parse!("pub fn add(a:u32,b:u32)->u32{a+b}").unwrap();
    let function = item.as_fn().unwrap();
    assert_eq!(function.sig.ident.text(), "add");
    assert_eq!(function.sig.params.inner.inputs.len(), 2);
    assert!(function.sig.output.is_type());
    assert_eq!(function.body.as_ref().unwrap().stmts.len(), 1);
    assert_eq!(moxy::fmt!(&item).unwrap(), "pub fn add(a: u32, b: u32) -> u32 {\n\ta + b\n}");
}

#[test]
fn async_generic_functions_preserve_bounds_references_and_return_types() {
    let item: Item = moxy::parse!("pub async fn fetch<T:Clone>(url:&str)->Option<T>{None}").unwrap();
    let function = item.as_fn().unwrap();
    assert!(function.sig.asyncness.is_some());
    assert_eq!(function.sig.generics.params.len(), 1);
    assert_eq!(function.sig.params.inner.inputs.len(), 1);
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "pub async fn fetch<T: Clone>(url: &str) -> Option<T> {\n\tNone\n}"
    );
}

#[test]
fn unsafe_extern_variadic_functions_render_exactly() {
    let item: Item = moxy::parse!("pub unsafe extern \"C\" fn log(format:*const u8,...)->i32{0}").unwrap();
    let function = item.as_fn().unwrap();
    assert!(function.sig.safety.is_some());
    assert!(function.sig.abi.is_some());
    assert!(function.sig.params.inner.variadic.is_some());
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "pub unsafe extern \"C\" fn log(format: *const u8, ...) -> i32 {\n\t0\n}"
    );
}

#[test]
fn parameters_after_a_path_generic_argument_are_kept() {
    let item: Item = moxy::parse!("fn f(x: Vec<a::B>, y: u8) -> u8 { y }").unwrap();
    let function = item.as_fn().unwrap();
    assert_eq!(function.sig.params.inner.inputs.len(), 2);
    assert_eq!(moxy::fmt!(&item).unwrap(), "fn f(x: Vec<a::B>, y: u8) -> u8 {\n\ty\n}");
}

#[test]
fn typed_self_receivers_keep_their_type() {
    for (source, expected) in [
        ("fn f(self: Box<Self>, x: u8) {}", "fn f(self: Box<Self>, x: u8) {}"),
        ("fn f(mut self: Box<Self>) {}", "fn f(mut self: Box<Self>) {}"),
        ("fn f(self: &Self) {}", "fn f(self: &Self) {}"),
        ("fn f(self: std::rc::Rc<Self>) {}", "fn f(self: std::rc::Rc<Self>) {}"),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let function = item.as_fn().unwrap();
        let moxy::ast::FnParam::Receiver(receiver) = function.sig.params.inner.inputs.first().unwrap() else {
            panic!("`{source}`: the first parameter is not a receiver");
        };
        assert!(receiver.ty.is_some(), "`{source}`");
        assert_eq!(moxy::fmt!(&item).unwrap(), expected);
    }
}

#[test]
fn weak_keywords_are_identifiers_outside_their_keyword_position() {
    for (source, tokens) in [
        ("fn default() {}", "fn default () {}"),
        (
            "fn f() { x.default; self.raw == 1; }",
            "fn f () {x . default ; self . raw == 1 ;}",
        ),
        (
            "fn f() { S { raw: false }; union(); }",
            "fn f () {S {raw : false} ; union () ;}",
        ),
        (
            "struct S { default: u8, raw: u8, union: u8, safe: u8, auto: u8 }",
            "struct S {default : u8 , raw : u8 , union : u8 , safe : u8 , auto : u8}",
        ),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(moxy::token::ToTokenStream::to_token_stream(&item).to_string(), tokens);
    }

    // In their keyword positions they still declare
    let union: Item = moxy::parse!("union U { a: u8 }").unwrap();
    let auto_trait: Item = moxy::parse!("unsafe auto trait A {}").unwrap();
    assert!(union.is_union());
    assert!(auto_trait.as_trait().unwrap().auto_keyword.is_some());
}

#[test]
fn weak_keywords_begin_paths_in_use_items_types_and_expressions() {
    for source in [
        "use crate::raw::Bucket;",
        "pub use core::default::Default;",
        "use raw::{default, union as u};",
        "fn f(x: raw::A) -> default {}",
        "impl default::A {}",
        "fn f() { let macro_rules = 1; macro_rules + 1; }",
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(
            moxy::token::ToTokenStream::to_token_stream(&item).to_string(),
            lexed.to_string()
        );
    }

    // Followed by `!`, `macro_rules` still defines a macro
    let item: Item = moxy::parse!("macro_rules! m { () => {} }").unwrap();
    assert!(item.is_macro2());
}

#[test]
fn an_abi_keeps_the_value_of_its_string() {
    for (source, abi, emitted) in [
        ("extern \"C\" fn f() {}", "C", "extern \"C\" fn f () {}"),
        ("extern \"\\0C\" fn f() {}", "\0C", "extern \"\\0C\" fn f () {}"),
        ("extern r\"C\" fn f() {}", "C", "extern r\"C\" fn f () {}"),
    ] {
        let item: moxy::ast::Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let function = item.as_fn().unwrap();
        assert_eq!(function.sig.abi.as_ref().unwrap().name.as_ref().unwrap().value(), abi);
        assert_eq!(item.to_token_stream().to_string(), emitted);
    }
}

#[test]
fn a_variadic_may_be_named_by_a_pattern() {
    for (source, emitted) in [
        (
            "unsafe extern \"C\" fn f(a: u8, mut args: ...) {}",
            "unsafe extern \"C\" fn f (a : u8 , mut args : ...) {}",
        ),
        (
            "unsafe extern \"C\" fn f(a: u8, _: ...) {}",
            "unsafe extern \"C\" fn f (a : u8 , _ : ...) {}",
        ),
        (
            "type B = unsafe extern \"C\" fn(_: u8, va: ...);",
            "type B = unsafe extern \"C\" fn (_ : u8 , va : ...) ;",
        ),
        ("extern \"C\" {fn f(a: u8, ...);}", "extern \"C\" {fn f (a : u8 , ...) ;}"),
    ] {
        let item: moxy::ast::Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(item.to_token_stream().to_string(), emitted);
        assert_eq!(moxy::fmt!(&item).unwrap().replace(['\n', '\t'], ""), source);
    }
}

#[test]
fn only_trait_functions_take_parameters_without_a_name() {
    for source in [
        "trait A { fn f(u8); }",
        "trait A { fn f(&u8) {} }",
        "fn f(_: u8) {}",
        "type F = fn(u8);",
    ] {
        assert!(moxy::parse!(source as moxy::ast::File).is_ok(), "`{source}`");
    }

    for source in ["fn f(u8) {}", "impl A { fn f(u8) {} }", "extern \"C\" { fn f(u8); }"] {
        assert!(moxy::parse!(source as moxy::ast::File).is_err(), "`{source}`");
    }
}

#[test]
fn a_variadic_takes_a_trailing_comma() {
    use moxy::token::ToTokenStream;

    for source in [
        "extern \"C\" { fn a(_: *mut u8, ...,); }",
        "extern \"C\" { fn c(_: *mut u8, #[cfg(never)] [w, t, f]: ...,); }",
        "type F = unsafe extern \"C\" fn(u8, ...,);",
    ] {
        let item: moxy::ast::Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let lexed: moxy::token::TokenStream = source.parse().unwrap();
        assert_eq!(item.to_token_stream().to_string(), lexed.to_string(), "`{source}`");
        let formatted = moxy::fmt!(&item).unwrap();
        assert!(moxy::parse!(formatted as moxy::ast::Item).is_ok(), "`{formatted}`");
    }

    // As in syn, nothing follows the variadic but its comma
    assert!(moxy::parse!("extern \"C\" { fn f(a: u8, ..., b: u8); }" as moxy::ast::Item).is_err());
}
