use moxy_token::{Span, Spanner, ToTokens, TokenStream};

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

impl Parse for StmtLocal {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor)
            .map(|cursor| <Token![let]>::peek(cursor) && Pattern::peek(cursor.offset(1)))
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let let_keyword = <_ as Parse>::parse(parser)?;
        let pat = <_ as Parse>::parse(parser)?;
        let ty = if <Token![:]>::peek(parser.cursor()) {
            let colon = <_ as Parse>::parse(parser)?;
            Some((colon, <_ as Parse>::parse(parser)?))
        } else {
            None
        };

        let init = <_ as Parse>::parse(parser)?;
        let semi = <_ as Parse>::parse(parser)?;

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
        cursor = Pattern::skip(cursor)?;

        if <Token![:]>::peek(cursor) {
            cursor = <Token![:]>::skip(cursor)?;
            cursor = Type::skip(cursor)?;
        }

        cursor = Option::<StmtLocalInit>::skip(cursor)?;
        Option::<Token![;]>::skip(cursor)
    }
}

impl Spanner for StmtLocal {
    fn span(&self) -> Span {
        let end = if let Some(s) = &self.semi {
            s.span()
        } else if let Some(init) = &self.init {
            init.span()
        } else {
            self.pat.span()
        };

        self.attrs.span().join(end)
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

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            eq: <_ as Parse>::parse(parser)?,
            expr: <_ as Parse>::parse(parser)?,
            diverge: if <Token![else]>::peek(parser.cursor()) {
                Some((<_ as Parse>::parse(parser)?, Box::new(<_ as Parse>::parse(parser)?)))
            } else {
                None
            },
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = <Token![=]>::skip(cursor)?;
        cursor = Expr::skip(cursor)?;

        if <Token![else]>::peek(cursor) {
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
