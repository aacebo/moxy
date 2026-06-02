use moxy_token::{Span, ToTokens, TokenStream};

#[doc = "Literal passthrough tokens in a template: any tokens not matched by interpolation or control flow."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TmplTokens {
    pub span: Span,
    pub stream: TokenStream,
}

impl ToTokens for TmplTokens {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.stream.to_tokens(t);
    }
}
