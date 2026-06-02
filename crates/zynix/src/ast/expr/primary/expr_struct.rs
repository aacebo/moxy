use crate::ast::*;
use crate::token::punct::{Comma, DotDot};
use crate::token::{Delim, Group, ToTokens};
use crate::{Span, TokenStream, TokenTree};

#[doc = "A struct literal expression: `Foo { a: 1, b, ..rest }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprStruct {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub fields: Punctuated<FieldValue, Comma>,
    pub rest: Option<Box<super::super::Expr>>,
}

impl ToTokens for ExprStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.path.to_tokens(t);
        let mut inner = TokenStream::new();
        self.fields.to_tokens(&mut inner);

        if let Some(rest) = &self.rest {
            DotDot::default().to_tokens(&mut inner);
            rest.to_tokens(&mut inner);
        }

        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));
    }
}
