#![cfg_attr(
    nightly,
    feature(
        proc_macro_diagnostic,
        proc_macro_span,
        // proc_macro_totokens,
        proc_macro_def_site,
    )
)]

extern crate proc_macro;

pub mod bridge;
mod delim;
mod group;
mod ident;
pub mod keyword;
pub mod lex;
mod lit;
pub mod punct;
pub mod source;
mod spacing;
pub mod span;
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
pub use lex::{LexError, Scan};
#[doc(inline)]
pub use lit::*;
#[doc(inline)]
pub use punct::*;
#[doc(inline)]
pub use spacing::*;
#[doc(inline)]
pub use span::{Span, Spanner};
#[doc(inline)]
pub use stream::*;
#[doc(inline)]
pub use tree::*;

pub trait ToTokens<T = TokenStream> {
    fn to_tokens(&self, tokens: &mut T);
}

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
