use std::str::FromStr;

use zynix::ast::Punctuated;
use zynix::parse::{ParseError, ParseStream};
use zynix::token::punct::{Comma, Lt, PathSep};
use zynix::token::{LexError, ToTokenStream, ToTokens, Token, TokenTree};
use zynix::{Parse, Span, TokenStream};
use zynix_macros::{Parse, ToTokens};

/// Minimal Parse-able identifier wrapper, so these tests don't depend on
/// the (separately-landed) `ast::Ident` Parse impl.
#[derive(Debug, Clone)]
struct Ident(zynix::token::Ident);

impl Ident {
    fn name(&self) -> std::borrow::Cow<'_, str> {
        self.0.name()
    }
}

impl Parse for Ident {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        match stream.advance() {
            Some(TokenTree::Token(Token::Ident(id))) => Ok(Ident(id.clone())),
            _ => Err(LexError::new(at).message("expected identifier").into()),
        }
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

fn parse<T: Parse>(src: &str) -> T {
    let ts = TokenStream::from_str(src).unwrap();
    let mut ps = ts.parse();
    ps.parse::<T>().unwrap()
}

fn render<T: ToTokenStream>(value: &T) -> String {
    value.to_token_stream().to_string()
}

#[derive(Parse, ToTokens)]
struct Pair {
    #[parse(skip)]
    #[allow(dead_code)]
    span: Span,
    first: Ident,
    second: Ident,
}

#[derive(Parse, ToTokens)]
struct Leading {
    #[parse(peek = PathSep)]
    leading: bool,
    #[parse(separated)]
    segments: Punctuated<Ident, PathSep>,
}

#[derive(Parse, ToTokens)]
struct Maybe {
    name: Ident,
    rest: Option<Ident>,
}

#[derive(Parse, ToTokens)]
struct Wrapped {
    #[parse(paren)]
    inner: Ident,
}

#[derive(Parse, ToTokens)]
struct Many {
    names: Vec<Ident>,
}

#[derive(Parse, ToTokens)]
enum Choice {
    #[parse(peek = Lt)]
    Angled {
        #[parse(prefix = Lt)]
        inner: Ident,
    },
    Bare {
        name: Ident,
    },
}

#[test]
fn struct_skip_and_fields() {
    let p: Pair = parse("a b");
    assert_eq!(p.first.name().as_ref(), "a");
    assert_eq!(p.second.name().as_ref(), "b");
    assert_eq!(render(&p), "a b");
}

#[test]
fn peek_toggle_present() {
    let l: Leading = parse("::a::b");
    assert!(l.leading);
    assert_eq!(l.segments.len(), 2);
    assert_eq!(render(&l), ":: a :: b");
}

#[test]
fn peek_toggle_absent() {
    let l: Leading = parse("a::b::c");
    assert!(!l.leading);
    assert_eq!(l.segments.len(), 3);
    assert_eq!(render(&l), "a :: b :: c");
}

#[test]
fn option_present_and_absent() {
    let some: Maybe = parse("a b");
    assert!(some.rest.is_some());

    let none: Maybe = parse("a");
    assert!(none.rest.is_none());
    assert_eq!(render(&none), "a");
}

#[test]
fn vec_greedy() {
    let m: Many = parse("a b c");
    assert_eq!(m.names.len(), 3);
    assert_eq!(render(&m), "a b c");

    let empty: Many = parse("");
    assert_eq!(empty.names.len(), 0);
}

#[test]
fn group_rewraps_on_emit() {
    let w: Wrapped = parse("(x)");
    assert_eq!(w.inner.name().as_ref(), "x");
    assert_eq!(render(&w), "(x)");
}

#[test]
fn enum_dispatch() {
    let angled: Choice = parse("<x");
    assert!(matches!(angled, Choice::Angled { .. }));
    assert_eq!(render(&angled), "< x");

    let bare: Choice = parse("y");
    assert!(matches!(bare, Choice::Bare { .. }));
    assert_eq!(render(&bare), "y");
}

#[allow(dead_code)]
fn _comma_used(_: Comma) {}
