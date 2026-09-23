use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A struct literal field (`member: expr` or shorthand `member`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
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
        Attributes::skip(cursor).unwrap_or(cursor).peek::<Member>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let member = parser.parse()?;

        if parser.peek::<Token![:]>() {
            Ok(Self {
                attrs,
                member,
                colon_punct: parser.parse()?,
                expr: parser.parse()?,
            })
        } else {
            let expr = match &member {
                Member::Named(id) => expr::ExprPath {
                    attrs: Attributes::default(),
                    qself: None,
                    path: id.clone().into(),
                }
                .into(),
                Member::Unnamed(_) => {
                    return parser.error("tuple index needs a value").into();
                }
            };

            Ok(Self {
                attrs,
                member,
                colon_punct: None,
                expr,
            })
        }
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        let shorthand = cursor.peek::<Ident>();
        cursor = cursor.skip::<Member>()?;

        if cursor.peek::<Token![:]>() {
            cursor.skip::<Token![:]>()?.skip::<Expr>()
        } else if shorthand {
            Some(cursor)
        } else {
            None
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
        self.member.to_tokens(t);
        self.colon_punct.to_tokens(t);

        if !self.is_shorthand() {
            self.expr.to_tokens(t);
        }
    }
}
