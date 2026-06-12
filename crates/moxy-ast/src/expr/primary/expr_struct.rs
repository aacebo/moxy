use moxy_token::punct::{Comma, DotDot};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StructBody {
    pub fields: Punctuated<FieldValue, Comma>,
    pub rest: Option<(DotDot, Box<Expr>)>,
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

/// A struct literal expression: `Foo { a: 1, b, ..rest }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprStruct {
    pub attrs: Attributes,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub body: Delimited<StructBody>,
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

impl ExprStruct {
    pub fn into_primary_expr(self) -> super::PrimaryExpr {
        super::PrimaryExpr::from(self)
    }
}
