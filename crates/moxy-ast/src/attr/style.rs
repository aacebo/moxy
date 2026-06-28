use moxy_token::punct::{Not, Pound};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

/// Whether an attribute is outer (`#[...]`) or inner (`#![...]`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttrStyle {
    Outer(Pound),
    Inner(Pound, Not),
}

impl AttrStyle {
    pub fn is_outer(&self) -> bool {
        matches!(self, Self::Outer(_))
    }

    pub fn is_inner(&self) -> bool {
        matches!(self, Self::Inner(_, _))
    }

    pub fn pound(&self) -> &Pound {
        match self {
            Self::Outer(p) => p,
            Self::Inner(p, _) => p,
        }
    }
}

impl Spanner for AttrStyle {
    fn span(&self) -> Span {
        match self {
            Self::Outer(pound) => pound.span(),
            Self::Inner(pound, not) => pound.span().join(not.span()),
        }
    }
}

impl ToTokens for AttrStyle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Outer(pound) => pound.to_tokens(tokens),
            Self::Inner(pound, not) => {
                pound.to_tokens(tokens);
                not.to_tokens(tokens);
            }
        }
    }
}

impl Parse for AttrStyle {
    fn parse(stream: &mut moxy_token::parser::ParseStream) -> Result<Self, moxy_token::parser::ParseError> {
        let pound = stream.parse::<Pound>()?;

        if let Ok(not) = stream.parse::<Not>() {
            Ok(Self::Inner(pound, not))
        } else {
            Ok(Self::Outer(pound))
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for AttrStyle {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Outer(_) => s.serialize_str("Outer"),
            Self::Inner(..) => s.serialize_str("Inner"),
        }
    }
}
