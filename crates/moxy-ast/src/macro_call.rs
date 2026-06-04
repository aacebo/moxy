use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Not;
use moxy_token::{Delim, Group, LexError, Parse, Span, ToTokens, TokenStream, TokenTree};

use crate::{Attribute, Path};

#[doc = "A macro invocation (`path!(...)`, `path![...]`, `path!{...}`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MacroCall {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub path: Path,
    pub bang: Not,
    pub body: Group,
}

impl MacroCall {
    pub fn parse_semi(stream: &mut ParseStream) -> Result<(Self, Option<moxy_token::punct::Semi>), ParseError> {
        use moxy_token::punct::Semi;
        let mac = stream.parse::<MacroCall>()?;
        let semi = stream.parse_if::<Semi>();
        Ok((mac, semi))
    }

    /// The delimiter of the macro body (`(`, `[`, or `{`).
    pub fn delim(&self) -> Delim {
        self.body.delim()
    }

    /// The token stream inside the macro body delimiters.
    pub fn tokens(&self) -> TokenStream {
        self.body.stream()
    }
}

impl Parse for MacroCall {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let path = stream.parse::<Path>()?;
        let bang = stream.parse::<Not>()?;

        let body = match stream.curr() {
            Some(TokenTree::Group(g)) => {
                let g = g.clone();
                stream.advance();
                g
            }
            _ => {
                return Err(LexError::new(stream.span()).message("expected macro delimiter").into());
            }
        };

        Ok(Self {
            span: Span::default(),
            attrs,
            path,
            bang,
            body,
        })
    }
}

impl ToTokens for MacroCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
        self.path.to_tokens(tokens);
        self.bang.to_tokens(tokens);
        tokens.extend_one(TokenTree::Group(self.body.clone()));
    }
}
