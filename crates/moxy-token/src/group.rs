use crate::lex::{Cursor, LexError, Scan};
use crate::span::DelimSpan;
use crate::{Delim, Span, TokenStream, TokenTree};

/// A delimited sequence of token trees.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub delim: Delim,
    pub span: DelimSpan,
    pub tokens: TokenStream,
}

impl Group {
    #[inline]
    pub fn new(delim: Delim, stream: TokenStream) -> Self {
        // A group built in code has no delimiters in a source. Around tokens
        // with source positions it opens at the first and closes at the last;
        // around tokens with compiler spans, which have no position here, it
        // takes the call site, as proc_macro's groups do, so the compiler
        // treats it as macro output
        let span = match (stream.first(), stream.last()) {
            (Some(first), Some(last)) if !stream.span().is_empty() => DelimSpan::new(first.span(), last.span()),
            _ => DelimSpan::new(Span::call_site(), Span::call_site()),
        };

        Self {
            delim,
            span,
            tokens: stream,
        }
    }

    #[inline]
    pub fn to_token_tree(&self) -> TokenTree {
        TokenTree::Group(self.clone())
    }

    #[inline]
    pub fn into_token_tree(self) -> TokenTree {
        TokenTree::Group(self)
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
        let c = cursor.advance_by(ch.len_utf8());
        let depth = crate::lex::depth::Depth::enter().ok_or_else(|| LexError::too_deep(cursor.error().span()))?;
        let (c, inner) = TokenStream::scan(c)?;
        drop(depth);
        let close_ch = c
            .first()
            .ok_or_else(|| cursor.error().message(format!("unclosed delimiter `{}`", delim.open())))?;

        let close_delim = Delim::from_close(close_ch).ok_or_else(|| {
            c.error()
                .message(format!("expected `{}`, found `{}`", delim.close(), close_ch))
        })?;

        if delim != close_delim {
            return Err(c
                .error()
                .message(format!("expected `{}`, found `{}`", delim.close(), close_ch,)));
        }

        let close = c.span();
        let c = c.advance_by(close_ch.len_utf8());
        let mut group = Self::new(delim, inner);
        group.span = DelimSpan::new(cursor.span(), close);

        Ok((c, group))
    }
}

impl crate::ToTokens for Group {
    fn to_tokens(&self, tokens: &mut crate::TokenStream) {
        tokens.extend_one(crate::TokenTree::Group(self.clone()));
    }
}

impl crate::Spanner for Group {
    fn span(&self) -> Span {
        self.span.span()
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
