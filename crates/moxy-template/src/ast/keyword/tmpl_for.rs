#![allow(unused)]

use moxy_ast::{Cursor, Parse, ParseError, Parser, Pattern, Token};
use moxy_token::{Delim, Group, Ident, Span, ToTokenStream, ToTokens, TokenStream, TokenTree};

use crate::Template;

use super::{parse_unparenthesized_expr, skip_unparenthesized_expr};

#[doc = "A template for-loop directive: `@for binding in iter { body }`, optionally followed by `@sep { tokens }`, which is emitted between iterations whose bodies emit tokens. Parentheses around the clause are optional."]
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
    pub separator: Option<Box<Template>>,
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
        let separator = if peek_separator(parser.cursor()) {
            parser.advance_by(2);
            Some(Box::new(Template::parse(&parser.parse_group(Delim::Brace)?)?))
        } else {
            None
        };

        Ok(Self {
            span,
            at_punct,
            for_keyword,
            binding,
            in_keyword,
            iter,
            body: Box::new(body),
            separator,
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
        let cursor = inner.is_empty().then(|| cursor.offset(1))?;

        if !peek_separator(cursor) {
            return Some(cursor);
        }

        let cursor = cursor.offset(2);
        let inner = cursor.descend(Delim::Brace)?;
        let inner = Template::skip(inner)?;
        inner.is_empty().then(|| cursor.offset(1))
    }
}

impl ToTokens for TmplFor {
    fn to_tokens(&self, out: &mut TokenStream) {
        use std::str::FromStr;

        let Some(separator) = &self.separator else {
            self.for_keyword.to_tokens(out);
            self.binding.to_tokens(out);
            self.in_keyword.to_tokens(out);
            self.iter.to_tokens(out);
            out.extend_one(TokenTree::Group(Group::new(Delim::Brace, self.body.to_token_stream())));
            return;
        };

        // `{ let mut emitted = false; for binding in iter { let iteration = { body };
        // if !iteration.is_empty() { if emitted { separator } emitted = true; extend } } }`.
        // `emitted` and `iteration` have mixed-site spans, so the user's binding or
        // body can neither name nor shadow them.
        let emitted = Ident::new("emitted").with_span(Span::mixed_site()).to_token_tree();
        let iteration = Ident::new("iteration").with_span(Span::mixed_site()).to_token_tree();

        let mut body = TokenStream::from_str("let mut __moxy_tmpl = ::moxy::token::TokenStream::new();").unwrap();
        self.body.to_tokens(&mut body);
        body.extend(TokenStream::from_str("__moxy_tmpl").unwrap());

        let mut emit = TokenStream::from_str("if").unwrap();
        emit.extend_one(emitted.clone());
        emit.extend_one(TokenTree::Group(Group::new(Delim::Brace, separator.to_token_stream())));
        emit.extend_one(emitted.clone());
        emit.extend(TokenStream::from_str("= true; __moxy_tmpl.extend").unwrap());
        emit.extend_one(TokenTree::Group(Group::new(
            Delim::Paren,
            TokenStream::from(vec![iteration.clone()]),
        )));
        emit.extend(TokenStream::from_str(";").unwrap());

        let mut iterate = TokenStream::from_str("let").unwrap();
        iterate.extend_one(iteration.clone());
        iterate.extend(TokenStream::from_str("=").unwrap());
        iterate.extend_one(TokenTree::Group(Group::new(Delim::Brace, body)));
        iterate.extend(TokenStream::from_str("; if !").unwrap());
        iterate.extend_one(iteration);
        iterate.extend(TokenStream::from_str(".is_empty()").unwrap());
        iterate.extend_one(TokenTree::Group(Group::new(Delim::Brace, emit)));

        let mut block = TokenStream::from_str("let mut").unwrap();
        block.extend_one(emitted);
        block.extend(TokenStream::from_str("= false;").unwrap());
        self.for_keyword.to_tokens(&mut block);
        self.binding.to_tokens(&mut block);
        self.in_keyword.to_tokens(&mut block);
        self.iter.to_tokens(&mut block);
        block.extend_one(TokenTree::Group(Group::new(Delim::Brace, iterate)));
        out.extend_one(TokenTree::Group(Group::new(Delim::Brace, block)));
    }
}

// `@sep`, the separator clause after an `@for` body
fn peek_separator(cursor: Cursor<'_>) -> bool {
    <Token![@]>::peek(cursor)
        && matches!(cursor.offset(1).curr(), Some(TokenTree::Ident(ident)) if ident == "sep")
        && cursor.offset(2).is_delimited(Delim::Brace)
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
