use crate::{Parse, ParseError, Parser};
use moxy_token::{LexError, Span, Spanner, ToTokens, TokenStream, TokenTree};

use super::Type;

/// Whether a raw pointer is `*const` or `*mut`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum PointerMutability {
    Const(Token![const]),
    Mut(Token![mut]),
}

/// A raw pointer type (e.g. `*const T`, `*mut T`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypePointer {
    pub star: Token![*],
    pub mutability: PointerMutability,
    pub elem: Box<Type>,
}

impl Parse for TypePointer {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let star = parser.parse::<Token![*]>()?;
        let at = parser.span();

        // A raw pointer requires an explicit `const` or `mut` after the `*`.
        let mutability = match parser.advance() {
            Some(TokenTree::Keyword(kw)) if kw.as_str() == "mut" => PointerMutability::Mut(<Token![mut]>::new(kw.span())),
            Some(TokenTree::Keyword(kw)) if kw.as_str() == "const" => PointerMutability::Const(<Token![const]>::new(kw.span())),
            _ => {
                return Err(LexError::new(at).message("expected `const` or `mut` after `*`").into());
            }
        };

        Ok(Self {
            star,
            mutability,
            elem: Box::new(parser.parse()?),
        })
    }
}

impl Spanner for PointerMutability {
    fn span(&self) -> Span {
        match self {
            Self::Const(k) => k.span(),
            Self::Mut(k) => k.span(),
        }
    }
}

impl Spanner for TypePointer {
    fn span(&self) -> Span {
        self.star.span().join(self.elem.span())
    }
}

impl ToTokens for TypePointer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.star.to_tokens(tokens);

        // Raw pointers always spell the mutability: `mut` or `const`.
        match &self.mutability {
            PointerMutability::Mut(kw) => kw.to_tokens(tokens),
            PointerMutability::Const(kw) => kw.to_tokens(tokens),
        }

        self.elem.to_tokens(tokens);
    }
}
