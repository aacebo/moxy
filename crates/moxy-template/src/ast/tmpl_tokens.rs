#![allow(unused)]

use std::str::FromStr;

use moxy_ast::{Cursor, Parse, ParseError, Parser};
use moxy_token::{Punct, Span, ToTokens, TokenStream, TokenTree};

#[doc = "Literal passthrough tokens in a template: any tokens not matched by interpolation or control flow."]
#[derive(Debug, Clone)]
pub struct TmplTokens {
    pub span: Span,
    pub tokens: TokenStream,
}

impl Parse for TmplTokens {
    fn peek(cursor: Cursor<'_>) -> bool {
        match cursor.curr() {
            Some(TokenTree::Punct(Punct::At(_))) | None => false,
            Some(TokenTree::Group(group)) => !super::Node::is_interp_group(group) && !super::Node::group_has_interp(group),
            Some(_) => true,
        }
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let span = parser.span();
        let mut tokens = TokenStream::new();

        while let Some(token) = parser.curr() {
            if matches!(token, TokenTree::Punct(Punct::At(_))) {
                break;
            }

            if let TokenTree::Group(group) = token
                && (super::Node::is_interp_group(group) || super::Node::group_has_interp(group))
            {
                break;
            }

            tokens.extend_one(token.clone());
            parser.advance();
        }

        if tokens.is_empty() {
            return parser.error("expected template tokens").into();
        }

        Ok(Self { span, tokens })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut skipped = false;

        while let Some(token) = cursor.curr() {
            if matches!(token, TokenTree::Punct(Punct::At(_))) {
                break;
            }

            if let TokenTree::Group(group) = token
                && (super::Node::is_interp_group(group) || super::Node::group_has_interp(group))
            {
                break;
            }

            cursor = cursor.offset(1);
            skipped = true;
        }

        skipped.then_some(cursor)
    }
}

impl ToTokens for TmplTokens {
    fn to_tokens(&self, out: &mut TokenStream) {
        let src = self.tokens.to_string();
        out.extend(TokenStream::from_str(&format!("::moxy::token::ToTokens::to_tokens(&{src:?}, &mut __moxy_tmpl);")).unwrap());
    }
}
