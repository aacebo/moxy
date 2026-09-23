#![allow(unused)]

use moxy_ast::{Cursor, Parse, ParseError, Parser, Token};
use moxy_token::{Delim, Group, Ident, Span, ToTokenStream, ToTokens, TokenStream, TokenTree};

use crate::Template;

#[doc = "A template for-loop directive: `@for (binding in iter) { body }`."]
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug))]
pub struct TmplFor {
    pub span: Span,
    pub at_punct: Token![@],
    pub for_keyword: Token![for],
    pub binding: Ident,
    pub in_keyword: Token![in],
    pub iter: TokenStream,
    pub body: Box<Template>,
}

impl Parse for TmplFor {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(cursor) = cursor.skip::<Token![@]>() else {
            return false;
        };

        cursor.peek::<Token![for]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let at_punct: Token![@] = parser.parse()?;
        let for_keyword = parser.parse()?;
        let span = at_punct.span();
        let clause = parser.parse_group(Delim::Paren)?;
        let binding = clause.parse()?;
        let in_keyword = clause.parse()?;
        let iter = clause.to_token_stream();
        let body = parser.parse_group(Delim::Brace)?.parse()?;

        Ok(Self {
            span,
            at_punct,
            for_keyword,
            binding,
            in_keyword,
            iter,
            body: Box::new(body),
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let cursor = cursor.skip::<Token![@]>()?;
        let cursor = cursor.skip::<Token![for]>()?;
        let inner = cursor.descend(Delim::Paren)?;
        let inner = inner.skip::<Ident>()?;
        let inner = inner.skip::<Token![in]>()?;
        let cursor = inner.offset(inner.remaining()).is_empty().then(|| cursor.offset(1))?;
        let inner = cursor.descend(Delim::Brace)?;
        let inner = Template::skip(inner)?;
        inner.is_empty().then(|| cursor.offset(1))
    }
}

impl ToTokens for TmplFor {
    fn to_tokens(&self, out: &mut TokenStream) {
        self.for_keyword.to_tokens(out);
        self.binding.to_tokens(out);
        self.in_keyword.to_tokens(out);
        self.iter.to_tokens(out);
        out.extend_one(TokenTree::Group(Group::new(Delim::Brace, self.body.to_token_stream())));
    }
}
