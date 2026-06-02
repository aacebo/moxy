use moxy_token::keyword::Match;
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::{At, Comma, FatArrow};
use moxy_token::{Delim, Group, LexError, Parse, Punctuation, Span, ToTokens, Token, TokenStream, TokenTree};

use crate::template::Template;

#[doc = "A template match directive: `@match (expr) { pat => { body }, … }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TmplMatch {
    pub span: Span,
    pub at_punct: At,
    pub match_keyword: Match,
    pub expr: TokenStream,
    pub arms: Vec<TmplMatchArm>,
}

#[doc = "A single arm of a `@match` directive: `pat => { body }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TmplMatchArm {
    pub span: Span,
    pub pat: TokenStream,
    pub fat_arrow: FatArrow,
    pub body: Template,
    pub comma: Option<Comma>,
}

impl TmplMatch {
    pub fn parse_after_keyword_match(stream: &mut ParseStream, at_punct: At, match_kw: Match) -> Result<Self, ParseError> {
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
                Some(TokenTree::Token(Token::Punct(Punctuation::FatArrow(_)))) => break,
                _ => {
                    pat.extend_one(stream.advance().unwrap().clone());
                }
            }
        }

        let fat_arrow = stream.parse::<FatArrow>()?;
        let body_stream = stream.parse_group(Delim::Brace)?;
        let mut body_ps = body_stream.parse();
        let body = Template::parse(&mut body_ps)?;
        let comma = stream.parse_if::<Comma>();

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
    fn to_tokens(&self, t: &mut TokenStream) {
        self.match_keyword.to_tokens(t);
        self.expr.to_tokens(t);

        let mut arms = TokenStream::new();
        for arm in &self.arms {
            arm.to_tokens(&mut arms);
        }
        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, arms)));
    }
}

impl ToTokens for TmplMatchArm {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.pat.to_tokens(t);
        self.fat_arrow.to_tokens(t);
        let mut body = TokenStream::new();
        self.body.to_tokens(&mut body);
        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, body)));
        self.comma.to_tokens(t);
    }
}
