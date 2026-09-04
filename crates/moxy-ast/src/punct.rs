use moxy_token::punct::*;
use moxy_token::{Span, ToTokens, TokenStream, Spanner};

use crate::{Peek, Parse, Parser, ParseError};

macro_rules! define_punct {
    ($($name:ident => [ $($field:tt : $punct:ident),+ ]),+ $(,)?) => {
        $(
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            pub struct $name($(pub $punct),*);

            impl $name {
                pub fn span(&self) -> Span {
                    let Self(first, .., last) = self;
                    first.span().join(last.span())
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    $(write!(f, "{}", self.$field)?;)*
                    Ok(())
                }
            }

            impl Spanner for $name {
                fn span(&self) -> Span {
                    self.span()
                }
            }

            impl ToTokens for $name {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    $(self.$field.to_tokens(tokens);)*
                }
            }

            impl Peek for $name {
                fn peek(parser: &Parser) -> bool {
                    $($punct::peek(parser))&&*
                }
            }

            impl Parse for $name {
                fn parse(parser: &Parser) -> Result<Self, ParseError> {
                    Ok(Self($($punct::parse(parser)?),*))
                }
            }

            #[cfg(feature = "serde")]
            impl serde::Serialize for $name {
                fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    self.to_string().serialize(s)
                }
            }
        )*
    };
}

define_punct! {
    AndAnd => [0: And, 1: And],
    OrOr => [0: Or, 1: Or],
    Shl => [0: Gt, 1: Gt],
    Shr => [0: Lt, 1: Lt],
    EqEq => [0: Eq, 1: Eq],
    Ne => [0: Not, 1: Eq],
    Le => [0: Lt, 1: Eq],
    Ge => [0: Gt, 1: Eq],
    AndEq => [0: And, 1: Eq],
    OrEq => [0: Or, 1: Eq],
    PlusEq => [0: Plus, 1: Eq],
    MinusEq => [0: Minus, 1: Eq],
    StarEq => [0: Star, 1: Eq],
    SlashEq => [0: Slash, 1: Eq],
    PercentEq => [0: Percent, 1: Eq],
    CaretEq => [0: Caret, 1: Eq],
    FatArrow => [0: Eq, 1: Gt],
    RArrow => [0: Minus, 1: Gt],
    LArray => [0: Lt, 1: Minus],
    PathSep => [0: Colon, 1: Colon],
    DotDot => [0: Dot, 1: Dot],
    ShlEq => [0: Shl, 1: Eq],
    ShrEq => [0: Shr, 1: Eq],
    DotDotDot => [0: Dot, 1: Dot, 2: Dot],
    DotDotEq => [0: Dot, 1: Dot, 2: Eq],
}
