use moxy_token::parser::ParseError;
use moxy_token::{Eq, EqEq, FatArrow, Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Expr, Meta, Path};

/// A name-value meta item (`name = expr`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MetaNameValue {
    pub path: Path,
    pub eq: Eq,
    pub value: Expr,
}

impl MetaNameValue {
    pub fn into_meta(self) -> Meta {
        Meta::NameValue(self)
    }
}

impl Spanner for MetaNameValue {
    fn span(&self) -> Span {
        self.path.span().join(self.value.span())
    }
}

impl ToTokens for MetaNameValue {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        self.eq.to_tokens(t);
        self.value.to_tokens(t);
    }
}

impl Parse for MetaNameValue {
    fn parse(stream: &mut moxy_token::parser::ParseStream) -> Result<Self, moxy_token::parser::ParseError> {
        let path = stream.parse::<Path>()?;

        if !stream.peek::<Eq>() || stream.peek::<EqEq>() || stream.peek::<FatArrow>() {
            return Err(ParseError::new(path.span(), "expected \"=\""));
        }

        Ok(Self {
            path,
            eq: stream.parse()?,
            value: stream.parse()?,
        })
    }
}
