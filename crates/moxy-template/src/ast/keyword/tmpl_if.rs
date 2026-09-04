#![allow(unused)]

use moxy_ast::{Parse, ParseError, Parser, Token};
use moxy_token::{Delim, Group, Span, ToTokenStream, ToTokens, TokenStream, TokenTree};

use crate::Template;

#[doc = "A template if/else-if/else directive: `@if (cond) { body } @else if (cond) { body } @else { body }`."]
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct TmplIfBranch {
    pub span: Span,
    pub at_punct: Option<Token![@]>,
    pub else_keyword: Option<Token![else]>,
    pub if_keyword: Token![if],
    pub cond: TokenStream,
    pub body: Template,
}

impl TmplIf {
    pub fn parse_after_keyword_if(parser: &Parser, at_punct: Token![@], if_kw: Token![if]) -> Result<Self, ParseError> {
        let span = at_punct.span();
        let first = Self::parse_branch(parser, None, None, if_kw)?;
        let mut branches = vec![first];
        let mut else_at_punct = None;
        let mut else_keyword_field = None;
        let mut else_body = None;

        loop {
            let mut fork = parser.lookahead();

            if !fork.peek::<Token![@]>() {
                break;
            }

            fork.advance();

            if !fork.peek::<Token![else]>() {
                break;
            }

            let at2 = parser.parse::<Token![@]>()?;
            let else_kw = parser.parse::<Token![else]>()?;

            if let Some(if_kw2) = parser.parse_if::<Token![if]>() {
                branches.push(Self::parse_branch(parser, Some(at2), Some(else_kw), if_kw2)?);
            } else {
                let body_stream = parser.parse_group(Delim::Brace)?;
                let body_ps = Parser::from_tokens(&body_stream);
                else_at_punct = Some(at2);
                else_keyword_field = Some(else_kw);
                else_body = Some(Box::new(body_ps.parse::<Template>()?));
                break;
            }
        }

        Ok(Self {
            span,
            at_punct,
            if_keyword: branches[0].if_keyword,
            branches,
            else_at_punct,
            else_keyword: else_keyword_field,
            else_body,
        })
    }

    pub fn parse_branch(
        parser: &Parser,
        at_punct: Option<Token![@]>,
        else_keyword: Option<Token![else]>,
        if_keyword: Token![if],
    ) -> Result<TmplIfBranch, ParseError> {
        let span = if_keyword.span();
        let cond = parser.parse_group(Delim::Paren)?;
        let body_stream = parser.parse_group(Delim::Brace)?;
        let body_ps = Parser::from_tokens(&body_stream);
        let body = body_ps.parse::<Template>()?;

        Ok(TmplIfBranch {
            span,
            at_punct,
            else_keyword,
            if_keyword,
            cond,
            body,
        })
    }
}

impl ToTokens for TmplIf {
    fn to_tokens(&self, out: &mut TokenStream) {
        for (i, branch) in self.branches.iter().enumerate() {
            if i > 0 {
                <Token![else]>::new(Span::call_site()).to_tokens(out);
            }

            <Token![if]>::new(Span::call_site()).to_tokens(out);
            branch.cond.to_tokens(out);
            out.extend_one(TokenTree::Group(Group::new(Delim::Brace, branch.body.to_token_stream())));
        }

        if let Some(else_b) = &self.else_body {
            <Token![else]>::new(Span::call_site()).to_tokens(out);
            out.extend_one(TokenTree::Group(Group::new(Delim::Brace, else_b.to_token_stream())));
        }
    }
}
