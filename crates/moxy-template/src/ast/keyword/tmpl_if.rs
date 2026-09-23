#![allow(unused)]

use moxy_ast::{Cursor, Parse, ParseError, Parser, Token};
use moxy_token::{Delim, Group, Span, ToTokenStream, ToTokens, TokenStream, TokenTree};

use crate::Template;

#[doc = "A template if/else-if/else directive: `@if (cond) { body } @else if (cond) { body } @else { body }`."]
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug))]
pub struct TmplIf {
    pub span: Span,
    pub at_punct: Token![@],
    pub if_keyword: Token![if],
    pub branches: Vec<TmplIfBranch>,
    pub else_at_punct: Option<Token![@]>,
    pub else_keyword: Option<Token![else]>,
    pub else_body: Option<Box<Template>>,
}

#[doc = "A single branch of a `@if` or `@else if` directive."]
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug))]
pub struct TmplIfBranch {
    pub span: Span,
    pub at_punct: Option<Token![@]>,
    pub else_keyword: Option<Token![else]>,
    pub if_keyword: Token![if],
    pub cond: TokenStream,
    pub body: Template,
}

impl Parse for TmplIfBranch {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Token![if]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let if_keyword: Token![if] = parser.parse()?;
        let span = if_keyword.span();
        let cond = parser.parse_group(Delim::Paren)?.to_token_stream();
        let body = parser.parse_group(Delim::Brace)?.parse()?;

        Ok(Self {
            span,
            at_punct: None,
            else_keyword: None,
            if_keyword,
            cond,
            body,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let cursor = cursor.skip::<Token![if]>()?;
        let inner = cursor.descend(Delim::Paren)?;
        let cursor = inner.offset(inner.remaining()).is_empty().then(|| cursor.offset(1))?;
        let inner = cursor.descend(Delim::Brace)?;
        let inner = Template::skip(inner)?;
        inner.is_empty().then(|| cursor.offset(1))
    }
}

impl Parse for TmplIf {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(cursor) = cursor.skip::<Token![@]>() else {
            return false;
        };

        cursor.peek::<Token![if]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let at_punct: Token![@] = parser.parse()?;
        let span = at_punct.span();
        let first: TmplIfBranch = parser.parse()?;
        let mut branches = vec![first];
        let mut else_at_punct = None;
        let mut else_keyword = None;
        let mut else_body = None;

        while let Some(cursor) = parser.cursor().skip::<Token![@]>()
            && cursor.peek::<Token![else]>()
        {
            let at_punct = parser.parse()?;
            let keyword = parser.parse()?;

            if parser.peek::<Token![if]>() {
                let mut branch: TmplIfBranch = parser.parse()?;
                branch.at_punct = Some(at_punct);
                branch.else_keyword = Some(keyword);
                branches.push(branch);
            } else {
                else_at_punct = Some(at_punct);
                else_keyword = Some(keyword);
                else_body = Some(Box::new(parser.parse_group(Delim::Brace)?.parse()?));
                break;
            }
        }

        Ok(Self {
            span,
            at_punct,
            if_keyword: branches[0].if_keyword,
            branches,
            else_at_punct,
            else_keyword,
            else_body,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = cursor.skip::<Token![@]>()?;
        cursor = cursor.skip::<TmplIfBranch>()?;

        while let Some(next) = cursor.skip::<Token![@]>()
            && next.peek::<Token![else]>()
        {
            cursor = next.skip::<Token![else]>()?;

            if cursor.peek::<Token![if]>() {
                cursor = cursor.skip::<TmplIfBranch>()?;
            } else {
                let inner = cursor.descend(Delim::Brace)?;
                let inner = Template::skip(inner)?;
                return inner.is_empty().then(|| cursor.offset(1));
            }
        }

        Some(cursor)
    }
}

impl ToTokens for TmplIf {
    fn to_tokens(&self, out: &mut TokenStream) {
        for (i, branch) in self.branches.iter().enumerate() {
            if i > 0 {
                branch.else_keyword.to_tokens(out);
            }

            branch.if_keyword.to_tokens(out);
            branch.cond.to_tokens(out);
            out.extend_one(TokenTree::Group(Group::new(Delim::Brace, branch.body.to_token_stream())));
        }

        if let Some(else_b) = &self.else_body {
            self.else_keyword.to_tokens(out);
            out.extend_one(TokenTree::Group(Group::new(Delim::Brace, else_b.to_token_stream())));
        }
    }
}
