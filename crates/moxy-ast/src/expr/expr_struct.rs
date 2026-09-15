use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A struct literal expression: `Foo { a: 1, b, ..rest }`.
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StructBody {
    pub fields: Punctuated<FieldValue, Token![,]>,
    pub rest: Option<(Token![..], Box<Expr>)>,
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
