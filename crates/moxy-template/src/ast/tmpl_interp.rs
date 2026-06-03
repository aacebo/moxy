use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Delim, Parse, Span, ToTokens, TokenStream};

use crate::template;

#[doc = "A template interpolation: `{{ expr }}`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
        template::push_value(out, self.expr.clone());
    }
}
