use moxy_token::keyword::SelfValue;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::And;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Lifetime, Mutability};

/// A method receiver parameter (`self`, `&self`, `&mut self`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Receiver {
    pub attrs: Attributes,
    pub reference: Option<And>,
    pub lifetime: Option<Lifetime>,
    pub mutability: Mutability,
    pub self_keyword: SelfValue,
}

impl Parse for Receiver {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let reference = stream.parse_if::<And>();

        let lifetime = if reference.is_some() {
            stream.parse_if::<Lifetime>()
        } else {
            None
        };
        let mutability = stream.parse::<Mutability>()?;
        let self_keyword = stream.parse::<SelfValue>()?;
        Ok(Self {
            attrs,
            reference,
            lifetime,
            mutability,
            self_keyword,
        })
    }
}

impl Spanner for Receiver {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if let Some(r) = &self.reference {
            r.span()
        } else if !matches!(self.mutability, Mutability::Immutable) {
            self.mutability.span()
        } else {
            self.self_keyword.span()
        };
        start.join(self.self_keyword.span())
    }
}

impl ToTokens for Receiver {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);

        if let Some(amp) = &self.reference {
            amp.to_tokens(t);

            if let Some(l) = &self.lifetime {
                l.to_tokens(t);
            }
        }

        self.mutability.to_tokens(t);
        self.self_keyword.to_tokens(t);
    }
}
