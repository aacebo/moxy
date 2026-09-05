#![allow(unused)]

use moxy_ast::{Parse, ParseError, Parser, Token};
use moxy_token::{Delim, Group, Ident, Span, ToTokenStream, ToTokens, TokenStream, TokenTree};

use crate::Template;

#[doc = "A template for-loop directive: `@for (binding in iter) { body }`."]
#[derive(Debug, Clone)]
pub struct TmplFor {
    pub span: Span,
    pub at_punct: Token![@],
    pub for_keyword: Token![for],
    pub binding: Ident,
    pub in_keyword: Token![in],
    pub iter: TokenStream,
    pub body: Box<Template>,
}

impl TmplFor {
    pub fn parse_after_keyword_for(parser: &Parser, at_punct: Token![@], for_kw: Token![for]) -> Result<Self, ParseError> {
        let span = at_punct.span();
        let paren_inner = parser.parse_group(Delim::Paren)?;
        let ps = Parser::from_tokens(&paren_inner);
        let binding = ps.parse::<Ident>()?;
        let in_keyword = ps.parse::<Token![in]>()?;
        let mut iter = TokenStream::new();

        while let Some(tt) = ps.advance() {
            iter.extend_one(tt.clone());
        }

        let body_stream = parser.parse_group(Delim::Brace)?;
        let body_ps = Parser::from_tokens(&body_stream);
        let body = body_ps.parse::<Template>()?;

        Ok(Self {
            span,
            at_punct,
            for_keyword: for_kw,
            binding,
            in_keyword,
            iter,
            body: Box::new(body),
        })
    }
}

impl ToTokens for TmplFor {
    fn to_tokens(&self, out: &mut TokenStream) {
        <Token![for]>::new(Span::call_site()).to_tokens(out);
        self.binding.to_tokens(out);
        <Token![in]>::new(Span::call_site()).to_tokens(out);
        self.iter.to_tokens(out);
        out.extend_one(TokenTree::Group(Group::new(Delim::Brace, self.body.to_token_stream())));
    }
}
