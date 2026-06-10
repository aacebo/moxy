#![allow(unused)]

use std::str::FromStr;

use moxy_token::parser::{ParseError, ParseStream};
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

#[doc = "A template interpolation fused with a trailing identifier suffix: `{{ expr }}suffix`."]
#[derive(Debug, Clone)]
pub struct TmplConcat {
    pub interp: TmplInterp,
    pub suffix: String,
}

impl ToTokens for TmplConcat {
    fn to_tokens(&self, out: &mut TokenStream) {
        let mut body =
            TokenStream::from_str("let mut __moxy_seg = ::moxy_token::TokenStream::new(); ::moxy_token::ToTokens::to_tokens")
                .unwrap();

        let mut seg_args = TokenStream::from_str("&").unwrap();
        seg_args.extend_one(TokenTree::Group(Group::new(Delim::Paren, self.interp.expr.clone())));
        seg_args.extend(TokenStream::from_str(", &mut __moxy_seg").unwrap());
        body.extend_one(TokenTree::Group(Group::new(Delim::Paren, seg_args)));

        body.extend(
            TokenStream::from_str(&format!(
                "; let __moxy_sp = __moxy_seg.last(); ::moxy_token::ToTokens::to_tokens(&::moxy_token::Ident::lex(::std::format!(\"{{}}{{}}\", __moxy_seg, {:?})).expect(\"template concat: not a valid identifier\").with_span(__moxy_sp), &mut __moxy_tmpl);",
                self.suffix
            ))
            .unwrap(),
        );

        out.extend_one(TokenTree::Group(Group::new(Delim::Brace, body)));
    }
}
