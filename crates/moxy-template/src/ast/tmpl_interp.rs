#![allow(unused)]

use std::str::FromStr;

use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Delim, Group, Parse, Span, ToTokens, TokenStream, TokenTree};

#[doc = "A template interpolation: `{{ expr }}`."]
#[derive(Debug, Clone)]
pub struct TmplInterp {
    pub span: Span,
    pub expr: TokenStream,
}

impl Parse for TmplInterp {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let span = stream.span();
        let outer = stream.parse_group(Delim::Brace)?;
        let mut outer_ps = outer.parse();
        let expr = outer_ps.parse_group(Delim::Brace)?;
        Ok(Self { span, expr })
    }
}

impl ToTokens for TmplInterp {
    fn to_tokens(&self, out: &mut TokenStream) {
        // `::moxy_token::ToTokens::to_tokens(&(<expr>), &mut __moxy_tmpl);`
        // The expr is spliced by value so its original spans survive.
        let mut args = TokenStream::from_str("&").unwrap();
        args.extend_one(TokenTree::Group(Group::new(Delim::Paren, self.expr.clone())));
        args.extend(TokenStream::from_str(", &mut __moxy_tmpl").unwrap());

        out.extend(TokenStream::from_str("::moxy_token::ToTokens::to_tokens").unwrap());
        out.extend_one(TokenTree::Group(Group::new(Delim::Paren, args)));
        out.extend(TokenStream::from_str(";").unwrap());
    }
}
