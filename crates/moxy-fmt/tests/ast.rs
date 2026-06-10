use moxy_ast::{Crate, Expr, Item, Pattern, Stmt, Type};
use moxy_fmt::{FmtConfig, NewlineStyle, fmt};

fn format<T: moxy_fmt::Format>(src: &str) -> String
where
    T: moxy_token::Parse,
{
    let value: T = moxy_token::parse!(src).unwrap();
    fmt!(&value, FmtConfig::default().with_newline(NewlineStyle::Unix)).unwrap()
}

fn idempotent<T: moxy_fmt::Format>(src: &str)
where
    T: moxy_token::Parse,
{
    let first = format::<T>(src);
    let second = format::<T>(&first);
    assert_eq!(first, second, "not idempotent for: {src}");
}

// ── Type ──────────────────────────────────────────────────────────────────────

#[test]
fn type_generic_no_spaces() {
    // Parser strips whitespace around angle brackets
    assert_eq!(format::<Type>("Vec<u8>"), "Vec<u8>");
}

#[test]
fn type_ref_mut_with_generic() {
    assert_eq!(format::<Type>("&mut Vec<u8>"), "&mut Vec<u8>");
    idempotent::<Type>("&mut Vec<u8>");
}

#[test]
fn type_multi_bound_missing_spaces() {
    // `+` separator gets spaces added on both sides
    assert_eq!(format::<Type>("impl Clone+Debug+Send"), "impl Clone + Debug + Send");
    idempotent::<Type>("impl Clone + Debug + Send");
}

#[test]
fn type_dyn_multi_bound() {
    assert_eq!(format::<Type>("dyn Clone+Debug"), "dyn Clone + Debug");
    idempotent::<Type>("dyn Clone + Debug");
}

// ── Pattern ───────────────────────────────────────────────────────────────────

#[test]
fn pat_tuple_no_spaces() {
    // Comma-separated elements get spaces after commas
    assert_eq!(format::<Pattern>("(a,b,c)"), "(a, b, c)");
    idempotent::<Pattern>("(a, b, c)");
}

#[test]
fn pat_or_no_spaces() {
    // `|` separator gets spaces on both sides
    assert_eq!(format::<Pattern>("A|B|C"), "A | B | C");
    idempotent::<Pattern>("A | B | C");
}

#[test]
fn pat_tuple_struct_no_spaces() {
    assert_eq!(format::<Pattern>("Some(x)"), "Some(x)");
    assert_eq!(format::<Pattern>("Ok(v)"), "Ok(v)");
}

#[test]
fn pat_slice_no_spaces() {
    assert_eq!(format::<Pattern>("[a,b,c]"), "[a, b, c]");
    idempotent::<Pattern>("[a, b, c]");
}

// ── Expr ──────────────────────────────────────────────────────────────────────

#[test]
fn expr_binary_no_spaces() {
    // Spaces around binary operators
    assert_eq!(format::<Expr>("a+b"), "a + b");
    idempotent::<Expr>("a + b");
}

#[test]
fn expr_call_no_spaces() {
    // Spaces after commas in arg list
    assert_eq!(format::<Expr>("foo(a,b,c)"), "foo(a, b, c)");
    idempotent::<Expr>("foo(a, b, c)");
}

#[test]
fn expr_match_inline_to_broken() {
    // Inline match gets broken into one arm per line
    assert_eq!(
        format::<Expr>("match x{Ok(v)=>v+1,Err(e)=>0,_=>42}"),
        "match x {\n\tOk(v) => v + 1,\n\tErr(e) => 0,\n\t_ => 42,\n}"
    );
    idempotent::<Expr>("match x {\n\tOk(v) => v + 1,\n\tErr(e) => 0,\n\t_ => 42,\n}");
}

#[test]
fn expr_if_else_inline_to_broken() {
    assert_eq!(format::<Expr>("if x>0{x}else{0}"), "if x > 0 {\n\tx\n} else {\n\t0\n}");
    idempotent::<Expr>("if x > 0 {\n\tx\n} else {\n\t0\n}");
}

#[test]
fn expr_nested_binary() {
    assert_eq!(format::<Expr>("a+b*c-d"), "a + b * c - d");
}

// ── Stmt ──────────────────────────────────────────────────────────────────────

#[test]
fn stmt_let_no_spaces() {
    // Spaces around `:` and `=`
    assert_eq!(format::<Stmt>("let x:u32=1;"), "let x: u32 = 1;");
    idempotent::<Stmt>("let x: u32 = 1;");
}

#[test]
fn stmt_let_mut_binding() {
    assert_eq!(format::<Stmt>("let mut count:u32=0;"), "let mut count: u32 = 0;");
    idempotent::<Stmt>("let mut count: u32 = 0;");
}

// ── Item ──────────────────────────────────────────────────────────────────────

#[test]
fn item_fn_compact_to_broken() {
    // Compact fn with inline body → indented body on new line
    assert_eq!(
        format::<Item>("pub fn add(a:u32,b:u32)->u32{a+b}"),
        "pub fn add(a: u32, b: u32) -> u32 {\n\ta + b\n}"
    );
    idempotent::<Item>("pub fn add(a: u32, b: u32) -> u32 {\n\ta + b\n}");
}

#[test]
fn item_fn_async_generic() {
    assert_eq!(
        format::<Item>("pub async fn fetch<T:Clone>(url:&str)->Option<T>{None}"),
        "pub async fn fetch<T: Clone>(url: &str) -> Option<T> {\n\tNone\n}"
    );
    idempotent::<Item>("pub async fn fetch<T: Clone>(url: &str) -> Option<T> {\n\tNone\n}");
}

#[test]
fn item_struct_compact_to_broken() {
    // Inline struct fields → each field on its own indented line
    assert_eq!(
        format::<Item>("pub struct Point{x:f64,y:f64}"),
        "pub struct Point {\n\tx: f64,\n\ty: f64,\n}"
    );
    idempotent::<Item>("pub struct Point {\n\tx: f64,\n\ty: f64,\n}");
}

#[test]
fn item_struct_with_generics_and_lifetime() {
    assert_eq!(
        format::<Item>("pub struct Request<'a,T:Clone>{url:&'a str,body:T,status:u32}"),
        "pub struct Request<'a, T: Clone> {\n\turl: &'a str,\n\tbody: T,\n\tstatus: u32,\n}"
    );
    idempotent::<Item>("pub struct Request<'a, T: Clone> {\n\turl: &'a str,\n\tbody: T,\n\tstatus: u32,\n}");
}

#[test]
fn item_enum_compact_to_broken() {
    // Inline enum variants → each variant on its own indented line
    assert_eq!(
        format::<Item>("pub enum Color{Red,Green,Blue}"),
        "pub enum Color {\n\tRed,\n\tGreen,\n\tBlue,\n}"
    );
    idempotent::<Item>("pub enum Color {\n\tRed,\n\tGreen,\n\tBlue,\n}");
}

#[test]
fn item_enum_with_data() {
    assert_eq!(
        format::<Item>("pub enum Msg<T,E>{Ok(T),Err(E)}"),
        "pub enum Msg<T, E> {\n\tOk(T),\n\tErr(E),\n}"
    );
    idempotent::<Item>("pub enum Msg<T, E> {\n\tOk(T),\n\tErr(E),\n}");
}

#[test]
fn item_trait_compact_to_broken() {
    assert_eq!(
        format::<Item>("pub trait Animal:Clone{type Name;fn name(&self)->&str;}"),
        "pub trait Animal: Clone {\n\ttype Name;\n\tfn name(&self) -> &str;\n}"
    );
    idempotent::<Item>("pub trait Animal: Clone {\n\ttype Name;\n\tfn name(&self) -> &str;\n}");
}

#[test]
fn item_use_group_no_spaces() {
    // Spaces after commas inside use group
    assert_eq!(format::<Item>("use std::{fmt,collections};"), "use std::{fmt, collections};");
    idempotent::<Item>("use std::{fmt, collections};");
}

#[test]
fn item_use_group_rename() {
    assert_eq!(
        format::<Item>("use std::{fmt,collections,io};"),
        "use std::{fmt, collections, io};"
    );
    idempotent::<Item>("use std::{fmt, collections, io};");
}

#[test]
fn item_impl_empty() {
    assert_eq!(format::<Item>("impl Foo{}"), "impl Foo {}");
    idempotent::<Item>("impl Foo {}");
}

#[test]
fn item_impl_for_trait() {
    assert_eq!(format::<Item>("impl Clone for Foo{}"), "impl Clone for Foo {}");
}

// ── Crate ─────────────────────────────────────────────────────────────────────

#[test]
fn crate_items_get_blank_lines() {
    // Items run together without blank lines → blank lines inserted between them
    assert_eq!(
        format::<Crate>("use std::fmt;const N:usize=8;type Foo=u32;"),
        "use std::fmt;\n\nconst N: usize = 8;\n\ntype Foo = u32;"
    );
    idempotent::<Crate>("use std::fmt;\n\nconst N: usize = 8;\n\ntype Foo = u32;");
}

#[test]
fn crate_struct_with_use() {
    assert_eq!(
        format::<Crate>("use std::fmt;pub struct Foo{x:u32,y:String}"),
        "use std::fmt;\n\npub struct Foo {\n\tx: u32,\n\ty: String,\n}"
    );
    idempotent::<Crate>("use std::fmt;\n\npub struct Foo {\n\tx: u32,\n\ty: String,\n}");
}
