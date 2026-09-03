use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use super::QSelf;
use crate::{Path, PathSegment};

/// A path type (e.g. `T`, `std::vec::Vec`, `<T as Trait>::Item`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypePath {
    pub qself: Option<QSelf>,
    pub path: Path,
}

impl Parse for TypePath {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.peek::<Token![<]>() {
            let (qself, path) = super::QSelf::parse_qualified(parser)?;

            return Ok(Self {
                qself: Some(qself),
                path,
            });
        }

        Ok(Self {
            qself: None,
            path: parser.parse()?,
        })
    }
}

impl Spanner for TypePath {
    fn span(&self) -> Span {
        if let Some(q) = &self.qself {
            q.span().join(self.path.span())
        } else {
            self.path.span()
        }
    }
}

impl TypePath {
    pub fn emit_segments(segs: &[&PathSegment], tokens: &mut TokenStream) {
        for (i, seg) in segs.iter().enumerate() {
            if i > 0 {
                <Token![::]>::default().to_tokens(tokens);
            }

            seg.to_tokens(tokens);
        }
    }
}

impl ToTokens for TypePath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.qself {
            None => self.path.to_tokens(tokens),
            Some(qself) => {
                // `< ty (as Trait)? > :: rest`, where `position` segments of the
                // path belong to the trait inside the angle brackets.
                <Token![<]>::default().to_tokens(tokens);
                qself.ty.to_tokens(tokens);

                let segs: Vec<&PathSegment> = self.path.iter().collect();

                if qself.position > 0 {
                    <Token![as]>::default().to_tokens(tokens);
                    Self::emit_segments(&segs[..qself.position], tokens);
                }

                <Token![>]>::default().to_tokens(tokens);

                for seg in &segs[qself.position..] {
                    <Token![::]>::default().to_tokens(tokens);
                    seg.to_tokens(tokens);
                }
            }
        }
    }
}
