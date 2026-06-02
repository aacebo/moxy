use crate::ast::{Attribute, Path};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::Not;
use crate::token::{Delim, Group, LexError, ToTokens};
use crate::{Parse, Span, TokenStream, TokenTree};

#[doc = "A macro invocation (`path!(...)`, `path![...]`, `path!{...}`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MacroCall {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub path: Path,
    pub delim: Delim,
    pub tokens: TokenStream,
}

impl MacroCall {
    pub fn parse_semi(stream: &mut ParseStream) -> Result<(Self, bool), ParseError> {
        use crate::token::punct::Semi;
        let mac = stream.parse::<MacroCall>()?;

        let semi = if stream.peek::<Semi>().is_some() {
            let _ = stream.parse::<Semi>()?;
            true
        } else {
            false
        };

        Ok((mac, semi))
    }
}

impl Parse for MacroCall {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec()?;
        let path = stream.parse::<Path>()?;
        let _ = stream.parse::<Not>()?;

        let (delim, tokens) = match stream.curr() {
            Some(TokenTree::Group(g)) => {
                let delim = g.delim;
                let tokens = g.stream();
                stream.advance();
                (delim, tokens)
            }
            _ => {
                return Err(LexError::new(stream.span()).message("expected macro delimiter").into());
            }
        };

        Ok(Self {
            span: Span::default(),
            attrs,
            path,
            delim,
            tokens,
        })
    }
}

impl ToTokens for MacroCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
        self.path.to_tokens(tokens);
        Not::default().to_tokens(tokens);
        tokens.extend_one(TokenTree::Group(Group::new(self.delim, self.tokens.clone())));
    }
}
