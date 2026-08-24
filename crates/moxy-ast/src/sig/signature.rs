use moxy_token::keyword::{Extern, Fn};
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Comma, Gt, Lt};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use super::{Abi, FnParam, Variadic};
use crate::{Asyncness, Constness, Delimited, Generics, Ident, Punctuated, ReturnType, Unsafety};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FnParams {
    pub inputs: Punctuated<FnParam, Comma>,
    pub variadic: Option<Variadic>,
}

impl Spanner for FnParams {
    fn span(&self) -> Span {
        let start = self.inputs.first().map(|i| i.span()).unwrap_or_else(Span::call_site);
        let end = self
            .variadic
            .as_ref()
            .map(|v| v.span())
            .or_else(|| self.inputs.last().map(|i| i.span()))
            .unwrap_or_else(Span::call_site);
        start.join(end)
    }
}

impl ToTokens for FnParams {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.inputs.to_tokens(t);
        if let Some(v) = &self.variadic {
            if !self.inputs.is_empty() && !self.inputs.is_trailing() {
                Comma::default().to_tokens(t);
            }
            v.to_tokens(t);
        }
    }
}

/// A function signature.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Signature {
    pub constness: Constness,
    pub asyncness: Asyncness,
    pub unsafety: Unsafety,
    pub abi: Option<Abi>,
    pub fn_keyword: Fn,
    pub ident: Ident,
    pub generics: Generics,
    pub params: Delimited<FnParams>,
    pub output: ReturnType,
}

impl Parse for Signature {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let constness = stream.parse::<Constness>()?;
        let asyncness = stream.parse::<Asyncness>()?;
        let unsafety = stream.parse::<Unsafety>()?;
        let abi = if stream.peek::<Extern>() {
            Some(stream.parse::<Abi>()?)
        } else {
            None
        };

        let fn_keyword = stream.parse::<Fn>()?;
        let ident = stream.parse::<Ident>()?;
        let mut generics = stream.parse::<Generics>()?;

        let params = Delimited::parse_paren_with(stream, |inner| {
            let mut inputs = Punctuated::new();
            let mut variadic = None;

            while !inner.is_empty() {
                if let Some(v) = inner.parse_if::<Variadic>() {
                    variadic = Some(v);
                    break;
                }

                inputs.push_value(inner.parse::<FnParam>()?);

                if inner.peek::<Comma>() {
                    inputs.push_punct(inner.parse::<Comma>()?);
                } else {
                    break;
                }
            }

            Ok(FnParams { inputs, variadic })
        })?;

        let output = stream.parse::<ReturnType>()?;

        if stream.peek::<moxy_token::keyword::Where>() {
            generics.where_clause = Some(stream.parse()?);
        }

        Ok(Self {
            constness,
            asyncness,
            unsafety,
            abi,
            fn_keyword,
            ident,
            generics,
            params,
            output,
        })
    }
}

impl Spanner for Signature {
    fn span(&self) -> Span {
        let start = if !matches!(self.constness, Constness::NoConst) {
            self.constness.span()
        } else if !matches!(self.asyncness, Asyncness::Sync) {
            self.asyncness.span()
        } else if !matches!(self.unsafety, Unsafety::Safe) {
            self.unsafety.span()
        } else if let Some(abi) = &self.abi {
            abi.span()
        } else {
            self.fn_keyword.span()
        };
        let end = match &self.output {
            ReturnType::Type(_, ty) => ty.span(),
            ReturnType::Default => self.params.span(),
        };
        start.join(end)
    }
}

impl Signature {
    pub fn emit_angle_params(generics: &Generics, t: &mut TokenStream) {
        if !generics.params.is_empty() {
            Lt::default().to_tokens(t);
            generics.params.to_tokens(t);
            Gt::default().to_tokens(t);
        }
    }

    pub fn is_start(stream: &mut moxy_token::parser::ParseStream) -> bool {
        let mut fork = stream.fork();
        let _ = fork.parse::<crate::Constness>();
        let _ = fork.parse::<crate::Asyncness>();
        let _ = fork.parse::<crate::Unsafety>();

        if fork.peek::<Extern>() {
            let _ = fork.parse::<crate::sig::Abi>();
        }

        fork.peek::<Fn>()
    }
}

impl ToTokens for Signature {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.constness.to_tokens(t);
        self.asyncness.to_tokens(t);
        self.unsafety.to_tokens(t);
        if let Some(abi) = &self.abi {
            abi.to_tokens(t);
        }
        self.fn_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        let mut params = TokenStream::new();
        Self::emit_angle_params(&self.generics, &mut params);
        t.extend(params);
        self.params.to_tokens(t);
        self.output.to_tokens(t);

        if let Some(w) = &self.generics.where_clause {
            w.to_tokens(t);
        }
    }
}
