#![allow(unused)]

use moxy_token::keyword::{For, In};
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::At;
use moxy_token::{Delim, Group, Ident, Parse, Span, ToTokenStream, ToTokens, Token, TokenStream, TokenTree};

use crate::template::Template;

#[doc = "A template for-loop directive: `@for (binding in iter) { body }`."]
#[derive(Debug, Clone)]
pub struct TmplFor {
    pub span: Span,
    pub at_punct: At,
    pub for_keyword: For,
    pub binding: Ident,
    pub in_keyword: In,
    pub iter: TokenStream,
    pub body: Box<Template>,
}

impl TmplFor {
    pub fn parse_after_keyword_for(stream: &mut ParseStream, at_punct: At, for_kw: For) -> Result<Self, ParseError> {
        let span = at_punct.span();
        let paren_inner = stream.parse_group(Delim::Paren)?;
        let mut ps = paren_inner.parse();
        let binding = ps.parse::<Ident>()?;
        let in_keyword = ps.parse::<In>()?;
        let mut iter = TokenStream::new();

        while let Some(tt) = ps.advance() {
            iter.extend_one(tt.clone());
        }

        let body_stream = stream.parse_group(Delim::Brace)?;
        let mut body_ps = body_stream.parse();
        let body = Template::parse(&mut body_ps)?;

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
