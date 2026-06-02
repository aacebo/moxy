use crate::ast::{Abi, BareFnArg, BoundLifetimes, Punctuated, ReturnType, Unsafety, Variadic};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{Extern, Fn};
use crate::token::punct::Comma;
use crate::token::{Delim, Group, ToTokens};
use crate::{Parse, Span, TokenStream, TokenTree};

#[doc = "A bare function pointer type (e.g. `fn(u8) -> u8`, `extern \"C\" fn()`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypeBareFn {
    pub span: Span,
    pub lifetimes: Option<BoundLifetimes>,
    pub unsafety: Unsafety,
    pub abi: Option<Abi>,
    pub inputs: Punctuated<BareFnArg, Comma>,
    pub variadic: Option<Variadic>,
    pub output: ReturnType,
}

impl Parse for TypeBareFn {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let lifetimes = stream.parse_opt::<BoundLifetimes>();
        let unsafety = stream.parse::<Unsafety>()?;
        let abi = if stream.peek::<Extern>().is_some() {
            Some(stream.parse::<Abi>()?)
        } else {
            None
        };

        let _ = stream.parse::<Fn>()?;
        let group = stream.parse_group(Delim::Paren)?;
        let mut inner = group.parse();
        let inputs = Punctuated::parse_terminated(&mut inner)?;
        let output = stream.parse::<ReturnType>()?;
        Ok(Self {
            span: Span::default(),
            lifetimes,
            unsafety,
            abi,
            inputs,
            variadic: None,
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

        Fn::default().to_tokens(t);
        let mut inner = TokenStream::new();
        self.inputs.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
        self.output.to_tokens(t);
    }
}
