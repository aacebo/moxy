use moxy_token::keyword::{Extern, Fn};
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Comma;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::{Abi, BareFnArg, BoundLifetimes, Delimited, Punctuated, ReturnType, Unsafety, Variadic};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BareFnParams {
    pub inputs: Punctuated<BareFnArg, Comma>,
    pub variadic: Option<Variadic>,
}

impl ToTokens for BareFnParams {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.inputs.to_tokens(t);
        if let Some(v) = &self.variadic {
            if !self.inputs.is_empty() && !self.inputs.trailing_punct() {
                Comma::default().to_tokens(t);
            }
            v.to_tokens(t);
        }
    }
}

#[doc = "A bare function pointer type (e.g. `fn(u8) -> u8`, `extern \"C\" fn()`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeBareFn {
    pub span: Span,
    pub lifetimes: Option<BoundLifetimes>,
    pub unsafety: Unsafety,
    pub abi: Option<Abi>,
    pub fn_keyword: Fn,
    pub params: Delimited<BareFnParams>,
    pub output: ReturnType,
}

impl Parse for TypeBareFn {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let lifetimes = stream.parse_if::<BoundLifetimes>();
        let unsafety = stream.parse::<Unsafety>()?;
        let abi = if stream.peek::<Extern>().is_some() {
            Some(stream.parse::<Abi>()?)
        } else {
            None
        };

        let fn_keyword = stream.parse::<Fn>()?;
        let params = Delimited::parse_paren_with(stream, |inner| {
            let inputs = Punctuated::parse_terminated(inner)?;
            Ok(BareFnParams { inputs, variadic: None })
        })?;
        let output = stream.parse::<ReturnType>()?;
        Ok(Self {
            span: Span::default(),
            lifetimes,
            unsafety,
            abi,
            fn_keyword,
            params,
            output,
        })
    }
}

impl ToTokens for TypeBareFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        if let Some(l) = &self.lifetimes {
            l.to_tokens(t);
        }
        self.unsafety.to_tokens(t);
        if let Some(abi) = &self.abi {
            abi.to_tokens(t);
        }
        self.fn_keyword.to_tokens(t);
        self.params.to_tokens(t);
        self.output.to_tokens(t);
    }
}
