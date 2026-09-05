#![allow(unused)]

use std::str::FromStr;

use moxy_token::{Span, ToTokens, TokenStream};

#[doc = "Literal passthrough tokens in a template: any tokens not matched by interpolation or control flow."]
#[derive(Debug, Clone)]
pub struct TmplTokens {
    pub span: Span,
    pub parser: TokenStream,
}

impl ToTokens for TmplTokens {
    fn to_tokens(&self, out: &mut TokenStream) {
        let src = self.parser.to_string();
        out.extend(TokenStream::from_str(&format!("::moxy::token::ToTokens::to_tokens(&{src:?}, &mut __moxy_tmpl);")).unwrap());
    }
}
