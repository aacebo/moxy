use moxy_token::{Span, ToTokens, TokenStream};

use crate::template;

#[doc = "Literal passthrough tokens in a template: any tokens not matched by interpolation or control flow."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TmplTokens {
    pub span: Span,
    pub stream: TokenStream,
}

impl ToTokens for TmplTokens {
    fn to_tokens(&self, out: &mut TokenStream) {
        let src = self.stream.to_string();
        template::push_value(out, TokenStream::from(vec![template::string_lit(&src)]));
    }
}
