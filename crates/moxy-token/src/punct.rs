use super::ToTokens;
use super::lex::{Cursor, LexError, Scan};
use crate::{Span, Spanner, TokenStream, TokenTree};

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
