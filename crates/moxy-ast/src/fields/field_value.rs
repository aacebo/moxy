use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::punct::Colon;
use moxy_token::{Parse, Span, ToTokens, TokenStream};

use crate::expr::{ExprPath, PrimaryExpr};
use crate::{Attribute, Expr, Member};

#[doc = "A struct literal field (`member: expr` or shorthand `member`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldValue {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub member: Member,
    pub colon_punct: Option<Colon>,
    pub expr: Expr,
    pub shorthand: bool,
}

impl Parse for FieldValue {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let member = stream.parse::<Member>()?;
        if stream.peek::<Colon>().is_some() {
            let colon_punct = Some(stream.parse::<Colon>()?);
            let expr = stream.parse::<Expr>()?;
            Ok(Self {
                span: Span::default(),
                attrs,
                member,
                colon_punct,
                expr,
                shorthand: false,
            })
        } else {
            let expr = match &member {
                Member::Named(id) => Expr::Primary(PrimaryExpr::Path(ExprPath {
                    span: Span::default(),
                    attrs: Vec::new(),
                    qself: None,
                    path: id.clone().into(),
                })),
                Member::Unnamed(_) => {
                    return Err(moxy_token::LexError::new(stream.span())
                        .message("tuple index needs a value")
                        .into());
                }
            };
            Ok(Self {
                span: Span::default(),
                attrs,
                member,
                colon_punct: None,
                expr,
                shorthand: true,
            })
        }
    }
}

impl ToTokens for FieldValue {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }

        if self.shorthand {
            self.member.to_tokens(t);
        } else {
            self.member.to_tokens(t);
            self.colon_punct.to_tokens(t);
            self.expr.to_tokens(t);
        }
    }
}
