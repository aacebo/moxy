#![allow(unused)]

use moxy_ast::{Cursor, Parse, ParseError, Parser, Pattern, Token};
use moxy_token::{Delim, Group, Span, ToTokenStream, ToTokens, TokenStream, TokenTree};

use crate::Template;

use super::{parse_unparenthesized_expr, skip_unparenthesized_expr};

#[doc = "A template for-loop directive: `@for binding in iter { body }`. Parentheses around the clause are optional."]
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug))]
pub struct TmplFor {
    pub span: Span,
    pub at_punct: Token![@],
    pub for_keyword: Token![for],
    pub binding: Pattern,
    pub in_keyword: Token![in],
    pub iter: TokenStream,
    pub body: Box<Template>,
}

impl Parse for TmplFor {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(cursor) = <Token![@]>::skip(cursor) else {
            return false;
        };

        <Token![for]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let at_punct: Token![@] = <_ as Parse>::parse(parser)?;
        let for_keyword = <_ as Parse>::parse(parser)?;
        let span = at_punct.span();
        let (binding, in_keyword, iter) = if is_parenthesized_clause(parser.cursor()) {
            let clause = parser.parse_group(Delim::Paren)?;
            let binding = <_ as Parse>::parse(&clause)?;
            let in_keyword = <_ as Parse>::parse(&clause)?;
            (binding, in_keyword, clause.to_token_stream())
        } else {
            let binding = <_ as Parse>::parse(parser)?;
            let in_keyword = <_ as Parse>::parse(parser)?;
            let iter = parse_unparenthesized_expr(parser)?;
            (binding, in_keyword, iter)
        };

        let body = Template::parse(&parser.parse_group(Delim::Brace)?)?;

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
        let cursor = <Token![@]>::skip(cursor)?;
        let cursor = <Token![for]>::skip(cursor)?;
        let cursor = if is_parenthesized_clause(cursor) {
            let inner = cursor.descend(Delim::Paren)?;
            let inner = Pattern::skip(inner)?;
            let inner = <Token![in]>::skip(inner)?;
            inner.offset(inner.remaining()).is_empty().then(|| cursor.offset(1))?
        } else {
            let cursor = Pattern::skip(cursor)?;
            let cursor = <Token![in]>::skip(cursor)?;
            skip_unparenthesized_expr(cursor)?
        };

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

fn is_parenthesized_clause(cursor: Cursor<'_>) -> bool {
    if !cursor.is_delimited(Delim::Paren) {
        return false;
    }

    let Some(inner) = cursor.descend(Delim::Paren) else {
        return false;
    };
    let Some(inner) = Pattern::skip(inner) else {
        return false;
    };

    <Token![in]>::peek(inner)
}
