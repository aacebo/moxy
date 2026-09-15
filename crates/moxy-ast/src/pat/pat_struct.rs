use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A struct pattern, e.g. `Point { x, y }` or `Point { x, .. }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatStruct {
    pub attrs: Attributes,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub body: Delimited<PatStructBody>,
}

impl Spanner for PatStruct {
    fn span(&self) -> Span {
        self.attrs.span().join(self.body.span())
    }
}

impl Parse for PatStruct {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<QSelf>() || cursor.peek::<Path>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            attrs: parser.parse()?,
            qself: parser.parse()?,
            path: parser.parse()?,
            body: parser.parse()?,
        })
    }
}

impl ToTokens for PatStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.qself.to_tokens(t);
        self.path.to_tokens(t);
        self.body.to_tokens(t);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatStructBody {
    pub fields: Punctuated<pat::PatField, Token![,]>,
    pub dotdot: Option<Token![..]>,
}

impl Spanner for PatStructBody {
    fn span(&self) -> Span {
        if let Some(dotdot) = &self.dotdot {
            self.fields.span().join(dotdot.span())
        } else {
            self.fields.span()
        }
    }
}

impl Parse for PatStructBody {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<pat::PatField>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            fields: Punctuated::parse_separated_nonempty(parser)?,
            dotdot: parser.parse()?,
        })
    }
}

impl ToTokens for PatStructBody {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.fields.to_tokens(t);
        self.dotdot.to_tokens(t);
    }
}
