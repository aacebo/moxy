#![allow(unused)]

use std::str::FromStr;

use moxy_ast::{Cursor, Parse, ParseError, Parser};
use moxy_token::{Delim, Group, Span, ToTokenStream, ToTokens, TokenStream, TokenTree};

#[doc = "A template interpolation: `{{ expr }}`."]
#[derive(Debug, Clone)]
pub struct TmplInterp {
    pub span: Span,
    pub expr: TokenStream,
    pub wrap: usize,
}

impl Parse for TmplInterp {
    fn peek(cursor: Cursor<'_>) -> bool {
        matches!(cursor.curr(), Some(TokenTree::Group(group)) if super::Node::is_interp_group(group))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let span = parser.span();
        let mut inner = parser.parse_group(Delim::Brace)?;
        let mut layers: usize = 1;

        while super::lone_brace_child(&inner.to_token_stream()).is_some() {
            inner = inner.parse_group(Delim::Brace)?;
            layers += 1;
        }

        Ok(Self {
            span,
            expr: inner.to_token_stream(),
            wrap: layers.saturating_sub(2),
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Self::peek(cursor).then(|| cursor.offset(1))
    }
}

impl ToTokens for TmplInterp {
    fn to_tokens(&self, out: &mut TokenStream) {
        // `::moxy::token::ToTokens::to_tokens(&(<expr>), &mut __moxy_tmpl);`
        // The expr is spliced by value so its original spans survive.
        let mut args = TokenStream::from_str("&").unwrap();
        args.extend_one(TokenTree::Group(Group::new(Delim::Paren, self.expr.clone())));
        args.extend(TokenStream::from_str(", &mut __moxy_tmpl").unwrap());

        out.extend(TokenStream::from_str("::moxy::token::ToTokens::to_tokens").unwrap());
        out.extend_one(TokenTree::Group(Group::new(Delim::Paren, args)));
        out.extend(TokenStream::from_str(";").unwrap());
    }
}
