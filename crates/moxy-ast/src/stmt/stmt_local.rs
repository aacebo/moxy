use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A `let` binding statement.
#[derive(Debug, Clone, PartialEq, Eq)]
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
            .map(|cursor| cursor.peek::<Token![let]>() && cursor.offset(1).peek::<Pattern>())
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let let_keyword = parser.parse()?;
        let pat = parser.parse()?;
        let ty = if parser.peek::<Token![:]>() {
            let colon = parser.parse()?;
            Some((colon, parser.parse()?))
        } else {
            None
        };

        let init = parser.parse()?;
        let semi = parser.parse()?;

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
        cursor = cursor.skip::<Token![let]>()?;
        cursor = cursor.skip::<Pattern>()?;

        if cursor.peek::<Token![:]>() {
            cursor = cursor.skip::<Token![:]>()?;
            cursor = cursor.skip::<Type>()?;
        }

        cursor = cursor.skip::<Option<StmtLocalInit>>()?;
        cursor.skip::<Option<Token![;]>>()
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
        cursor.peek::<Token![=]>() && cursor.offset(1).peek::<Expr>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            eq: parser.parse()?,
            expr: parser.parse()?,
            diverge: if parser.peek::<Token![else]>() {
                Some((parser.parse()?, Box::new(parser.parse()?)))
            } else {
                None
            },
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = cursor.skip::<Token![=]>()?;
        cursor = cursor.skip::<Expr>()?;

        if cursor.peek::<Token![else]>() {
            cursor = cursor.skip::<Token![else]>()?;
            cursor = cursor.skip::<Expr>()?;
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
