use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream, TokenTree};

/// An ABI string (`extern "C"`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Abi {
    pub extern_keyword: Token![extern],
    pub name: Option<String>,
}

impl Parse for Abi {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let extern_keyword = stream.parse::<Token![extern]>()?;
        let name = match stream.curr() {
            Some(TokenTree::Literal(lit)) if lit.repr().starts_with('"') => {
                let repr = lit.repr().to_string();
                stream.advance();
                Some(repr.trim_matches('"').to_string())
            }
            _ => None,
        };

        Ok(Self { extern_keyword, name })
    }
}

impl Spanner for Abi {
    fn span(&self) -> Span {
        self.extern_keyword.span()
    }
}

impl ToTokens for Abi {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.extern_keyword.to_tokens(t);

        if let Some(name) = &self.name {
            moxy_token::Lit::string(name).to_tokens(t);
        }
    }
}
