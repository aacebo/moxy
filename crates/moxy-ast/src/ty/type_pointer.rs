use moxy_token::keyword::{Const, Mut};
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Star;
use moxy_token::{LexError, Parse, Span, ToTokens, Token, TokenStream, TokenTree};

use super::Type;

#[doc = "Whether a raw pointer is `*const` or `*mut`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum PointerMutability {
    Const(Const),
    Mut(Mut),
}

#[doc = "A raw pointer type (e.g. `*const T`, `*mut T`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypePointer {
    pub span: Span,
    pub star: Star,
    pub mutability: PointerMutability,
    pub elem: Box<Type>,
}

impl Parse for TypePointer {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let star = stream.parse::<Star>()?;
        let at = stream.span();

        // A raw pointer requires an explicit `const` or `mut` after the `*`.
        let mutability = match stream.advance() {
            Some(TokenTree::Token(Token::Keyword(kw))) if kw.as_str() == "mut" => PointerMutability::Mut(Mut::new(kw.span())),
            Some(TokenTree::Token(Token::Keyword(kw))) if kw.as_str() == "const" => {
                PointerMutability::Const(Const::new(kw.span()))
            }
            _ => {
                return Err(LexError::new(at).message("expected `const` or `mut` after `*`").into());
            }
        };

        Ok(Self {
            span: Span::default(),
            star,
            mutability,
            elem: Box::new(stream.parse()?),
        })
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
