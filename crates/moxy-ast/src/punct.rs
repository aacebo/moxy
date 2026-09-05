use moxy_token::punct::*;
use moxy_token::{Punct, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::{Parse, ParseError, Parser, Peek};

macro_rules! define_punct {
    ($($name:ident($len:literal) => [ $($field:tt : $punct:ident),+ ]),+ $(,)?) => {
        $(
            #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
            pub struct $name($(pub $punct),*);

            impl $name {
                pub fn new(span: Span) -> Self {
                    Self($($punct::new(span)),*)
                }

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
                    let mut i = 0;

                    $(
                        let Some(TokenTree::Punct(Punct::$punct(token))) = parser.advance() else {
                            return false;
                        };

                        i += 1;

                        if i < $len && !token.spacing().is_joint() {
                            return false;
                        }
                    )*

                    true
                }
            }

            impl Parse for $name {
                fn parse(parser: &Parser) -> Result<Self, ParseError> {
                    Ok(Self($(parser.parse::<$punct>()?),*))
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
    AndAnd(2) => [0: And, 1: And],
    OrOr(2) => [0: Or, 1: Or],
    Shl(2) => [0: Lt, 1: Lt],
    Shr(2) => [0: Gt, 1: Gt],
    EqEq(2) => [0: Eq, 1: Eq],
    Ne(2) => [0: Not, 1: Eq],
    Le(2) => [0: Lt, 1: Eq],
    Ge(2) => [0: Gt, 1: Eq],
    AndEq(2) => [0: And, 1: Eq],
    OrEq(2) => [0: Or, 1: Eq],
    PlusEq(2) => [0: Plus, 1: Eq],
    MinusEq(2) => [0: Minus, 1: Eq],
    StarEq(2) => [0: Star, 1: Eq],
    SlashEq(2) => [0: Slash, 1: Eq],
    PercentEq(2) => [0: Percent, 1: Eq],
    CaretEq(2) => [0: Caret, 1: Eq],
    FatArrow(2) => [0: Eq, 1: Gt],
    RArrow(2) => [0: Minus, 1: Gt],
    LArrow(2) => [0: Lt, 1: Minus],
    PathSep(2) => [0: Colon, 1: Colon],
    DotDot(2) => [0: Dot, 1: Dot],
    ShlEq(2) => [0: Lt, 1: Lt, 2: Eq],
    ShrEq(2) => [0: Gt, 1: Gt, 2: Eq],
    DotDotDot(3) => [0: Dot, 1: Dot, 2: Dot],
    DotDotEq(3) => [0: Dot, 1: Dot, 2: Eq],
}
