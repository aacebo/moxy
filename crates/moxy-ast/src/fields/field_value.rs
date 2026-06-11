use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::Colon;
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::expr::{ExprPath, PrimaryExpr};
use crate::{Attributes, Expr, Member};

/// A struct literal field (`member: expr` or shorthand `member`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldValue {
    pub attrs: Attributes,
    pub member: Member,
    pub colon_punct: Option<Colon>,
    pub expr: Expr,
    pub shorthand: bool,
}

impl Parse for FieldValue {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let member = stream.parse::<Member>()?;
        if stream.peek::<Colon>() {
            let colon_punct = Some(stream.parse::<Colon>()?);
            let expr = stream.parse::<Expr>()?;
            Ok(Self {
                attrs,
                member,
                colon_punct,
                expr,
                shorthand: false,
            })
        } else {
            let expr = match &member {
                Member::Named(id) => Expr::Primary(PrimaryExpr::Path(ExprPath {
                    attrs: Attributes::default(),
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
                attrs,
                member,
                colon_punct: None,
                expr,
                shorthand: true,
            })
        }
    }
}

impl Spanner for FieldValue {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.member.span()
        };
        start.join(self.expr.span())
    }
}

impl ToTokens for FieldValue {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);

        if self.shorthand {
            self.member.to_tokens(t);
        } else {
            self.member.to_tokens(t);
            self.colon_punct.to_tokens(t);
            self.expr.to_tokens(t);
        }
    }
}
