use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Colon;
use moxy_token::{Parse, Punctuation, Span, Spanner, ToTokens, Token, TokenStream, TokenTree};

use crate::Lifetime;

#[doc = "A loop label (`'outer:`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Label {
    pub name: Lifetime,
    pub colon: Colon,
}

impl Parse for Label {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let name = stream.parse::<Lifetime>()?;
        let colon = stream.parse::<Colon>()?;
        Ok(Self { name, colon })
    }
}

impl Spanner for Label {
    fn span(&self) -> Span {
        self.name.span().join(self.colon.span())
    }
}

impl ToTokens for Label {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
        self.colon.to_tokens(tokens);
    }
}

impl Label {
    /// Returns `true` when the stream is positioned at a lifetime (`'a`) directly
    /// followed by `:`, which signals a loop/block label.
    pub fn is_prefix(stream: &mut ParseStream) -> bool {
        matches!(stream.curr(), Some(TokenTree::Token(Token::Punct(Punctuation::Quote(_)))))
            && matches!(stream.nth(2), Some(TokenTree::Token(Token::Punct(Punctuation::Colon(_)))))
    }
}
