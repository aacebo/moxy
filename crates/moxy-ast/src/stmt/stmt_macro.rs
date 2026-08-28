use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, MacroCall};

/// A macro invocation used as a statement (`name!(...);` or `name!(...)`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtMacro {
    pub attrs: Attributes,
    pub mac: MacroCall,
    pub semi: Option<Token![;]>,
}

impl Parse for StmtMacro {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let mac = stream.parse::<MacroCall>()?;
        let semi = stream.parse_if::<Token![;]>();
        Ok(Self { attrs, mac, semi })
    }
}

impl Spanner for StmtMacro {
    fn span(&self) -> Span {
        let end = self.semi.as_ref().map(|s| s.span()).unwrap_or_else(|| self.mac.span());
        self.attrs.span().join(end)
    }
}

impl ToTokens for StmtMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.mac.to_tokens(t);
        self.semi.to_tokens(t);
    }
}

impl StmtMacro {
    pub fn into_stmt(self) -> super::Stmt {
        super::Stmt::Macro(self)
    }
}
