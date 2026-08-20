use moxy_ast::{Crate, Expr, Item, Pattern, Stmt, Type};
use moxy_fmt::{FmtConfig, Indent, NewlineStyle, fmt};

#[test]
fn expressions_format_reparse_and_remain_idempotent() {
    for source in [
        "Point{x:1,y,..base}",
        "async move{work().await?}",
        "unsafe{call()}",
        "loop{break 1}",
        "while let Some(x)=next(){consume(x);}",
        "for item in items{consume(item);}",
        "|x:i32|->i32{x+1}",
        "object.method::<T>(a,b)[index]",
        "if ready{yes()}else{no()}",
        "match value{Some(x)if x>0=>x,None=>0,_=>1}",
        "vec![1,2,3]",
    ] {
        let value: Expr = moxy_token::parse!(source).unwrap_or_else(|error| panic!("failed to parse {source}: {error}"));
        let first = fmt!(&value).unwrap();
        let reparsed: Expr =
            moxy_token::parse!(first.clone()).unwrap_or_else(|error| panic!("failed to reparse {first}: {error}"));
        let second = fmt!(&reparsed).unwrap();
        assert_eq!(first, second, "formatter was not idempotent for {source}");
    }
}

#[test]
fn patterns_and_types_format_reparse_and_remain_idempotent() {
    for source in [
        "ref mut name@Some(_)",
        "Point{x,y:renamed,..}",
        "[first,..,last]",
        "&mut value",
        "1..=10",
        "value:Option<T>",
        "const{1}",
    ] {
        let value: Pattern = moxy_token::parse!(source).unwrap_or_else(|error| panic!("failed to parse {source}: {error}"));
        let first = fmt!(&value).unwrap();
        let reparsed: Pattern =
            moxy_token::parse!(first.clone()).unwrap_or_else(|error| panic!("failed to reparse {first}: {error}"));
        assert_eq!(fmt!(&reparsed).unwrap(), first);
    }

    for source in [
        "!",
        "_",
        "std::collections::HashMap<String,Vec<u8>>",
        "&'a mut [T]",
        "*const T",
        "[u8;32]",
        "(A,B,C)",
        "impl Clone+Send+'static",
        "dyn Trait<Item=T>+Send+'a",
        "unsafe extern \"C\" fn(&str)->Result<T,E>",
        "m!(T)",
    ] {
        let value: Type = moxy_token::parse!(source).unwrap_or_else(|error| panic!("failed to parse {source}: {error}"));
        let first = fmt!(&value).unwrap();
        let reparsed: Type =
            moxy_token::parse!(first.clone()).unwrap_or_else(|error| panic!("failed to reparse {first}: {error}"));
        assert_eq!(fmt!(&reparsed).unwrap(), first);
    }
}

#[test]
fn statements_and_every_item_family_format_to_parseable_idempotent_text() {
    for source in [
        "let mut value:u64=1 else{return;};",
        "function();",
        "macro_call!();",
        "const LOCAL:usize=1;",
    ] {
        let value: Stmt = moxy_token::parse!(source).unwrap();
        let first = fmt!(&value).unwrap();
        let reparsed: Stmt = moxy_token::parse!(first.clone()).unwrap();
        assert_eq!(fmt!(&reparsed).unwrap(), first);
    }

    for source in [
        "use std::{fmt as formatting,io::*};",
        "extern crate core as rust_core;",
        "mod inline{pub const VALUE:usize=1;}",
        "mod external;",
        "pub extern \"C\" fn function<T:Clone>(value:T)->T{value}",
        "pub struct Named<T>{pub value:T,hidden:usize}",
        "pub struct Tuple(pub i32,String);",
        "pub struct Unit;",
        "pub enum Choice<T>{Unit,Tuple(T),Named{value:T}}",
        "pub union Storage{integer:u64,float:f64}",
        "pub unsafe auto trait Marker:Send{}",
        "pub trait Service<T>:Send{const LIMIT:usize=1;type Output:Clone;fn call(&self,value:T)->Self::Output;macro_call!();}",
        "pub trait Alias=Clone+Send;",
        "impl<T:Clone> Service<T> for Named<T>{const LIMIT:usize=2;type Output=T;fn call(&self,value:T)->T{value}macro_call!();}",
        "type AliasType<T>=Option<T>;",
        "const CONST_VALUE:usize=4;",
        "static mut STATIC_VALUE:usize=5;",
        "macro_call!(tokens);",
        "macro_rules! local_macro{()=>{};}",
        "extern \"C\"{static FOREIGN:u8;type ForeignType;fn foreign(value:i32)->i32;macro_call!();}",
    ] {
        let value: Item = moxy_token::parse!(source).unwrap_or_else(|error| panic!("failed to parse {source}: {error}"));
        let first = fmt!(&value).unwrap();
        let reparsed: Item =
            moxy_token::parse!(first.clone()).unwrap_or_else(|error| panic!("failed to reparse {first}: {error}"));
        assert_eq!(fmt!(&reparsed).unwrap(), first, "formatter was not idempotent for {source}");
    }
}

#[test]
fn crate_formatting_has_exact_indentation_newlines_and_blank_lines() {
    let value: Crate =
        moxy_token::parse!("use std::fmt;pub struct Point{x:i32,y:i32}fn sum(point:Point)->i32{point.x+point.y}").unwrap();
    let config = FmtConfig::default()
        .with_indent(Indent::space(2))
        .with_newline(NewlineStyle::Windows);
    let output = fmt!(&value, config).unwrap();
    assert_eq!(
        output,
        "use std::fmt;\r\n\r\npub struct Point {\r\n  x: i32,\r\n  y: i32,\r\n}\r\n\r\nfn sum(point: Point) -> i32 {\r\n  point.x + point.y\r\n}"
    );
    let reparsed: Crate = moxy_token::parse!(output.clone()).unwrap();
    assert_eq!(fmt!(&reparsed, config).unwrap(), output);
}
