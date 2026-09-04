use super::ToTokens;
use super::lex::{Cursor, LexError, Scan};
use crate::{Spacing, Span, Spanner, TokenStream, TokenTree};

fn spacing_after(text: &str, cursor: Cursor<'_>) -> Spacing {
    let Some(next) = cursor.first() else {
        return Spacing::Alone;
    };

    let current = text.chars().next().unwrap_or_default();
    let is_compound_pair = matches!(
        (current, next),
        ('&', '&' | '=')
            | ('|', '|' | '=')
            | ('<', '<' | '=' | '-')
            | ('>', '>' | '=')
            | ('=', '=' | '>')
            | ('!', '=')
            | ('+', '=')
            | ('-', '=' | '>')
            | ('*', '=')
            | ('/', '=')
            | ('%', '=')
            | ('^', '=')
            | (':', ':')
            | ('.', '.' | '=')
    );

    if is_compound_pair || (current == '\'' && (next == '_' || next.is_alphabetic())) {
        Spacing::Joint
    } else {
        Spacing::Alone
    }
}

macro_rules! define_punct {
    ($($name:ident[$is_method:ident, $as_method:ident] => $text:literal),+ $(,)?) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum Punct {
            $($name($name),)*
        }

        impl Punct {
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

            pub fn spacing(&self) -> Spacing {
                match self {
                    $(Self::$name(v) => v.spacing(),)*
                }
            }

            pub fn set_spacing(&mut self, spacing: Spacing) {
                match self {
                    $(Self::$name(v) => v.set_spacing(spacing),)*
                }
            }

            #[inline]
            pub fn to_token_tree(&self) -> TokenTree {
                TokenTree::Punct(*self)
            }

            #[inline]
            pub fn into_token_tree(self) -> TokenTree {
                TokenTree::Punct(self)
            }
        }

        impl ToTokens for Punct {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                match self {
                    $(Self::$name(v) => v.to_tokens(tokens),)*
                }
            }
        }

        impl Spanner for Punct {
            fn span(&self) -> Span {
                self.span()
            }
        }

        impl std::fmt::Display for Punct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$name(v) => v.fmt(f),)*
                }
            }
        }

        impl Scan for Punct {
            fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
                let mut end = cursor;
                let mut best = None;

                while let Some(ch) = end.first() && ch.is_ascii_punctuation() {
                    end = end.advance_by(ch.len_utf8());
                    let text = cursor.slice_to(end);

                    $(
                        if $name::TEXT == text {
                            best = Some((end, Self::$name($name {
                                span: cursor.span_to(&end),
                                spacing: spacing_after($name::TEXT, end),
                            })));
                            continue;
                        }
                    )*

                    break;
                }

                best.ok_or_else(|| cursor.error())
            }
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for Punct {
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
                spacing: Spacing,
            }

            impl $name {
                pub const TEXT: &'static str = $text;

                pub fn new(span: Span) -> Self {
                    Self {
                        span,
                        spacing: Spacing::Alone,
                    }
                }

                pub fn span(&self) -> Span {
                    self.span
                }

                pub fn set_span(&mut self, span: Span) {
                    self.span = span;
                }

                pub fn spacing(&self) -> Spacing {
                    self.spacing
                }

                pub fn set_spacing(&mut self, spacing: Spacing) {
                    self.spacing = spacing;
                }

                pub fn with_spacing(mut self, spacing: Spacing) -> Self {
                    self.spacing = spacing;
                    self
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
                    let end = cursor.advance_by($text.len());
                    let text = cursor.slice_to(end);

                    if text == $text {
                        Ok((
                            end,
                            Self::new(cursor.span_to(&end)).with_spacing(spacing_after(Self::TEXT, end)),
                        ))
                    } else {
                        cursor.error().into()
                    }
                }
            }

            impl ToTokens for $name {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    tokens.extend_one(TokenTree::Punct(Punct::$name(*self)));
                }
            }

            impl Spanner for $name {
                fn span(&self) -> Span {
                    self.span
                }
            }

            impl From<$name> for Punct {
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

            pub fn as_punct(&self) -> Option<&Punct> {
                match self {
                    Self::Punct(v) => Some(v),
                    _ => None,
                }
            }

            $(
                #[doc = concat!("**", stringify!($name), "** (\"", $text, "\")")]
                pub fn $is_method(&self) -> bool {
                    matches!(self, Self::Punct(Punct::$name(_)))
                }

                #[doc = concat!("**", stringify!($name), "** (\"", $text, "\")")]
                pub fn $as_method(&self) -> Option<&$name> {
                    match self {
                        Self::Punct(Punct::$name(v)) => Some(v),
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
    Lt[is_punct_lt, as_punct_lt]                            => "<",
    Gt[is_punct_gt, as_punct_gt]                            => ">",
    At[is_punct_at, as_punct_at]                            => "@",
    Dot[is_punct_dot, as_punct_dot]                         => ".",
    Comma[is_punct_comma, as_punct_comma]                   => ",",
    Semi[is_punct_semi, as_punct_semi]                      => ";",
    Colon[is_punct_colon, as_punct_colon]                   => ":",
    Pound[is_punct_pound, as_punct_pound]                   => "#",
    Dollar[is_punct_dollar, as_punct_dollar]                => "$",
    Question[is_punct_question, as_punct_question]          => "?",
    Quote[is_punct_quote, as_punct_quote]                   => "'",
}
