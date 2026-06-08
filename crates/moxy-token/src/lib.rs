#![cfg_attr(
    nightly,
    feature(
        extend_one,
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
mod literal;
pub mod parser;
pub mod punct;
pub mod source;
mod spacing;
pub mod span;
mod stream;
mod token_tree;

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
pub use literal::*;
#[doc(inline)]
pub use parser::Parse;
#[doc(inline)]
pub use punct::*;
#[doc(inline)]
pub use spacing::*;
#[doc(inline)]
pub use span::{Span, Spanner};
#[doc(inline)]
pub use stream::*;
#[doc(inline)]
pub use token_tree::*;

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
    use super::*;
    use crate::source::SourceMap;
    use crate::span::fallback as span_fb;

    fn span(start: u32, end: u32) -> Span {
        SourceMap::with_mut(|sm| {
            if sm.is_empty() {
                sm.push("0123456789abcdef");
            }
        });
        Span::Fallback(span_fb::Span::new(start, end))
    }

    // --- Ident ---

    #[test]
    fn ident_new_and_name() {
        let id = Ident::new("foo", Span::default());
        assert_eq!(id.text(), "foo");
    }

    #[test]
    fn ident_span_and_set_span() {
        let mut id = Ident::new("x", span(0, 1));
        assert_eq!(id.span().start().index(), 0);
        id.set_span(span(5, 6));
        assert_eq!(id.span().start().index(), 5);
    }

    #[test]
    fn ident_display() {
        let id = Ident::new("hello", Span::default());
        assert_eq!(format!("{}", id), "hello");
    }

    // --- Punct (operators) ---

    #[test]
    fn op_as_str() {
        use crate::punct::{Plus, Semi};
        assert_eq!(Plus::default().as_str(), "+");
        assert_eq!(Semi::default().as_str(), ";");
    }

    #[test]
    fn op_display() {
        use crate::punct::{EqEq, Semi};
        assert_eq!(format!("{}", Semi::default()), ";");
        assert_eq!(format!("{}", EqEq::default()), "==");
    }

    #[test]
    fn op_is_a_token() {
        use crate::punct::Plus;
        let t: TokenTree = Punctuation::from(Plus::default()).into();
        assert!(matches!(t, TokenTree::Punct(Punctuation::Plus(_))));
    }

    // --- Literal ---

    #[test]
    fn literal_string() {
        let lit = Literal::string("hello");
        let s = format!("{}", lit);
        assert!(s.contains("hello"));
    }

    #[test]
    fn literal_integer() {
        let lit = Literal::u32_suffixed(42);
        let s = format!("{}", lit);
        assert!(s.contains("42"));
    }

    // --- Group ---

    #[test]
    fn group_new_and_delim() {
        let g = Group::new(Delim::Paren, TokenStream::new());
        assert_eq!(g.delim(), Delim::Paren);
    }

    // --- TokenStream ---

    #[test]
    fn token_stream_new_is_empty() {
        let ts = TokenStream::new();
        assert!(ts.is_empty());
    }

    #[test]
    fn token_stream_extend_one() {
        let mut ts = TokenStream::new();
        ts.extend_one(Ident::new("a", Span::default()).into());
        assert_eq!(ts.len(), 1);
    }

    #[test]
    fn token_stream_iter() {
        let mut ts = TokenStream::new();
        ts.extend_one(Ident::new("x", Span::default()).into());
        ts.extend_one(Punctuation::from(crate::punct::Plus::default()).into());
        let count = ts.iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn token_stream_from_str() {
        use std::str::FromStr;
        let ts = TokenStream::from_str("fn main() {}").unwrap();
        assert!(!ts.is_empty());
    }

    // --- TokenTree ---

    #[test]
    fn token_from_ident() {
        let t: TokenTree = Ident::new("foo", Span::default()).into();
        assert!(matches!(t, TokenTree::Ident(_)));
    }

    #[test]
    fn token_from_punct() {
        let t: TokenTree = Punctuation::from(crate::punct::Plus::default()).into();
        assert!(matches!(t, TokenTree::Punct(_)));
    }

    #[test]
    fn token_from_literal() {
        let t: TokenTree = Literal::string("x").into();
        assert!(matches!(t, TokenTree::Literal(_)));
    }

    #[test]
    fn token_tree_from_group() {
        let t: TokenTree = Group::new(Delim::Paren, TokenStream::new()).into();
        assert!(matches!(t, TokenTree::Group(_)));
    }

    #[test]
    fn token_span() {
        let t: TokenTree = Ident::new("x", span(3, 4)).into();
        assert_eq!(t.span().start().index(), 3);
    }

    #[test]
    fn token_display() {
        let t: TokenTree = Ident::new("hello", Span::default()).into();
        assert_eq!(format!("{}", t), "hello");
    }

    // --- scan_puncts_spanned: multi-char assembly + span preservation ---

    fn puncts(run: &[(char, Span)]) -> TokenStream {
        let mut ts = TokenStream::new();
        crate::token_tree::scan_puncts_spanned(run, &mut ts);
        ts
    }

    #[test]
    fn punct_run_longest_match() {
        let ts = puncts(&[('=', span(0, 1)), ('>', span(1, 2))]);
        let trees: Vec<_> = ts.iter().cloned().collect();
        assert_eq!(trees.len(), 1);
        assert!(matches!(trees[0], TokenTree::Punct(Punctuation::FatArrow(_))));
    }

    #[test]
    fn punct_run_preserves_span() {
        let ts = puncts(&[(':', span(4, 5)), (':', span(5, 6))]);
        let trees: Vec<_> = ts.iter().cloned().collect();
        assert_eq!(trees.len(), 1);
        let TokenTree::Punct(p) = &trees[0] else {
            panic!("expected punct")
        };
        assert!(matches!(p, Punctuation::PathSep(_)));
        assert_eq!(p.span().start().index(), 4);
        assert_eq!(p.span().end().index(), 6);
    }

    #[test]
    fn punct_run_splits_multiple() {
        let ts = puncts(&[(',', span(0, 1)), (';', span(1, 2))]);
        let trees: Vec<_> = ts.iter().cloned().collect();
        assert_eq!(trees.len(), 2);
        assert!(matches!(trees[0], TokenTree::Punct(Punctuation::Comma(_))));
        assert!(matches!(trees[1], TokenTree::Punct(Punctuation::Semi(_))));
    }

    // --- Token! macro ---

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

    #[cfg(feature = "serde")]
    mod serde {
        use std::str::FromStr;

        use crate::TokenStream;

        #[test]
        fn token_serializes_as_string() {
            let ts = TokenStream::from_str("foo").unwrap();
            let tree = ts.into_iter().next().unwrap();
            assert_eq!(serde_json::to_value(&tree).unwrap(), serde_json::json!("foo"));
        }

        #[test]
        fn token_tree_group_serializes_as_object() {
            let ts = TokenStream::from_str("(x)").unwrap();
            let tree = ts.into_iter().next().unwrap();
            assert_eq!(
                serde_json::to_value(&tree).unwrap(),
                serde_json::json!({ "delim": "paren", "tokens": ["x"] })
            );
        }
    }
}
