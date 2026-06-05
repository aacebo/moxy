use moxy_token::keyword::Extern;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, ToTokens, Token, TokenStream, TokenTree};

#[doc = "An ABI string (`extern \"C\"`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Abi {
    pub span: Span,
    pub extern_keyword: Extern,
    pub name: Option<String>,
}

impl Parse for Abi {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let extern_keyword = stream.parse::<Extern>()?;

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
            extern_keyword,
            name,
        })
    }
}

impl ToTokens for Abi {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.extern_keyword.to_tokens(t);

        if let Some(name) = &self.name {
            moxy_token::Literal::string(name).to_tokens(t);
        }
    }
}
