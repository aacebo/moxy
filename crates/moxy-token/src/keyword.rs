use super::ToTokens;
use super::lex::{Cursor, LexError, Scan};
use crate::{Span, Spanner, TokenStream, TokenTree};

macro_rules! define_keyword {
    ($($name:ident[$is_method:ident, $as_method:ident] => $text:literal),+ $(,)?) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum Keyword {
            $($name($name),)*
        }

        impl Keyword {
            #[inline]
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$name(v) => v.as_str(),)*
                }
            }

            #[inline]
            pub fn span(&self) -> Span {
                match self {
                    $(Self::$name(v) => v.span(),)*
                }
            }

            #[inline]
            pub fn set_span(&mut self, span: Span) {
                match self {
                    $(Self::$name(v) => v.set_span(span),)*
                }
            }

            #[inline]
            pub fn from_str(text: &str, span: Span) -> Option<Self> {
                match text {
                    $($text => Some(Self::$name($name::new(span))),)*
                    _ => None,
                }
            }

            #[inline]
            pub fn to_token_tree(&self) -> TokenTree {
                TokenTree::Keyword(self.clone())
            }

            #[inline]
            pub fn into_token_tree(self) -> TokenTree {
                TokenTree::Keyword(self)
            }
        }

        impl Scan for Keyword {
            fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
                let (end, id) = super::Ident::scan(cursor)?;

                if id.is_raw() {
                    return cursor.error().into();
                }

                match id.text() {
                    $($text => Ok((end, Self::$name($name::new(id.span())))),)*
                    _ => cursor.error().into(),
                }
            }
        }

        impl ToTokens for Keyword {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                match self {
                    $(Self::$name(v) => v.to_tokens(tokens),)*
                }
            }
        }

        impl Spanner for Keyword {
            fn span(&self) -> Span {
                self.span()
            }
        }

        impl std::fmt::Display for Keyword {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$name(v) => v.fmt(f),)*
                }
            }
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for Keyword {
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

                #[inline]
                pub fn new(span: Span) -> Self {
                    Self { span }
                }

                #[inline]
                pub fn span(&self) -> Span {
                    self.span
                }

                #[inline]
                pub fn set_span(&mut self, span: Span) {
                    self.span = span;
                }

                #[inline]
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
                    let (end, id) = super::Ident::scan(cursor)?;

                    if !id.is_raw() && id.text() == $text {
                        Ok((end, Self::new(id.span())))
                    } else {
                        cursor.error().into()
                    }
                }
            }

            impl ToTokens for $name {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    tokens.extend_one(TokenTree::Keyword(Keyword::$name(*self)));
                }
            }

            impl Spanner for $name {
                fn span(&self) -> Span {
                    self.span
                }
            }

            impl From<$name> for Keyword {
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
            pub fn is_keyword(&self) -> bool {
                matches!(self, Self::Keyword(_))
            }

            pub fn as_keyword(&self) -> Option<&Keyword> {
                match self {
                    Self::Keyword(v) => Some(v),
                    _ => None,
                }
            }

            $(
                #[doc = concat!("**", stringify!($name), "** (\"", $text, "\")")]
                pub fn $is_method(&self) -> bool {
                    matches!(self, Self::Keyword(Keyword::$name(_)))
                }

                #[doc = concat!("**", stringify!($name), "** (\"", $text, "\")")]
                pub fn $as_method(&self) -> Option<&$name> {
                    match self {
                        Self::Keyword(Keyword::$name(v)) => Some(v),
                        _ => None,
                    }
                }
            )*
        }
    };
}

define_keyword! {
    As[is_keyword_as, as_keyword_as]                            => "as",
    Async[is_keyword_async, as_keyword_async]                   => "async",
    Auto[is_keyword_auto, as_keyword_auto]                      => "auto",
    Await[is_keyword_await, as_keyword_await]                   => "await",
    Become[is_keyword_become, as_keyword_become]                => "become",
    Box[is_keyword_box, as_keyword_box]                         => "box",
    Break[is_keyword_break, as_keyword_break]                   => "break",
    Const[is_keyword_const, as_keyword_const]                   => "const",
    Continue[is_keyword_continue, as_keyword_continue]          => "continue",
    Crate[is_keyword_crate, as_keyword_crate]                   => "crate",
    Default[is_keyword_default, as_keyword_default]             => "default",
    Do[is_keyword_do, as_keyword_do]                            => "do",
    Dyn[is_keyword_dyn, as_keyword_dyn]                         => "dyn",
    Else[is_keyword_else, as_keyword_else]                      => "else",
    Enum[is_keyword_enum, as_keyword_enum]                      => "enum",
    Extern[is_keyword_extern, as_keyword_extern]                => "extern",
    Final[is_keyword_final, as_keyword_final]                   => "final",
    Fn[is_keyword_fn, as_keyword_fn]                            => "fn",
    For[is_keyword_for, as_keyword_for]                         => "for",
    If[is_keyword_if, as_keyword_if]                            => "if",
    Impl[is_keyword_impl, as_keyword_impl]                      => "impl",
    In[is_keyword_in, as_keyword_in]                            => "in",
    Let[is_keyword_let, as_keyword_let]                         => "let",
    Loop[is_keyword_loop, as_keyword_loop]                      => "loop",
    Macro[is_keyword_macro, as_keyword_macro]                   => "macro",
    MacroRules[is_keyword_macro_rules, as_keyword_macro_rules]  => "macro_rules",
    Match[is_keyword_match, as_keyword_match]                   => "match",
    Mod[is_keyword_mod, as_keyword_mod]                         => "mod",
    Move[is_keyword_move, as_keyword_move]                      => "move",
    Mut[is_keyword_mut, as_keyword_mut]                         => "mut",
    Override[is_keyword_override, as_keyword_override]          => "override",
    Priv[is_keyword_priv, as_keyword_priv]                      => "priv",
    Pub[is_keyword_pub, as_keyword_pub]                         => "pub",
    Raw[is_keyword_raw, as_keyword_raw]                         => "raw",
    Ref[is_keyword_ref, as_keyword_ref]                         => "ref",
    Return[is_keyword_return, as_keyword_return]                => "return",
    SelfType[is_keyword_self_type, as_keyword_self_type]        => "Self",
    SelfValue[is_keyword_self_value, as_keyword_self_value]     => "self",
    Static[is_keyword_static, as_keyword_static]                => "static",
    Struct[is_keyword_struct, as_keyword_struct]                => "struct",
    Super[is_keyword_super, as_keyword_super]                   => "super",
    Trait[is_keyword_trait, as_keyword_trait]                   => "trait",
    Try[is_keyword_try, as_keyword_try]                         => "try",
    Type[is_keyword_type, as_keyword_type]                      => "type",
    Typeof[is_keyword_typeof, as_keyword_typeof]                => "typeof",
    Union[is_keyword_union, as_keyword_union]                   => "union",
    Unsafe[is_keyword_unsafe, as_keyword_unsafe]                => "unsafe",
    Unsized[is_keyword_unsized, as_keyword_unsized]             => "unsized",
    Use[is_keyword_use, as_keyword_use]                         => "use",
    Virtual[is_keyword_virtual, as_keyword_virtual]             => "virtual",
    Where[is_keyword_where, as_keyword_where]                   => "where",
    While[is_keyword_while, as_keyword_while]                   => "while",
    Yield[is_keyword_yield, as_keyword_yield]                   => "yield",
}
