use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::ty::TypePath;
use crate::*;

/// A struct pattern, e.g. `Point { x, y }` or `Point { x, .. }`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
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
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = if <Token![<]>::peek(cursor) {
            let Some(cursor) = TypePath::skip(cursor) else {
                return false;
            };

            cursor
        } else {
            let Some(cursor) = Path::skip(cursor) else {
                return false;
            };

            cursor
        };

        cursor.is_delimited(moxy_token::Delim::Brace)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let (qself, path) = if <Token![<]>::peek(parser.cursor()) {
            let (qself, path) = QSelf::parse_qualified(parser)?;
            (Some(qself), path)
        } else {
            (None, <_ as Parse>::parse(parser)?)
        };

        let body = Delimited::parse_brace(parser)?;
        Ok(Self {
            attrs,
            qself,
            path,
            body,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut cursor = Attributes::skip(cursor)?;
        cursor = if <Token![<]>::peek(cursor) {
            TypePath::skip(cursor)?
        } else {
            Path::skip(cursor)?
        };

        let mut inner = cursor.descend(moxy_token::Delim::Brace)?;
        inner = PatStructBody::skip(inner)?;
        inner.is_empty().then(|| cursor.offset(1))
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

/// The AST node for a Rust struct body pattern.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
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
        cursor.is_empty() || pat::PatField::peek(cursor) || <Token![..]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let mut fields = Punctuated::new();
        let mut dotdot = None;

        while !parser.is_empty() {
            if <Token![..]>::peek(parser.cursor()) {
                dotdot = Some(<_ as Parse>::parse(parser)?);
                break;
            }

            fields.push_value(<_ as Parse>::parse(parser)?);

            if <Token![,]>::peek(parser.cursor()) {
                fields.push_punct(<_ as Parse>::parse(parser)?);
            } else {
                break;
            }
        }

        Ok(Self { fields, dotdot })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        while !cursor.is_empty() {
            if <Token![..]>::peek(cursor) {
                return <Token![..]>::skip(cursor);
            }

            cursor = pat::PatField::skip(cursor)?;

            if <Token![,]>::peek(cursor) {
                cursor = <Token![,]>::skip(cursor)?;
            } else {
                break;
            }
        }

        Some(cursor)
    }
}

impl ToTokens for PatStructBody {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.fields.to_tokens(t);
        self.dotdot.to_tokens(t);
    }
}
