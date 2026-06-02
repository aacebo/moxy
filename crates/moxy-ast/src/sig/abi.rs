use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::token::keyword::Extern;
use moxy_token::token::{self, ToTokens, Token, TokenTree};
use moxy_token::{Parse, Span, TokenStream};

#[doc = "An ABI string (`extern \"C\"`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Abi {
    pub span: Span,
    pub name: Option<String>,
}

impl Parse for Abi {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let _ = stream.parse::<Extern>()?;

        let name = match stream.curr() {
            Some(TokenTree::Token(Token::Literal(lit))) if lit.repr().starts_with('"') => {
                let repr = lit.repr().to_string();
                stream.advance();
                Some(repr.trim_matches('"').to_string())
            }
            _ => None,
        };

        Ok(Self {
            span: Span::default(),
            name,
        })
    }
}

impl ToTokens for Abi {
    fn to_tokens(&self, t: &mut TokenStream) {
        Extern::default().to_tokens(t);

        if let Some(name) = &self.name {
            token::Literal::string(name).to_tokens(t);
        }
    }
}
