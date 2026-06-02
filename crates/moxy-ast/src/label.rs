use moxy_macros::{Parse, ToTokens};
use moxy_token::parse::ParseStream;
use moxy_token::punct::Colon;
use moxy_token::{Punctuation, Span, Token, TokenTree};

use crate::Lifetime;

#[doc = "A loop label (`'outer:`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Label {
    #[parse(skip)]
    pub span: Span,

    #[parse(suffix = Colon)]
    pub name: Lifetime,
}

impl Label {
    /// Returns `true` when the stream is positioned at a lifetime (`'a`) directly
    /// followed by `:`, which signals a loop/block label.
    pub fn is_prefix(stream: &mut ParseStream) -> bool {
        matches!(stream.curr(), Some(TokenTree::Token(Token::Punct(Punctuation::Quote(_)))))
            && matches!(stream.nth(2), Some(TokenTree::Token(Token::Punct(Punctuation::Colon(_)))))
    }
}
