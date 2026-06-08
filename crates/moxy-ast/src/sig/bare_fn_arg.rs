use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Colon;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attribute, Ident, Type};

/// An argument of a bare function pointer type.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BareFnArg {
    pub attrs: Vec<Attribute>,
    pub name: Option<(Ident, Colon)>,
    pub ty: Type,
}

impl Parse for BareFnArg {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;

        let name = {
            let mut fork = stream.fork();
            if let Ok(id) = fork.parse::<Ident>() {
                if fork.peek::<Colon>() {
                    stream.seek(&fork);
                    let colon = stream.parse::<Colon>()?;
                    Some((id, colon))
                } else {
                    None
                }
            } else {
                None
            }
        };

        let ty = stream.parse::<Type>()?;
        Ok(Self { attrs, name, ty })
    }
}

impl Spanner for BareFnArg {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else if let Some((id, _)) = &self.name {
            id.span()
        } else {
            self.ty.span()
        };
        start.join(self.ty.span())
    }
}

impl ToTokens for BareFnArg {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        if let Some((n, colon)) = &self.name {
            n.to_tokens(t);
            colon.to_tokens(t);
        }

        self.ty.to_tokens(t);
    }
}
