use super::Type;
use crate::ast::Mutability;
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{Const, Mut};
use crate::token::punct::Star;
use crate::token::{LexError, ToTokens, Token, TokenTree};
use crate::{Parse, Span, TokenStream};

#[doc = "A raw pointer type (e.g. `*const T`, `*mut T`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypePointer {
    pub span: Span,
    pub mutability: Mutability,
    pub elem: Box<Type>,
}

impl Parse for TypePointer {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let _ = stream.parse::<Star>()?;
        let at = stream.span();

        // A raw pointer requires an explicit `const` or `mut` after the `*`.
        let mutability = match stream.advance() {
            Some(TokenTree::Token(Token::Keyword(kw))) if kw.as_str() == "mut" => Mutability::Mutable,
            Some(TokenTree::Token(Token::Keyword(kw))) if kw.as_str() == "const" => Mutability::Immutable,
            _ => {
                return Err(LexError::new(at).message("expected `const` or `mut` after `*`").into());
            }
        };

        Ok(Self {
            span: Span::default(),
            mutability,
            elem: Box::new(stream.parse()?),
        })
    }
}

impl ToTokens for TypePointer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Star::default().to_tokens(tokens);

        // Raw pointers always spell the mutability: `mut` or `const`.
        match self.mutability {
            Mutability::Mutable => Mut::default().to_tokens(tokens),
            Mutability::Immutable => Const::default().to_tokens(tokens),
        }

        self.elem.to_tokens(tokens);
    }
}
