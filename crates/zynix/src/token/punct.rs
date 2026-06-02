use super::ToTokens;
use super::lex::{Cursor, LexError, Scan};
use crate::parse::{ParseError, ParseStream};
use crate::{Parse, Span, Token, TokenStream, TokenTree};

macro_rules! define_punct {
    ($($name:ident => $text:literal),+ $(,)?) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum Punctuation {
            $($name($name),)*
        }

        impl Punctuation {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$name(v) => v.as_str(),)*
                }
            }

            pub fn span(&self) -> Span {
                match self {
                    $(Self::$name(v) => v.span(),)*
                }
            }

            pub fn set_span(&mut self, span: Span) {
                match self {
                    $(Self::$name(v) => v.set_span(span),)*
                }
            }
        }

        impl ToTokens for Punctuation {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                match self {
                    $(Self::$name(v) => v.to_tokens(tokens),)*
                }
            }
        }

        impl std::fmt::Display for Punctuation {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$name(v) => v.fmt(f),)*
                }
            }
        }

        impl Scan for Punctuation {
            fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
                let mut best: Option<(Cursor<'_>, Self)> = None;

                $(
                    if let Ok((end, op)) = <$name as Scan>::scan(cursor) {
                        let longer = best
                            .as_ref()
                            .is_none_or(|(b, _)| end.offset() > b.offset());

                        if longer {
                            best = Some((end, Self::$name(op)));
                        }
                    }
                )*

                best.ok_or_else(|| cursor.error())
            }
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for Punctuation {
            fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.as_str().serialize(s)
            }
        }

        $(
            #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
            pub struct $name {
                pub span: Span,
            }

            impl $name {
                pub const TEXT: &'static str = $text;

                pub fn new(span: Span) -> Self {
                    Self { span }
                }

                pub fn span(&self) -> Span {
                    self.span
                }

                pub fn set_span(&mut self, span: Span) {
                    self.span = span;
                }

                pub fn as_str(&self) -> &'static str {
                    Self::TEXT
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str($text)
                }
            }

            impl Scan for $name {
                fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
                    if cursor.starts_with($text) {
                        let end = cursor.advance($text.len());
                        Ok((end, Self::new(cursor.span_to(&end))))
                    } else {
                        cursor.error().into()
                    }
                }
            }

            impl Parse for $name {
                fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
                    let at = stream.span();

                    match stream.advance() {
                        Some(TokenTree::Token(Token::Punct(Punctuation::$name(op)))) => {
                            Ok(Self::new(op.span()))
                        }
                        _ => Err(LexError::new(at)
                            .message(concat!("expected `", $text, "`"))
                            .into()),
                    }
                }
            }

            impl ToTokens for $name {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    tokens.extend_one(Token::Punct(Punctuation::$name(*self)).into());
                }
            }

            impl From<$name> for Punctuation {
                fn from(value: $name) -> Self {
                    Self::$name(value)
                }
            }

            #[cfg(feature = "serde")]
            impl serde::Serialize for $name {
                fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    self.as_str().serialize(s)
                }
            }
        )+
    };
}

define_punct! {
    And        => "&",
    Or         => "|",
    Not        => "!",
    Tilde      => "~",
    Plus       => "+",
    Minus      => "-",
    Star       => "*",
    Slash      => "/",
    Percent    => "%",
    Caret      => "^",
    Eq         => "=",
    Lt         => "<",
    Gt         => ">",
    At         => "@",
    Dot        => ".",
    Comma      => ",",
    Semi       => ";",
    Colon      => ":",
    Pound      => "#",
    Dollar     => "$",
    Question   => "?",
    Quote      => "'",

    AndAnd     => "&&",
    OrOr       => "||",
    Shl        => "<<",
    Shr        => ">>",
    EqEq       => "==",
    Ne         => "!=",
    Le         => "<=",
    Ge         => ">=",
    AndEq      => "&=",
    OrEq       => "|=",
    PlusEq     => "+=",
    MinusEq    => "-=",
    StarEq     => "*=",
    SlashEq    => "/=",
    PercentEq  => "%=",
    CaretEq    => "^=",
    FatArrow   => "=>",
    RArrow     => "->",
    LArrow     => "<-",
    PathSep    => "::",
    DotDot     => "..",

    ShlEq      => "<<=",
    ShrEq      => ">>=",
    DotDotDot  => "...",
    DotDotEq   => "..=",
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::TokenStream;
    use crate::token::ToTokenStream;

    #[test]
    fn parse_comma() {
        let ts = TokenStream::from_str(",").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Comma>().is_ok());
    }

    #[test]
    fn parse_eq_eq() {
        let ts = TokenStream::from_str("==").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<EqEq>().is_ok());
    }

    #[test]
    fn alone_spaced_eq_not_eq_eq() {
        let ts = TokenStream::from_str("= =").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<EqEq>().is_err());
    }

    #[test]
    fn alone_spaced_eq_parses_as_eq() {
        let ts = TokenStream::from_str("= =").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Eq>().is_ok());
    }

    #[test]
    fn parse_dot_dot_eq() {
        let ts = TokenStream::from_str("..=").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<DotDotEq>().is_ok());
    }

    #[test]
    fn lexes_whole_operators() {
        use crate::token::Token;

        let ops: Vec<Punctuation> = TokenStream::from_str("a == b => c :: d ..= e")
            .unwrap()
            .into_iter()
            .filter_map(|tt| match tt {
                TokenTree::Token(Token::Punct(op)) => Some(op),
                _ => None,
            })
            .collect();

        assert!(matches!(ops[0], Punctuation::EqEq(_)));
        assert!(matches!(ops[1], Punctuation::FatArrow(_)));
        assert!(matches!(ops[2], Punctuation::PathSep(_)));
        assert!(matches!(ops[3], Punctuation::DotDotEq(_)));
    }

    #[test]
    fn shr_is_one_whole_op() {
        use crate::token::Token;

        let toks: Vec<TokenTree> = TokenStream::from_str("a >> b").unwrap().into_iter().collect();
        let op_count = toks.iter().filter(|t| matches!(t, TokenTree::Token(Token::Punct(_)))).count();
        assert_eq!(op_count, 1);
        assert!(matches!(toks[1], TokenTree::Token(Token::Punct(Punctuation::Shr(_)))));
    }

    #[test]
    fn parse_path_sep() {
        let ts = TokenStream::from_str("::").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<PathSep>().is_ok());
    }

    #[test]
    fn parse_fat_arrow() {
        let ts = TokenStream::from_str("=>").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<FatArrow>().is_ok());
    }

    #[test]
    fn roundtrip_comma() {
        let s = Comma::default().to_token_stream().to_string();
        let ts = TokenStream::from_str(&s).unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Comma>().is_ok());
    }

    #[test]
    fn roundtrip_eq_eq() {
        let s = EqEq::default().to_token_stream().to_string();
        let ts = TokenStream::from_str(&s).unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<EqEq>().is_ok());
    }

    #[test]
    fn underscore_lexes_as_ident() {
        use crate::token::Ident;

        let ts = TokenStream::from_str("_").unwrap();
        let tree = ts.into_iter().next().unwrap();
        let TokenTree::Token(Token::Ident(id)) = tree else {
            panic!("expected `_` to lex as an ident");
        };
        assert_eq!(id.name().as_ref(), "_");

        let ts = TokenStream::from_str("_").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Ident>().is_ok());
    }

    #[test]
    fn display_strings() {
        assert_eq!(format!("{}", Comma::default()), ",");
        assert_eq!(format!("{}", EqEq::default()), "==");
        assert_eq!(format!("{}", DotDotEq::default()), "..=");
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;
        use crate::Token;

        #[test]
        fn punct_serializes_as_string() {
            let ts = TokenStream::from_str("+").unwrap();
            let tree = ts.into_iter().next().unwrap();
            let TokenTree::Token(Token::Punct(p)) = tree else {
                panic!("expected punct");
            };
            assert_eq!(serde_json::to_value(&p).unwrap(), serde_json::json!("+"));
        }

        #[test]
        fn named_punct_serializes_as_string() {
            assert_eq!(serde_json::to_value(EqEq::default()).unwrap(), serde_json::json!("=="));
        }

        #[test]
        fn punctuation_serializes_as_string() {
            let p = Punctuation::from(Comma::default());
            assert_eq!(serde_json::to_value(p).unwrap(), serde_json::json!(","));
        }
    }
}
