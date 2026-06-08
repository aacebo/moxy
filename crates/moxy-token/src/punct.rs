use super::ToTokens;
use super::lex::{Cursor, LexError, Scan};
use crate::parser::{ParseError, ParseStream};
use crate::{Parse, Span, Spanner, TokenStream, TokenTree};

macro_rules! define_punct {
    ($($name:ident[$is_method:ident, $as_method:ident] $($split:ident)? => $text:literal),+ $(,)?) => {
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

            #[inline]
            pub fn to_token_tree(&self) -> TokenTree {
                TokenTree::Punct(self.clone())
            }

            #[inline]
            pub fn into_token_tree(self) -> TokenTree {
                TokenTree::Punct(self)
            }
        }

        impl ToTokens for Punctuation {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                match self {
                    $(Self::$name(v) => v.to_tokens(tokens),)*
                }
            }
        }

        impl Spanner for Punctuation {
            fn span(&self) -> Span {
                self.span()
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
                span: Span,
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

            define_punct!(@parse $name, $text $(, $split)?);

            impl ToTokens for $name {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    tokens.extend_one(TokenTree::Punct(Punctuation::$name(*self)));
                }
            }

            impl Spanner for $name {
                fn span(&self) -> Span {
                    self.span
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

        impl TokenTree {
            pub fn is_punct(&self) -> bool {
                matches!(self, Self::Punct(_))
            }

            pub fn as_punct(&self) -> Option<&Punctuation> {
                match self {
                    Self::Punct(v) => Some(v),
                    _ => None,
                }
            }

            $(
                #[doc = concat!("**", stringify!($name), "** (\"", $text, "\")")]
                pub fn $is_method(&self) -> bool {
                    matches!(self, Self::Punct(Punctuation::$name(_)))
                }

                #[doc = concat!("**", stringify!($name), "** (\"", $text, "\")")]
                pub fn $as_method(&self) -> Option<&$name> {
                    match self {
                        Self::Punct(Punctuation::$name(v)) => Some(v),
                        _ => None,
                    }
                }
            )*
        }
    };

    // Splitting parse: accept a glued punct (`>>`, `>=`, ...) by peeling off the
    // first char and leaving the remainder pending. Used by `Gt`/`Lt` so nested
    // generics like `Vec<Box<T>>` parse without the lexer pre-splitting `>>`.
    (@parse $name:ident, $text:literal, split) => {
        impl Parse for $name {
            fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
                let at = stream.span();

                match stream.eat_punct_head($text) {
                    Some(span) => Ok(Self::new(span)),
                    None => Err(LexError::new(at)
                        .message(concat!("expected `", $text, "`"))
                        .into()),
                }
            }
        }
    };

    // Exact-match parse: consume the next token only if it is exactly this punct.
    // Reads via `curr` so a pending split half (e.g. the `>=` left after a `>`
    // was peeled from `>>=`) is matched and consumed correctly.
    (@parse $name:ident, $text:literal) => {
        impl Parse for $name {
            fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
                let at = stream.span();

                match stream.curr() {
                    Some(TokenTree::Punct(Punctuation::$name(op))) => {
                        let span = op.span();
                        stream.advance();
                        Ok(Self::new(span))
                    }
                    _ => Err(LexError::new(at)
                        .message(concat!("expected `", $text, "`"))
                        .into()),
                }
            }
        }
    };
}

define_punct! {
    And[is_punct_and, as_punct_and]                         => "&",
    Or[is_punct_or, as_punct_or]                            => "|",
    Not[is_punct_not, as_punct_not]                         => "!",
    Tilde[is_punct_tilde, as_punct_tilde]                   => "~",
    Plus[is_punct_plus, as_punct_plus]                      => "+",
    Minus[is_punct_minus, as_punct_minus]                   => "-",
    Star[is_punct_star, as_punct_star]                      => "*",
    Slash[is_punct_slash, as_punct_slash]                   => "/",
    Percent[is_punct_percent, as_punct_percent]             => "%",
    Caret[is_punct_caret, as_punct_caret]                   => "^",
    Eq[is_punct_eq, as_punct_eq]                            => "=",
    Lt[is_punct_lt, as_punct_lt] split                      => "<",
    Gt[is_punct_gt, as_punct_gt] split                      => ">",
    At[is_punct_at, as_punct_at]                            => "@",
    Dot[is_punct_dot, as_punct_dot]                         => ".",
    Comma[is_punct_comma, as_punct_comma]                   => ",",
    Semi[is_punct_semi, as_punct_semi]                      => ";",
    Colon[is_punct_colon, as_punct_colon]                   => ":",
    Pound[is_punct_pound, as_punct_pound]                   => "#",
    Dollar[is_punct_dollar, as_punct_dollar]                => "$",
    Question[is_punct_question, as_punct_question]          => "?",
    Quote[is_punct_quote, as_punct_quote]                   => "'",

    AndAnd[is_punct_and_and, as_punct_and_and]              => "&&",
    OrOr[is_punct_or_or, as_punct_or_or]                    => "||",
    Shl[is_punct_shl, as_punct_shl]                         => "<<",
    Shr[is_punct_shr, as_punct_shr]                         => ">>",
    EqEq[is_punct_eq_eq, as_punct_eq_eq]                    => "==",
    Ne[is_punct_ne, as_punct_ne]                            => "!=",
    Le[is_punct_le, as_punct_le]                            => "<=",
    Ge[is_punct_ge, as_punct_ge]                            => ">=",
    AndEq[is_punct_and_eq, as_punct_and_eq]                 => "&=",
    OrEq[is_punct_or_eq, as_punct_or_eq]                    => "|=",
    PlusEq[is_punct_plus_eq, as_punct_plus_eq]              => "+=",
    MinusEq[is_punct_minus_eq, as_punct_minus_eq]           => "-=",
    StarEq[is_punct_star_eq, as_punct_star_eq]              => "*=",
    SlashEq[is_punct_slash_eq, as_punct_slash_eq]           => "/=",
    PercentEq[is_punct_percent_eq, as_punct_percent_eq]     => "%=",
    CaretEq[is_punct_caret_eq, as_punct_caret_eq]           => "^=",
    FatArrow[is_punct_fat_arrow, as_punct_fat_arrow]        => "=>",
    RArrow[is_punct_rarrow, as_punct_rarrow]                => "->",
    LArrow[is_punct_larrow, as_punct_larrow]                => "<-",
    PathSep[is_punct_path_sep, as_punct_path_sep]           => "::",
    DotDot[is_punct_dot_dot, as_punct_dot_dot]              => "..",

    ShlEq[is_punct_shl_eq, as_punct_shl_eq]                 => "<<=",
    ShrEq[is_punct_shr_eq, as_punct_shr_eq]                 => ">>=",
    DotDotDot[is_punct_dot_dot_dot, as_punct_dot_dot_dot]   => "...",
    DotDotEq[is_punct_dot_dot_eq, as_punct_dot_dot_eq]      => "..=",
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::{ToTokenStream, TokenStream};

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
        let ops: Vec<Punctuation> = TokenStream::from_str("a == b => c :: d ..= e")
            .unwrap()
            .into_iter()
            .filter_map(|tt| match tt {
                TokenTree::Punct(op) => Some(op),
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
        let toks: Vec<TokenTree> = TokenStream::from_str("a >> b").unwrap().into_iter().collect();
        let op_count = toks.iter().filter(|t| matches!(t, TokenTree::Punct(_))).count();
        assert_eq!(op_count, 1);
        assert!(matches!(toks[1], TokenTree::Punct(Punctuation::Shr(_))));
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
        use crate::Ident;

        let ts = TokenStream::from_str("_").unwrap();
        let tree = ts.into_iter().next().unwrap();
        let TokenTree::Ident(id) = tree else {
            panic!("expected `_` to lex as an ident");
        };
        assert_eq!(id.text(), "_");

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

    #[test]
    fn gt_splits_shr() {
        // `>>` lexes as one `Shr`, but two `Gt` parses peel it apart (nested generics).
        let ts = TokenStream::from_str(">>").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Gt>().is_ok());
        assert!(!ps.is_empty());
        assert!(ps.parse::<Gt>().is_ok());
        assert!(ps.is_empty());
    }

    #[test]
    fn gt_splits_shr_eq() {
        // `>>=` -> `>` then leftover `>=`.
        let ts = TokenStream::from_str(">>=").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Gt>().is_ok());
        assert!(ps.parse::<Ge>().is_ok());
        assert!(ps.is_empty());
    }

    #[test]
    fn lt_splits_shl() {
        let ts = TokenStream::from_str("<<").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Lt>().is_ok());
        assert!(ps.parse::<Lt>().is_ok());
        assert!(ps.is_empty());
    }

    #[test]
    fn shr_still_parses_whole() {
        // Requesting `Shr` directly still consumes the whole glued op (binary `>>`).
        let ts = TokenStream::from_str(">>").unwrap();
        let mut ps = ts.parse();
        assert!(ps.parse::<Shr>().is_ok());
        assert!(ps.is_empty());
    }

    #[test]
    fn peek_gt_sees_shr() {
        let ts = TokenStream::from_str(">>").unwrap();
        let mut ps = ts.parse();
        assert!(ps.peek::<Gt>());
        // peek does not consume: `Shr` is still wholly parseable afterwards.
        assert!(ps.parse::<Shr>().is_ok());
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn punct_serializes_as_string() {
            let ts = TokenStream::from_str("+").unwrap();
            let tree = ts.into_iter().next().unwrap();
            let TokenTree::Punct(p) = tree else {
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
