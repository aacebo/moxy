use crate::span::DelimSpan;
use crate::token::Delim;
use crate::token::lex::{Cursor, LexError, Scan};
use crate::{Span, TokenStream};

#[derive(Debug, Clone)]
pub struct Group {
    pub(crate) delim: Delim,
    pub(crate) span: DelimSpan,
    pub(crate) tokens: TokenStream,
}

impl Group {
    pub fn new(delim: Delim, stream: TokenStream) -> Self {
        Self {
            delim,
            span: DelimSpan::new(Span::call_site(), Span::call_site()),
            tokens: stream,
        }
    }

    pub fn delim(&self) -> Delim {
        self.delim
    }

    pub fn span(&self) -> DelimSpan {
        self.span
    }

    pub fn stream(&self) -> TokenStream {
        self.tokens.clone()
    }

    pub fn set_span(&mut self, span: DelimSpan) {
        self.span = span;
    }
}

impl From<proc_macro::Group> for Group {
    fn from(value: proc_macro::Group) -> Self {
        let mut group = Self::new(value.delimiter().into(), value.stream().into());
        group.set_span(DelimSpan::new(value.span_open().into(), value.span_close().into()));
        group
    }
}

impl From<Group> for proc_macro::Group {
    fn from(value: Group) -> Self {
        proc_macro::Group::new(value.delim.into(), value.tokens.into())
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.delim {
            Delim::None => write!(f, "{}", self.tokens),
            d => write!(f, "{}{}{}", d.open(), self.tokens, d.close()),
        }
    }
}

impl Scan for Group {
    fn scan(cursor: Cursor<'_>) -> Result<(Cursor<'_>, Self), LexError> {
        let ch = cursor.first().ok_or(cursor.error())?;
        let delim = Delim::from_open(ch).ok_or(cursor.error())?;
        let c = cursor.advance(ch.len_utf8());
        let (c, inner) = TokenStream::scan(c)?;
        let close_ch = c
            .first()
            .ok_or_else(|| cursor.error().message(format!("unclosed delimiter '{}'", delim.open())))?;

        let close_delim = Delim::from_close(close_ch).ok_or_else(|| {
            c.error()
                .message(format!("expected '{}', found '{}'", delim.close(), close_ch))
        })?;

        if delim != close_delim {
            return Err(c.error().message(format!(
                "mismatched delimiter: expected '{}', found '{}'",
                delim.close(),
                close_ch,
            )));
        }

        let c = c.advance(close_ch.len_utf8());
        let mut group = Self::new(delim, inner);
        group.set_span(DelimSpan::new(cursor.span(), c.span()));

        Ok((c, group))
    }
}

impl crate::token::ToTokens for Group {
    fn to_tokens(&self, tokens: &mut crate::TokenStream) {
        tokens.extend_one(crate::TokenTree::Group(self.clone()));
    }
}

impl crate::Parse for Group {
    fn parse(stream: &mut crate::parse::ParseStream) -> Result<Self, crate::parse::ParseError> {
        match stream.advance() {
            Some(crate::TokenTree::Group(v)) => Ok(v.clone()),
            _ => Err(crate::token::lex::LexError::new(stream.span())
                .message("expected Group")
                .into()),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Group {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut o = s.serialize_struct("Group", 2)?;
        o.serialize_field("delim", &self.delim)?;
        o.serialize_field("tokens", &self.tokens)?;
        o.end()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    mod serde {
        use std::str::FromStr;

        use crate::TokenStream;
        use crate::token::TokenTree;

        #[test]
        fn group_serializes_as_delim_and_tokens() {
            let ts = TokenStream::from_str("[a, b]").unwrap();
            let tree = ts.into_iter().next().unwrap();
            let TokenTree::Group(g) = tree else {
                panic!("expected group");
            };

            assert_eq!(
                serde_json::to_value(&g).unwrap(),
                serde_json::json!({ "delim": "bracket", "tokens": ["a", ",", "b"] })
            );
        }
    }
}
