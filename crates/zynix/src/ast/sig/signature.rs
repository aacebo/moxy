use super::{Abi, FnParam, Variadic};
use crate::ast::{Asyncness, Constness, Generics, Ident, Punctuated, ReturnType, Unsafety};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{Extern, Fn};
use crate::token::punct::{Comma, Gt, Lt};
use crate::token::{Delim, ToTokens, TokenTree};
use crate::{Parse, Span, TokenStream};

#[doc = "A function signature."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Signature {
    pub span: Span,
    pub constness: Constness,
    pub asyncness: Asyncness,
    pub unsafety: Unsafety,
    pub abi: Option<Abi>,
    pub ident: Ident,
    pub generics: Generics,
    pub inputs: Punctuated<FnParam, Comma>,
    pub variadic: Option<Variadic>,
    pub output: ReturnType,
}

impl Parse for Signature {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let constness = stream.parse::<Constness>()?;
        let asyncness = stream.parse::<Asyncness>()?;
        let unsafety = stream.parse::<Unsafety>()?;
        let abi = if stream.peek::<Extern>().is_some() {
            Some(stream.parse::<Abi>()?)
        } else {
            None
        };

        let _ = stream.parse::<Fn>()?;
        let ident = stream.parse::<Ident>()?;
        let mut generics = stream.parse::<Generics>()?;

        let group = stream.parse_group(Delim::Paren)?;
        let mut inner = group.parse();
        let mut inputs = Punctuated::new();
        let mut variadic = None;
        while !inner.is_empty() {
            if let Some(v) = inner.parse_opt::<Variadic>() {
                variadic = Some(v);
                break;
            }
            inputs.push_value(inner.parse::<FnParam>()?);
            if inner.peek::<Comma>().is_some() {
                inputs.push_punct(inner.parse::<Comma>()?);
            } else {
                break;
            }
        }

        let output = stream.parse::<ReturnType>()?;

        if stream.peek::<crate::token::keyword::Where>().is_some() {
            generics.where_clause = Some(stream.parse()?);
        }

        Ok(Self {
            span: Span::default(),
            constness,
            asyncness,
            unsafety,
            abi,
            ident,
            generics,
            inputs,
            variadic,
            output,
        })
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

    pub fn is_start(stream: &mut crate::parse::ParseStream) -> bool {
        let mut fork = stream.fork();
        let _ = fork.parse::<crate::ast::Constness>();
        let _ = fork.parse::<crate::ast::Asyncness>();
        let _ = fork.parse::<crate::ast::Unsafety>();

        if fork.peek::<Extern>().is_some() {
            let _ = fork.parse::<crate::ast::sig::Abi>();
        }

        fork.peek::<Fn>().is_some()
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
        Fn::default().to_tokens(t);
        self.ident.to_tokens(t);
        let mut params = TokenStream::new();
        Signature::emit_angle_params(&self.generics, &mut params);
        t.extend(params);
        let mut inner = TokenStream::new();
        self.inputs.to_tokens(&mut inner);

        if let Some(v) = &self.variadic {
            if !self.inputs.is_empty() && !self.inputs.trailing_punct() {
                Comma::default().to_tokens(&mut inner);
            }
            v.to_tokens(&mut inner);
        }

        t.extend_one(TokenTree::Group(crate::token::Group::new(Delim::Paren, inner)));
        self.output.to_tokens(t);

        if let Some(w) = &self.generics.where_clause {
            w.to_tokens(t);
        }
    }
}
