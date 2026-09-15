use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A struct literal field (`member: expr` or shorthand `member`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FieldValue {
    pub attrs: Attributes,
    pub member: Member,
    pub colon_punct: Option<Token![:]>,
    pub expr: Expr,
}

impl FieldValue {
    pub fn is_shorthand(&self) -> bool {
        self.colon_punct.is_none()
    }
}

impl Parse for FieldValue {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Member>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let member = parser.parse::<Member>()?;

        if parser.peek::<Token![:]>() {
            Ok(Self {
                attrs: Default::default(),
                member,
                colon_punct: parser.parse()?,
                expr: parser.parse()?,
            })
        } else {
            Ok(Self {
                attrs: Default::default(),
                member,
                colon_punct: None,
                expr: match &member {
                    Member::Named(id) => expr::ExprPath {
                        attrs: Attributes::default(),
                        qself: None,
                        path: id.clone().into(),
                    }
                    .into(),
                    Member::Unnamed(_) => {
                        return parser.error("tuple index needs a value").into();
                    }
                },
            })
        }
    }
}

impl Spanner for FieldValue {
    fn span(&self) -> Span {
        self.attrs.span().join(self.expr.span())
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
