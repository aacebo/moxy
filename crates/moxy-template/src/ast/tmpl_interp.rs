#![allow(unused)]

use std::str::FromStr;

use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Delim, Group, Parse, Span, ToTokens, TokenStream, TokenTree};

#[doc = "A template interpolation: `{{ expr }}`."]
#[derive(Debug, Clone)]
pub struct TmplInterp {
    pub span: Span,
    pub expr: TokenStream,
    pub wrap: usize,
}

impl Parse for TmplInterp {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let span = stream.span();
        let mut inner = stream.parse_group(Delim::Brace)?;
        let mut layers: usize = 1;

        while super::lone_brace_child(&inner).is_some() {
            inner = inner.parse().parse_group(Delim::Brace)?;
            layers += 1;
        }

        Ok(Self {
            span,
            expr: inner,
            wrap: layers.saturating_sub(2),
        })
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

#[cfg(test)]
mod tests {
    use moxy_token::parse;

    use super::TmplInterp;

    #[test]
    fn wrap_counts_extra_brace_layers() {
        let single = parse!("{{ x }}" as TmplInterp).unwrap();
        assert_eq!(single.wrap, 0);
        assert_eq!(single.expr.to_string(), "x");

        let triple = parse!("{{{ x }}}" as TmplInterp).unwrap();
        assert_eq!(triple.wrap, 1);
        assert_eq!(triple.expr.to_string(), "x");

        let quad = parse!("{{{{ x }}}}" as TmplInterp).unwrap();
        assert_eq!(quad.wrap, 2);
        assert_eq!(quad.expr.to_string(), "x");
    }
}
