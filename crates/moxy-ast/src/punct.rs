use moxy_token::punct::*;
use moxy_token::{Spacing, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::{Parse, ParseError, Parser, Peek};

macro_rules! define_punct {
    ($($name:ident => [ $($field:tt : $punct:ident),+ ]),+ $(,)?) => {
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
                    let mut puncts = vec![$(TokenTree::from(moxy_token::Punct::from(self.$field))),*];
                    let last = puncts.len() - 1;

                    for punct in puncts.iter_mut().take(last) {
                        if let TokenTree::Punct(punct) = punct {
                            punct.set_spacing(Spacing::Joint);
                        }
                    }

                    tokens.extend(puncts);
                }
            }

            impl Peek for $name {
                fn peek(parser: &Parser) -> bool {
                    let parser = parser.lookahead();

                    $(
                        if !matches!(
                            parser.advance(),
                            Some(TokenTree::Punct(moxy_token::Punct::$punct(_)))
                        ) {
                            return false;
                        }
                    )*

                    true
                }
            }

            impl Parse for $name {
                fn parse(parser: &Parser) -> Result<Self, ParseError> {
                    Ok(Self($({
                        match parser.parse::<moxy_token::Punct>()? {
                            moxy_token::Punct::$punct(value) => value,
                            _ => return Err(parser.error(concat!("expected `", stringify!($punct), "` punctuation"))),
                        }
                    }),*))
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
    Shl => [0: Lt, 1: Lt],
    Shr => [0: Gt, 1: Gt],
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
    LArrow => [0: Lt, 1: Minus],
    PathSep => [0: Colon, 1: Colon],
    DotDot => [0: Dot, 1: Dot],
    ShlEq => [0: Lt, 1: Lt, 2: Eq],
    ShrEq => [0: Gt, 1: Gt, 2: Eq],
    DotDotDot => [0: Dot, 1: Dot, 2: Dot],
    DotDotEq => [0: Dot, 1: Dot, 2: Eq],
}
