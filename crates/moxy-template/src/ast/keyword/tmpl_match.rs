#![allow(unused)]

use moxy_ast::{Cursor, Delimited, Parse, ParseError, Parser, Token};
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

impl Parse for TmplMatch {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(cursor) = cursor.skip::<Token![@]>() else {
            return false;
        };

        cursor.peek::<Token![match]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let at: Token![@] = parser.parse()?;
        let keyword = parser.parse()?;
        let span = at.span();
        let expr = parser.parse_group(Delim::Paren)?.to_token_stream();
        let arms = Delimited::parse_brace(parser)?;

        Ok(Self {
            span,
            at,
            keyword,
            expr,
            arms,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let cursor = cursor.skip::<Token![@]>()?;
        let cursor = cursor.skip::<Token![match]>()?;
        let inner = cursor.descend(Delim::Paren)?;
        let cursor = inner.offset(inner.remaining()).is_empty().then(|| cursor.offset(1))?;
        let mut inner = cursor.descend(Delim::Brace)?;

        while !inner.is_empty() {
            inner = inner.skip::<TmplMatchArm>()?;
        }

        Some(cursor.offset(1))
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
    fn peek(cursor: Cursor<'_>) -> bool {
        Self::skip(cursor).is_some()
    }

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

        let arrow = parser.parse()?;
        let body = Delimited::parse_brace(parser)?;
        let comma = parser.parse()?;

        Ok(Self {
            span,
            pat,
            arrow,
            body,
            comma,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        while !cursor.peek::<Token![=>]>() {
            cursor.curr()?;
            cursor = cursor.offset(1);
        }

        cursor = cursor.skip::<Token![=>]>()?;
        let inner = cursor.descend(Delim::Brace)?;
        let inner = Template::skip(inner)?;

        if !inner.is_empty() {
            return None;
        }

        cursor = cursor.offset(1);
        cursor.skip::<Option<Token![,]>>()
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
