use moxy_token::punct::{Not, Pound};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

/// Whether an attribute is outer (`#[...]`) or inner (`#![...]`).
#[derive(Debug, Clone)]
pub enum AttrStyle {
    Outer(Pound),
    Inner(Pound, Not),
}

// Equality/hashing ignore token spans: two attributes of the same style match.
impl PartialEq for AttrStyle {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for AttrStyle {}

impl std::hash::Hash for AttrStyle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl Spanner for AttrStyle {
    fn span(&self) -> Span {
        match self {
            AttrStyle::Outer(pound) => pound.span(),
            AttrStyle::Inner(pound, not) => pound.span().join(not.span()),
        }
    }
}

impl ToTokens for AttrStyle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            AttrStyle::Outer(pound) => pound.to_tokens(tokens),
            AttrStyle::Inner(pound, not) => {
                pound.to_tokens(tokens);
                not.to_tokens(tokens);
            }
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
            AttrStyle::Outer(_) => s.serialize_str("Outer"),
            AttrStyle::Inner(..) => s.serialize_str("Inner"),
        }
    }
}
