use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// A `let` binding statement.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtLocal {
    pub attrs: Attributes,
    pub let_keyword: Token![let],
    pub pat: Pattern,
    pub ty: Option<(Token![:], Type)>,
    pub init: Option<StmtLocalInit>,
    pub semi: Option<Token![;]>,
}

impl Attributed for StmtLocal {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for StmtLocal {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor)
            .map(|cursor| <Token![let]>::peek(cursor) && Pattern::peek(cursor.offset(1)))
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let let_keyword = <_ as Parse>::parse(parser)?;
        // As in rustc, the pattern has no top-level `|`
        let pat = crate::pat::parse::single(parser)?;
        let ty = if <Token![:]>::peek(parser.cursor()) {
            let colon = <_ as Parse>::parse(parser)?;
            Some((colon, <_ as Parse>::parse(parser)?))
        } else {
            None
        };

        let init: Option<StmtLocalInit> = <_ as Parse>::parse(parser)?;

        // A `let` statement always ends with `;`, even last in a block
        let semi = Some(<Token![;]>::parse(parser)?);

        Ok(Self {
            attrs,
            let_keyword,
            pat,
            ty,
            init,
            semi,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = <Token![let]>::skip(cursor)?;
        cursor = crate::pat::skip::single(cursor)?;

        if <Token![:]>::peek(cursor) {
            cursor = <Token![:]>::skip(cursor)?;
            cursor = Type::skip(cursor)?;
        }

        cursor = Option::<StmtLocalInit>::skip(cursor)?;
        <Token![;]>::skip(cursor)
    }
}

impl Spanner for StmtLocal {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for StmtLocal {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.let_keyword.to_tokens(t);
        self.pat.to_tokens(t);

        if let Some((colon, ty)) = &self.ty {
            colon.to_tokens(t);
            ty.to_tokens(t);
        }

        self.init.to_tokens(t);
        self.semi.to_tokens(t);
    }
}

/// The initializer of a `let` binding.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtLocalInit {
    pub eq: Token![=],
    pub expr: Expr,
    pub diverge: Option<(Token![else], Box<Expr>)>,
}

impl Spanner for StmtLocalInit {
    fn span(&self) -> Span {
        self.eq.span().join(self.expr.span())
    }
}

impl Parse for StmtLocalInit {
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![=]>::peek(cursor) && Expr::peek(cursor.offset(1))
    }

    // An initializer, whose `let` holds a pattern and a type as well, takes
    // another level of stack, as in `{ let a = { let a = .. }; }`
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let _weight = moxy_token::__private::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;
        let eq = <_ as Parse>::parse(parser)?;
        let expr = <_ as Parse>::parse(parser)?;
        let diverge = if <Token![else]>::peek(parser.cursor()) {
            if after_brace(parser.cursor()) {
                return parser.error("an initializer that ends with `}` cannot take `else`").into();
            }

            Some((<_ as Parse>::parse(parser)?, Box::new(<_ as Parse>::parse(parser)?)))
        } else {
            None
        };

        Ok(Self { eq, expr, diverge })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let _weight = moxy_token::__private::Depth::enter()?;
        cursor = <Token![=]>::skip(cursor)?;
        cursor = Expr::skip(cursor)?;

        if <Token![else]>::peek(cursor) {
            if after_brace(cursor) {
                return None;
            }

            cursor = <Token![else]>::skip(cursor)?;
            cursor = Expr::skip(cursor)?;
        }

        Some(cursor)
    }
}

impl ToTokens for StmtLocalInit {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.eq.to_tokens(tokens);
        self.expr.to_tokens(tokens);

        if let Some((keyword, expr)) = &self.diverge {
            keyword.to_tokens(tokens);
            expr.to_tokens(tokens);
        }
    }
}

// Whether the initializer before `else` ends with the `}` of a block, a
// struct literal, or a brace macro, which rustc rejects as `else` would read
// as part of it
fn after_brace(cursor: Cursor<'_>) -> bool {
    matches!(cursor.prev(), Some(moxy_token::TokenTree::Group(group)) if group.delim == moxy_token::Delim::Brace)
}
