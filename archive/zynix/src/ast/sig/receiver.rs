use crate::ast::{Attribute, Lifetime, Mutability};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::SelfValue;
use crate::token::punct::And;
use crate::{Parse, Span, TokenStream};

#[doc = "A method receiver parameter (`self`, `&self`, `&mut self`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Receiver {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub reference: bool,
    pub lifetime: Option<Lifetime>,
    pub mutability: Mutability,
}

impl Parse for Receiver {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let reference = if stream.peek::<And>().is_some() {
            let _ = stream.parse::<And>()?;
            true
        } else {
            false
        };

        let lifetime = if reference { stream.parse_opt::<Lifetime>() } else { None };
        let mutability = stream.parse::<Mutability>()?;
        let _ = stream.parse::<SelfValue>()?;
        Ok(Self {
            span: Span::default(),
            attrs,
            reference,
            lifetime,
            mutability,
        })
    }
}

impl ToTokens for Receiver {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        if self.reference {
            And::default().to_tokens(t);

            if let Some(l) = &self.lifetime {
                l.to_tokens(t);
            }
        }

        self.mutability.to_tokens(t);
        SelfValue::default().to_tokens(t);
    }
}
