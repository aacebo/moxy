#![cfg_attr(nightly, feature(proc_macro_diagnostic, proc_macro_span, proc_macro_def_site,))]

//! # Moxy tokens
//!
//! Token types, lexing, spans, and compiler-token conversions for Rust syntax
//! tooling. [`TokenStream`] is an owned sequence of [`TokenTree`] values that
//! preserves token text, punctuation spacing, and source spans.
//!
//! ## Quick start
//!
//! Parse source into a token stream, or construct individual token types with
//! the provided macros:
//!
//! ```ignore
//! use moxy::token::{ident, TokenStream};
//! use moxy::Token;
//!
//! let stream: TokenStream = "fn generated() {}".parse().unwrap();
//! let name = ident!(generated);
//! let semi: Token![;] = Default::default();
//! assert_eq!(name.to_string(), "generated");
//! assert_eq!(semi.to_string(), ";");
//! ```
//!
//! ## Integrations
//!
//! Enable `serde` to serialize supported token types. Enable `proc-macro2` for
//! conversions with `proc_macro2`; compiler `proc_macro` conversions are
//! available through [`bridge`].

extern crate proc_macro;

mod bridge;
mod delim;
mod group;
mod ident;
mod keyword;
mod lex;
mod lit;
mod punct;
/// Source files, locations, and source maps used by spans.
pub mod source;
mod spacing;
mod span;
mod stream;
mod tree;

#[doc(inline)]
pub use delim::*;
#[doc(inline)]
pub use group::*;
#[doc(inline)]
pub use ident::*;
#[doc(inline)]
pub use keyword::*;
#[doc(inline)]
pub use lex::*;
#[doc(inline)]
pub use lit::*;
#[doc(inline)]
pub use punct::*;
#[doc(inline)]
pub use spacing::*;
#[doc(inline)]
pub use span::{DelimSpan, RangeSpan, Span, Spanner};
#[doc(inline)]
pub use stream::*;
#[doc(inline)]
pub use tree::*;

/// Emits a value as tokens into a destination token stream.
/// A public Rust token API type for to tokens<t.
pub trait ToTokens<T = TokenStream> {
    fn to_tokens(&self, tokens: &mut T);
}

/// Convenience methods for values that can emit a [`TokenStream`].
/// A public Rust token API type for to token stream:.
pub trait ToTokenStream: ToTokens<TokenStream> {
    fn to_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens(&mut tokens);
        tokens
    }

    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        self.to_token_stream()
    }
}

impl<X: ToTokens<TokenStream> + ?Sized> ToTokenStream for X {}

impl<T: ToTokens> ToTokens for ::std::boxed::Box<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<T: ToTokens> ToTokens for &T {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        ToTokens::to_tokens(*self, tokens);
    }
}

impl<T: ToTokens> ToTokens for Option<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(v) = self {
            v.to_tokens(tokens);
        }
    }
}

impl<T: ToTokens> ToTokens for Vec<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for v in self {
            v.to_tokens(tokens);
        }
    }
}

impl<T: ToTokens, E: ToTokens> ToTokens for Result<T, E> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ok(v) => v.to_tokens(tokens),
            Self::Err(err) => err.to_tokens(tokens),
        }
    }
}

/// Map a Rust punctuation or keyword symbol to its [`crate`] token type.
#[macro_export]
macro_rules! Token {
    // --- punctuation: single char ---
    [&]     => { $crate::And };
    [|]     => { $crate::Or };
    [!]     => { $crate::Not };
    [~]     => { $crate::Tilde };
    [+]     => { $crate::Plus };
    [-]     => { $crate::Minus };
    [_]     => { $crate::Underscore };
    [*]     => { $crate::Star };
    [/]     => { $crate::Slash };
    [%]     => { $crate::Percent };
    [^]     => { $crate::Caret };
    [=]     => { $crate::Eq };
    [<]     => { $crate::Lt };
    [>]     => { $crate::Gt };
    [@]     => { $crate::At };
    [.]     => { $crate::Dot };
    [,]     => { $crate::Comma };
    [;]     => { $crate::Semi };
    [:]     => { $crate::Colon };
    [#]     => { $crate::Pound };
    [$]     => { $crate::Dollar };
    [?]     => { $crate::Question };

    // --- keywords ---
    [as]          => { $crate::As };
    [async]       => { $crate::Async };
    [auto]        => { $crate::Auto };
    [await]       => { $crate::Await };
    [become]      => { $crate::Become };
    [box]         => { $crate::Box };
    [break]       => { $crate::Break };
    [const]       => { $crate::Const };
    [continue]    => { $crate::Continue };
    [crate]       => { $crate::Crate };
    [default]     => { $crate::Default };
    [do]          => { $crate::Do };
    [dyn]         => { $crate::Dyn };
    [else]        => { $crate::Else };
    [enum]        => { $crate::Enum };
    [extern]      => { $crate::Extern };
    [final]       => { $crate::Final };
    [fn]          => { $crate::Fn };
    [for]         => { $crate::For };
    [if]          => { $crate::If };
    [impl]        => { $crate::Impl };
    [in]          => { $crate::In };
    [let]         => { $crate::Let };
    [loop]        => { $crate::Loop };
    [macro]       => { $crate::Macro };
    [macro_rules] => { $crate::MacroRules };
    [match]       => { $crate::Match };
    [mod]         => { $crate::Mod };
    [move]        => { $crate::Move };
    [mut]         => { $crate::Mut };
    [override]    => { $crate::Override };
    [priv]        => { $crate::Priv };
    [pub]         => { $crate::Pub };
    [raw]         => { $crate::Raw };
    [ref]         => { $crate::Ref };
    [return]      => { $crate::Return };
    [safe]        => { $crate::Safe };
    [Self]        => { $crate::SelfType };
    [self]        => { $crate::SelfValue };
    [static]      => { $crate::Static };
    [struct]      => { $crate::Struct };
    [super]       => { $crate::Super };
    [trait]       => { $crate::Trait };
    [try]         => { $crate::Try };
    [type]        => { $crate::Type };
    [typeof]      => { $crate::Typeof };
    [union]       => { $crate::Union };
    [unsafe]      => { $crate::Unsafe };
    [unsized]     => { $crate::Unsized };
    [use]         => { $crate::Use };
    [virtual]     => { $crate::Virtual };
    [where]       => { $crate::Where };
    [while]       => { $crate::While };
    [yield]       => { $crate::Yield };
}
