use moxy_token::{Punctuation, TokenTree, punct};

use crate::{Parse, ParseError, Parser, Peek};

impl Peek for Punctuation {
    fn peek(parser: &Parser) -> bool {
        let Some(next) = parser.next() else {
            return false;
        };

        next.is_punct()
    }
}

impl Parse for Punctuation {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.advance() {
            Some(TokenTree::Punct(v)) => Ok(*v),
            _ => parser.error("expected punctuation").into(),
        }
    }
}

macro_rules! impl_punct_parse {
    ($($name:ident => $text:literal),* $(,)?) => {
        $(
            impl Peek for punct::$name {
                fn peek(parser: &Parser) -> bool {
                    let Some(next) = parser.next() else {
                        return false;
                    };

                    matches!(next, TokenTree::Punct(Punctuation::$name(_)))
                }
            }

            impl Parse for punct::$name {
                fn parse(parser: &Parser) -> Result<Self, ParseError> {
                    match parser.parse::<Punctuation>()? {
                        Punctuation::$name(v) => Ok(v),
                        _ => parser
                            .error(concat!("expected `", $text, "` punctuation"))
                            .into(),
                    }
                }
            }
        )*
    };
}

impl_punct_parse! {
    And => "&",
    Or => "|",
    Not => "!",
    Tilde => "~",
    Plus => "+",
    Minus => "-",
    Star => "*",
    Slash => "/",
    Percent => "%",
    Caret => "^",
    Eq => "=",
    Lt => "<",
    Gt => ">",
    At => "@",
    Dot => ".",
    Comma => ",",
    Semi => ";",
    Colon => ":",
    Pound => "#",
    Dollar => "$",
    Question => "?",
    Quote => "'",

    AndAnd => "&&",
    OrOr => "||",
    Shl => "<<",
    Shr => ">>",
    EqEq => "==",
    Ne => "!=",
    Le => "<=",
    Ge => ">=",
    AndEq => "&=",
    OrEq => "|=",
    PlusEq => "+=",
    MinusEq => "-=",
    StarEq => "*=",
    SlashEq => "/=",
    PercentEq => "%=",
    CaretEq => "^=",
    FatArrow => "=>",
    RArrow => "->",
    LArrow => "<-",
    PathSep => "::",
    DotDot => "..",

    ShlEq => "<<=",
    ShrEq => ">>=",
    DotDotDot => "...",
    DotDotEq => "..=",
}
