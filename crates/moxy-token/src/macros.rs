//! The [`Token!`] macro: maps a punctuation/keyword symbol to its token type,
//! mirroring syn's `Token![...]`.
//!
//! `Token![=>]` resolves to the type [`crate::FatArrow`]; construct a value with
//! the type's `::new(span)` / `::default()`. The arms here are the single flat
//! artifact (as in syn); keep them in sync with the `define_punct!` table in
//! [`crate::punct`] and the `define_keyword!` table in [`crate::keyword`].

/// Map a Rust punctuation or keyword symbol to its [`crate`] token type.
///
/// ```
/// use moxy_token::{Token, Span, ToTokens, TokenStream};
///
/// let arrow = <Token![=>]>::new(Span::call_site());
/// let mut ts = TokenStream::new();
/// arrow.to_tokens(&mut ts);
/// assert_eq!(ts.to_string(), "=>");
/// ```
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
    [?]     => { $crate::Question };

    // --- punctuation: multi char ---
    [&&]    => { $crate::AndAnd };
    [||]    => { $crate::OrOr };
    [<<]    => { $crate::Shl };
    [>>]    => { $crate::Shr };
    [==]    => { $crate::EqEq };
    [!=]    => { $crate::Ne };
    [<=]    => { $crate::Le };
    [>=]    => { $crate::Ge };
    [&=]    => { $crate::AndEq };
    [|=]    => { $crate::OrEq };
    [+=]    => { $crate::PlusEq };
    [-=]    => { $crate::MinusEq };
    [*=]    => { $crate::StarEq };
    [/=]    => { $crate::SlashEq };
    [%=]    => { $crate::PercentEq };
    [^=]    => { $crate::CaretEq };
    [=>]    => { $crate::FatArrow };
    [->]    => { $crate::RArrow };
    [<-]    => { $crate::LArrow };
    [::]    => { $crate::PathSep };
    [..]    => { $crate::DotDot };
    [<<=]   => { $crate::ShlEq };
    [>>=]   => { $crate::ShrEq };
    [...]   => { $crate::DotDotDot };
    [..=]   => { $crate::DotDotEq };

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

#[cfg(test)]
mod tests {
    use crate::{Span, ToTokens, TokenStream};

    fn render<T: ToTokens>(tok: T) -> String {
        let mut ts = TokenStream::new();
        tok.to_tokens(&mut ts);
        ts.to_string()
    }

    #[test]
    fn punct_single() {
        assert_eq!(render(<Token![&]>::new(Span::call_site())), "&");
        assert_eq!(render(<Token![,]>::new(Span::call_site())), ",");
    }

    #[test]
    fn punct_multi() {
        assert_eq!(render(<Token![=>]>::new(Span::call_site())), "=>");
        assert_eq!(render(<Token![::]>::new(Span::call_site())), "::");
    }

    #[test]
    fn keyword() {
        assert_eq!(render(<Token![for]>::new(Span::call_site())), "for");
        assert_eq!(render(<Token![match]>::default()), "match");
    }

    #[test]
    fn resolves_to_type() {
        let _: Token![=>] = crate::FatArrow::default();
        let _: Token![for] = crate::For::default();
    }
}
