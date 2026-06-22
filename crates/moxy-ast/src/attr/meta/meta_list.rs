use moxy_token::parser::ParseError;
use moxy_token::{Comma, Group, Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Delimited, Meta, Path, Punctuated};

/// A list-style meta item (`name(tokens)`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MetaList {
    pub path: Path,
    pub items: Delimited<Punctuated<Meta, Comma>>,
}

impl MetaList {
    pub fn into_meta(self) -> Meta {
        Meta::List(self)
    }
}

impl std::ops::Deref for MetaList {
    type Target = Delimited<Punctuated<Meta, Comma>>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl std::ops::DerefMut for MetaList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl Spanner for MetaList {
    fn span(&self) -> Span {
        self.path.span().join(self.items.span())
    }
}

impl ToTokens for MetaList {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.path.to_tokens(t);
        self.items.to_tokens(t);
    }
}

impl Parse for MetaList {
    fn parse(stream: &mut moxy_token::parser::ParseStream) -> Result<Self, moxy_token::parser::ParseError> {
        let path = stream.parse::<Path>()?;

        if !stream.peek::<Group>() {
            return Err(ParseError::new(path.span(), "expected \"(...)\""));
        }

        Ok(Self {
            path,
            items: Delimited::parse_paren_with(stream, Punctuated::parse_terminated)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use moxy_token::parse;

    use crate::Attribute;

    #[test]
    fn mutliple() {
        let v = parse!("#[cfg(a, b, c)]" as Attribute).unwrap();
        assert!(v.meta.is_list());
        assert_eq!(v.meta.as_list().unwrap().len(), 3);
    }
}
