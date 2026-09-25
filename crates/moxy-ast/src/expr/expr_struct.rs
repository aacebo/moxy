use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A struct literal expression: `Foo { a: 1, b, ..rest }`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprStruct {
    pub attrs: Attributes,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub body: Delimited<StructBody>,
}

impl From<ExprStruct> for Expr {
    fn from(value: ExprStruct) -> Self {
        Self::Struct(value)
    }
}

impl Spanner for ExprStruct {
    fn span(&self) -> Span {
        self.attrs.span().join(self.body.span())
    }
}

impl ToTokens for ExprStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.path.to_tokens(t);
        self.body.to_tokens(t);
    }
}

/// An AST representation of Rust struct body syntax.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StructBody {
    pub fields: Punctuated<FieldValue, Token![,]>,
    pub rest: Option<(Token![..], Box<Expr>)>,
}

impl Parse for StructBody {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.is_empty() || FieldValue::peek(cursor) || <Token![..]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let mut fields = Punctuated::new();
        let mut rest = None;

        while !parser.is_empty() {
            if <Token![..]>::peek(parser.cursor()) {
                rest = Some((<_ as Parse>::parse(parser)?, <_ as Parse>::parse(parser)?));
                break;
            }

            fields.push_value(<_ as Parse>::parse(parser)?);

            if <Token![,]>::peek(parser.cursor()) {
                fields.push_punct(<_ as Parse>::parse(parser)?);
            } else {
                break;
            }
        }

        Ok(Self { fields, rest })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        while !cursor.is_empty() {
            if <Token![..]>::peek(cursor) {
                cursor = <Token![..]>::skip(cursor)?;
                return Expr::skip(cursor);
            }

            cursor = FieldValue::skip(cursor)?;

            if <Token![,]>::peek(cursor) {
                cursor = <Token![,]>::skip(cursor)?;
            } else {
                break;
            }
        }

        Some(cursor)
    }
}

impl ToTokens for StructBody {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.fields.to_tokens(t);

        if let Some((dotdot, rest)) = &self.rest {
            dotdot.to_tokens(t);
            rest.to_tokens(t);
        }
    }
}
