use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Delim, Group, Parse, Span, ToTokens, TokenStream, TokenTree};

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
    fn to_tokens(&self, t: &mut TokenStream) {
        let mut inner = TokenStream::new();
        self.expr.to_tokens(&mut inner);
        let inner_group = TokenTree::Group(Group::new(Delim::Brace, inner));
        let mut outer = TokenStream::new();
        outer.extend_one(inner_group);
        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, outer)));
    }
}
