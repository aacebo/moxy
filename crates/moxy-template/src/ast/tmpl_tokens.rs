#![allow(unused)]

use std::str::FromStr;

use moxy_token::{Span, ToTokens, TokenStream, TokenTree};

#[doc = "Literal passthrough tokens in a template: any tokens not matched by interpolation or control flow."]
#[derive(Debug, Clone)]
pub struct TmplTokens {
    pub span: Span,
    pub stream: TokenStream,
}

impl TmplTokens {
    pub fn lead_ident(&self) -> Option<String> {
        match self.stream.get(0)? {
            TokenTree::Ident(v) => Some(v.text().to_string()),
            TokenTree::Literal(v) => Some(Self::unquote(v.repr()).to_string()),
            _ => None,
        }
    }

    pub fn rest(&self) -> TokenStream {
        TokenStream::from(self.stream.iter().skip(1).cloned().collect::<Vec<_>>())
    }

    fn unquote(repr: &str) -> &str {
        let bytes = repr.as_bytes();

        match (bytes.first(), bytes.last()) {
            (Some(b'"'), Some(b'"')) | (Some(b'\''), Some(b'\'')) if repr.len() >= 2 => &repr[1..repr.len() - 1],
            _ => repr,
        }
    }
}

impl ToTokens for TmplTokens {
    fn to_tokens(&self, out: &mut TokenStream) {
        let src = self.stream.to_string();
        out.extend(TokenStream::from_str(&format!("::moxy_token::ToTokens::to_tokens(&{src:?}, &mut __moxy_tmpl);")).unwrap());
    }
}
