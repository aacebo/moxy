use moxy_token::keyword::{Else, If};
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::At;
use moxy_token::{Delim, Parse, Span, ToTokens, TokenStream};

use crate::template::{self, Template};

#[doc = "A template if/else-if/else directive: `@if (cond) { body } @else if (cond) { body } @else { body }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TmplIf {
    pub span: Span,
    pub at_punct: At,
    pub if_keyword: If,
    pub branches: Vec<TmplIfBranch>,
    pub else_at_punct: Option<At>,
    pub else_keyword: Option<Else>,
    pub else_body: Option<Box<Template>>,
}

#[doc = "A single branch of a `@if` or `@else if` directive."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TmplIfBranch {
    pub span: Span,
    pub at_punct: Option<At>,
    pub else_keyword: Option<Else>,
    pub if_keyword: If,
    pub cond: TokenStream,
    pub body: Template,
}

impl TmplIf {
    pub fn parse_after_keyword_if(stream: &mut ParseStream, at_punct: At, if_kw: If) -> Result<Self, ParseError> {
        let span = at_punct.span();
        let first = Self::parse_branch(stream, None, None, if_kw)?;
        let mut branches = vec![first];
        let mut else_at_punct = None;
        let mut else_keyword_field = None;
        let mut else_body = None;

        loop {
            let mut fork = stream.fork();
            let Ok(at2) = fork.parse::<At>() else { break };
            let Ok(else_kw) = fork.parse::<Else>() else { break };

            stream.seek(&fork);

            if let Some(if_kw2) = stream.parse_if::<If>() {
                branches.push(Self::parse_branch(stream, Some(at2), Some(else_kw), if_kw2)?);
            } else {
                let body_stream = stream.parse_group(Delim::Brace)?;
                let mut body_ps = body_stream.parse();
                else_at_punct = Some(at2);
                else_keyword_field = Some(else_kw);
                else_body = Some(Box::new(Template::parse(&mut body_ps)?));
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
        stream: &mut ParseStream,
        at_punct: Option<At>,
        else_keyword: Option<Else>,
        if_keyword: If,
    ) -> Result<TmplIfBranch, ParseError> {
        let span = if_keyword.span();
        let cond = stream.parse_group(Delim::Paren)?;
        let body_stream = stream.parse_group(Delim::Brace)?;
        let mut body_ps = body_stream.parse();
        let body = Template::parse(&mut body_ps)?;

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
                out.extend(template::rust("else"));
            }
            out.extend(template::rust("if"));
            branch.cond.to_tokens(out);
            out.extend_one(template::brace_body(&branch.body.nodes));
        }

        if let Some(else_b) = &self.else_body {
            out.extend(template::rust("else"));
            out.extend_one(template::brace_body(&else_b.nodes));
        }
    }
}
