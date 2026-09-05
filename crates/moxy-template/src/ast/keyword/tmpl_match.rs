#![allow(unused)]

use moxy_ast::{Delimited, Parse, ParseError, Parser, Token};
use moxy_token::{Delim, Group, LexError, Span, ToTokenStream, ToTokens, TokenStream, TokenTree};

use crate::Template;

#[doc = "A template match directive: `@match (expr) { pat => { body }, … }`."]
#[derive(Debug, Clone)]
pub struct TmplMatch {
    pub span: Span,
    pub at: Token![@],
    pub keyword: Token![match],
    pub expr: TokenStream,
    pub arms: Delimited<Vec<TmplMatchArm>>,
}

impl TmplMatch {
    pub fn parse_after_keyword_match(parser: &Parser, at: Token![@], keyword: Token![match]) -> Result<Self, ParseError> {
        let span = at.span();
        let expr = parser.parse_group(Delim::Paren)?;
        let arms = Delimited::parse_brace(parser)?;

        Ok(Self {
            span,
            at,
            keyword,
            expr,
            arms,
        })
    }
}

impl ToTokens for TmplMatch {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.keyword.to_tokens(tokens);
        self.expr.to_tokens(tokens);
        self.arms.to_tokens(tokens);
    }
}

#[doc = "A single arm of a `@match` directive: `pat => { body }`."]
#[derive(Debug, Clone)]
pub struct TmplMatchArm {
    pub span: Span,
    pub pat: TokenStream,
    pub arrow: Token![=>],
    pub body: Delimited<Template>,
    pub comma: Option<Token![,]>,
}

impl Parse for TmplMatchArm {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let span = parser.span();
        let mut pat = TokenStream::new();

        loop {
            if parser.peek::<Token![=>]>() {
                break;
            }

            match parser.curr() {
                None => return Err(LexError::new(span).message("unexpected end of match arm").into()),
                _ => {
                    pat.extend_one(parser.advance().unwrap().clone());
                }
            }
        }

        let arrow = parser.parse::<Token![=>]>()?;
        let body = Delimited::parse_brace(parser)?;
        let comma = parser.parse_if::<Token![,]>();

        Ok(Self {
            span,
            pat,
            arrow,
            body,
            comma,
        })
    }
}

impl ToTokens for TmplMatchArm {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pat.to_tokens(tokens);
        self.arrow.to_tokens(tokens);
        self.body.to_tokens(tokens);
        self.comma.to_tokens(tokens);
    }
}
