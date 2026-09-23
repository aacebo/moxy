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
        cursor.is_empty() || cursor.peek::<FieldValue>() || cursor.peek::<Token![..]>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let mut fields = Punctuated::new();
        let mut rest = None;

        while !parser.is_empty() {
            if parser.peek::<Token![..]>() {
                rest = Some((parser.parse()?, parser.parse()?));
                break;
            }

            fields.push_value(parser.parse()?);

            if parser.peek::<Token![,]>() {
                fields.push_punct(parser.parse()?);
            } else {
                break;
            }
        }

        Ok(Self { fields, rest })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        while !cursor.is_empty() {
            if cursor.peek::<Token![..]>() {
                cursor = cursor.skip::<Token![..]>()?;
                return cursor.skip::<Expr>();
            }

            cursor = cursor.skip::<FieldValue>()?;

            if cursor.peek::<Token![,]>() {
                cursor = cursor.skip::<Token![,]>()?;
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
