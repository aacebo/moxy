use moxy_token::{Punct, TokenTree, punct};

use crate::{Parse, ParseError, Parser, Peek};

impl Peek for Punct {
    fn peek(parser: &Parser) -> bool {
        let Some(next) = parser.curr() else {
            return false;
        };

        next.is_punct()
    }
}

impl Parse for Punct {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.advance() {
            Some(TokenTree::Punct(v)) => Ok(*v),
            _ => Err(parser.error("expected punctuation")),
        }
    }
}

macro_rules! impl_punct_parse {
    ($($name:ident),* $(,)?) => {
        $(
            impl Peek for punct::$name {
                fn peek(parser: &Parser) -> bool {
                    let Some(next) = parser.curr() else {
                        return false;
                    };

                    matches!(next, TokenTree::Punct(Punct::$name(_)))
                }
            }

            impl Parse for punct::$name {
                fn parse(parser: &Parser) -> Result<Self, ParseError> {
                    match parser.parse::<Punct>()? {
                        Punct::$name(v) => Ok(v),
                        _ => parser.error(format!("expected `{}` punctuation", punct::$name::TEXT)).into(),
                    }
                }
            }
        )*
    };
}

impl_punct_parse! {
    And,
    Or,
    Not,
    Tilde,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Eq,
    Lt,
    Gt,
    At,
    Dot,
    Comma,
    Semi,
    Colon,
    Pound,
    Dollar,
    Question,
    Quote,

    AndAnd,
    OrOr,
    Shl,
    Shr,
    EqEq,
    Ne,
    Le,
    Ge,
    AndEq,
    OrEq,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    CaretEq,
    FatArrow,
    RArrow,
    LArrow,
    PathSep,
    DotDot,

    ShlEq,
    ShrEq,
    DotDotDot,
    DotDotEq,
}
