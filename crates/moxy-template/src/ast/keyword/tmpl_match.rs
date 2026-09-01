#![allow(unused)]

use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Delim, Group, LexError, Parse, Punctuation, Span, ToTokenStream, ToTokens, Token, TokenStream, TokenTree};

use crate::Template;

#[doc = "A template match directive: `@match (expr) { pat => { body }, … }`."]
#[derive(Debug, Clone)]
pub struct TmplMatch {
    pub span: Span,
    pub at_punct: Token![@],
    pub match_keyword: Token![match],
    pub expr: TokenStream,
    pub arms: Vec<TmplMatchArm>,
}

#[doc = "A single arm of a `@match` directive: `pat => { body }`."]
#[derive(Debug, Clone)]
pub struct TmplMatchArm {
    pub span: Span,
    pub pat: TokenStream,
    pub fat_arrow: Token![=>],
    pub body: Template,
    pub comma: Option<Token![,]>,
}

impl TmplMatch {
    pub fn parse_after_keyword_match(
        stream: &mut ParseStream,
        at_punct: Token![@],
        match_kw: Token![match],
    ) -> Result<Self, ParseError> {
        let span = at_punct.span();
        let expr = stream.parse_group(Delim::Paren)?;
        let arms_stream = stream.parse_group(Delim::Brace)?;
        let mut arms_ps = arms_stream.parse();
        let arms = arms_ps.parse::<Vec<TmplMatchArm>>()?;

        Ok(Self {
            span,
            at_punct,
            match_keyword: match_kw,
            expr,
            arms,
        })
    }
}

impl Parse for TmplMatchArm {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let span = stream.span();
        let mut pat = TokenStream::new();

        loop {
            match stream.curr() {
                None => return Err(LexError::new(span).message("unexpected end of match arm").into()),
                Some(TokenTree::Punct(Punctuation::FatArrow(_))) => break,
                _ => {
                    pat.extend_one(stream.advance().unwrap().clone());
                }
            }
        }

        let fat_arrow = stream.parse::<Token![=>]>()?;
        let body_stream = stream.parse_group(Delim::Brace)?;
        let mut body_ps = body_stream.parse();
        let body = body_ps.parse::<Template>()?;
        let comma = stream.parse_if::<Token![,]>();

        Ok(Self {
            span,
            pat,
            fat_arrow,
            body,
            comma,
        })
    }
}

impl ToTokens for TmplMatch {
    fn to_tokens(&self, out: &mut TokenStream) {
        <Token![match]>::new(Span::call_site()).to_tokens(out);
        self.expr.to_tokens(out);
        let mut arms = TokenStream::new();

        for arm in &self.arms {
            arm.pat.to_tokens(&mut arms);
            <Token![=>]>::new(Span::call_site()).to_tokens(&mut arms);
            arms.extend_one(TokenTree::Group(Group::new(Delim::Brace, arm.body.to_token_stream())));
            <Token![,]>::new(Span::call_site()).to_tokens(&mut arms);
        }

        out.extend_one(TokenTree::Group(Group::new(Delim::Brace, arms)));
    }
}
