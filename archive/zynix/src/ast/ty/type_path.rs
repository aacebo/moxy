use super::QSelf;
use crate::ast::{Path, PathSegment};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::As;
use crate::token::punct::{Gt, Lt, PathSep};
use crate::{Parse, Span, TokenStream};

#[doc = "A path type (e.g. `T`, `std::vec::Vec`, `<T as Trait>::Item`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypePath {
    pub span: Span,
    pub qself: Option<QSelf>,
    pub path: Path,
}

impl Parse for TypePath {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<Lt>().is_some() {
            let (qself, path) = super::QSelf::parse_qualified(stream)?;
            return Ok(Self {
                span: Span::default(),
                qself: Some(qself),
                path,
            });
        }

        Ok(Self {
            span: Span::default(),
            qself: None,
            path: stream.parse()?,
        })
    }
}

impl TypePath {
    pub fn emit_segments(segs: &[&PathSegment], tokens: &mut TokenStream) {
        for (i, seg) in segs.iter().enumerate() {
            if i > 0 {
                PathSep::default().to_tokens(tokens);
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
                Lt::default().to_tokens(tokens);
                qself.ty.to_tokens(tokens);

                let segs: Vec<&PathSegment> = self.path.segments.iter().collect();

                if qself.position > 0 {
                    As::default().to_tokens(tokens);
                    TypePath::emit_segments(&segs[..qself.position], tokens);
                }

                Gt::default().to_tokens(tokens);

                for seg in &segs[qself.position..] {
                    PathSep::default().to_tokens(tokens);
                    seg.to_tokens(tokens);
                }
            }
        }
    }
}
