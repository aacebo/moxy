use moxy_token::{Keyword, TokenTree, keyword};

use crate::{Parse, ParseError, Parser, Peek};

impl Peek for Keyword {
    fn peek(parser: &Parser) -> bool {
        let Some(next) = parser.curr() else {
            return false;
        };

        next.is_keyword()
    }
}

impl Parse for Keyword {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        match parser.advance() {
            Some(TokenTree::Keyword(v)) => Ok(*v),
            _ => Err(parser.error("expected keyword")),
        }
    }
}

macro_rules! impl_keyword_parse {
    ($($name:ident => $text:literal),* $(,)?) => {
        $(
            impl Peek for keyword::$name {
                fn peek(parser: &Parser) -> bool {
                    let Some(next) = parser.curr() else {
                        return false;
                    };

                    matches!(next, TokenTree::Keyword(Keyword::$name(_)))
                }
            }

            impl Parse for keyword::$name {
                fn parse(parser: &Parser) -> Result<Self, ParseError> {
                    match parser.parse::<Keyword>()? {
                        Keyword::$name(v) => Ok(v),
                        _ => Err(parser.error(concat!("expected `", $text, "` keyword"))),
                    }
                }
            }
        )*
    };
}

impl_keyword_parse! {
    As => "as",
    Async => "async",
    Auto => "auto",
    Await => "await",
    Become => "become",
    Box => "box",
    Break => "break",
    Const => "const",
    Continue => "continue",
    Crate => "crate",
    Default => "default",
    Do => "do",
    Dyn => "dyn",
    Else => "else",
    Enum => "enum",
    Extern => "extern",
    Final => "final",
    Fn => "fn",
    For => "for",
    If => "if",
    Impl => "impl",
    In => "in",
    Let => "let",
    Loop => "loop",
    Macro => "macro",
    MacroRules => "macro_rules",
    Match => "match",
    Mod => "mod",
    Move => "move",
    Mut => "mut",
    Override => "override",
    Priv => "priv",
    Pub => "pub",
    Raw => "raw",
    Ref => "ref",
    Return => "return",
    SelfType => "Self",
    SelfValue => "self",
    Static => "static",
    Struct => "struct",
    Super => "super",
    Trait => "trait",
    Try => "try",
    Type => "type",
    Typeof => "typeof",
    Union => "union",
    Unsafe => "unsafe",
    Unsized => "unsized",
    Use => "use",
    Virtual => "virtual",
    Where => "where",
    While => "while",
    Yield => "yield",
}
